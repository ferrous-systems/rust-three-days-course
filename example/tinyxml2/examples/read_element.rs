use tinyxml2;
use tinyxml2::tinyxml2_XMLDocument as XMLDocument;
use std::ffi::CString;
use std::ffi::CStr;

fn main() {
    unsafe {
        let mut doc = XMLDocument::new(false, tinyxml2::tinyxml2_Whitespace_PRESERVE_WHITESPACE);
        let xml = CString::new("<?xml version=\"1.0\" encoding=\"utf-8\" ?>\n<test>foo</test>").unwrap();
        let xml_ptr = xml.as_ptr();
        println!("{:?}", xml_ptr);
        let success = doc.Parse(xml_ptr, 60);

        println!("success: {}", success);
        let node = doc._base;
        let node_name = CString::new("test").unwrap();
        let child = node.FirstChildElement(node_name.as_ptr());
        
        //let value = (*child).GetText();

        //let value = CStr::from_ptr(node.Value());
        println!("{:?}", child);

        //let node = doc._base;
        //println!("{:?}", node);
    }
}