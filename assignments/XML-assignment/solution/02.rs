use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
             .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}
struct Adress {
    name: String,
    street: String,
    city: String,
    state: String,
    zip: u8,
    country: String,
}
struct Item {
    part_number: String,
    product_name: String,
    quantity: u8,
    price: f32,
    comment: String,
}
struct PurchaseOrder {
    purchase_order_number: u32,
    order_date: String,
    adress: Adress,
    delivery_notes: String,
    items: Vec<Item>
}

fn main() {
    let file = File::open("src/file2.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, ..}) => {
                if attributes.len() > 0 {
                    println!("{}\n{}: {}", name, attributes[0].name.local_name, attributes[0].value  );
                    // depth += 1;
                } else {
                    print!("{}: ", name );
                    // depth += 1;
                }
            }

            Ok(XmlEvent::Characters(content)) => {
                if content.len() == 0 {
                    print!("0");
                } else {
                print!("{}{} \n", indent(depth), content);
                }
            }

            Ok(XmlEvent::EndElement { name }) => {
                // depth -= 1;
                // println!("{}-{}", indent(depth), name);
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}
