extern crate serde_json;
extern crate pandoc_types;
extern crate glob;

use glob::glob;
use pandoc_types::definition::*;

use std::io::Write;
use std::fs::{File, create_dir};
use std::process::{Command, Stdio};

fn change_code(filename: &str, locale: &str, extract: bool) {
    let filepath = format!("presentation/chapters/{}/{}.chapter", locale, filename);

    if extract {
        let codedir = format!("presentation/chapters/shared/code/{}", filename);
        create_dir(codedir).expect("Could not create code directory");
    }

    let parser = Command::new("/usr/local/bin/pandoc")
                         .arg(&filepath)
                         .arg("-w")
                         .arg("json")
                         .stdout(Stdio::piped())
                         .spawn()
                         .expect("failed to execute pandoc");

    let mut doc: Pandoc = serde_json::from_reader (parser.stdout.unwrap()).expect("unable to read JSON input");

    {
        let blocks = &mut doc.1;

        let mut blocknumber = 1;
        for block in blocks {
            let mut new_block = Block::Null;
            if let &mut Block::CodeBlock(ref attributes, ref content) = block {
                let code_lang = match attributes.1.first() {
                    Some(lang) => lang,
                    None => "output"
                };

                let extension = match code_lang {
                    "rust" => "rs",
                    _ => code_lang
                };

                let codepath = format!("presentation/chapters/shared/code/{}/{}.{}", filename, blocknumber, extension);
                new_block = Block::RawBlock(Format("html".into()),
                    format!("<pre><code data-source=\"{}\" data-trim=\"hljs {}\"></code></pre>", codepath, code_lang));

                if extract {
                    let mut f = File::create(&codepath).expect("File creation should succeed");
                    write!(f, "{}", content).expect("Could not write to file");
                }

                blocknumber += 1;
            }

            if new_block != Block::Null {
                std::mem::replace(block, new_block);
            }
        }
    }

    let cmd = Command::new("/usr/local/bin/pandoc")
                         .arg("-f")
                         .arg("json")
                         .arg("-w")
                         .arg("markdown_github")
                         .arg("--atx-headers")
                         .arg("-o")
                         .arg(&filepath)
                         .stdin(Stdio::piped())
                         .spawn()
                         .expect("failed to execute process");

    write!(cmd.stdin.expect("expected stdin to be present"), "{}", serde_json::to_string(&doc).expect("expected serialisation to work"))
        .expect("expected to be able to write to command stdin");
}

fn main() {
    for locale in ["de-DE", "en-US", "es-ES"].iter() {
        for entry in glob(&format!("presentation/chapters/{}/*.chapter", locale)).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    let name = path.file_stem().expect("filestem to exist");
                    change_code(&name.to_string_lossy(), locale, false);
                },
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
