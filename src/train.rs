use std::fs::File;
use std::io::BufReader;

use crfsuite::{Algorithm, GraphicalModel, Trainer};
use xml::reader::{EventReader, XmlEvent};

use crate::{get_address_features, tokenize};

pub fn train_model(export_path: &str) -> std::io::Result<()> {
    let file = File::open("training/labeled.xml")?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut trainer = Trainer::new(false);

    match trainer.select(Algorithm::LBFGS, GraphicalModel::CRF1D) {
        Ok(()) => (),
        Err(e) => println!("Error selecting algorithm: {}", e),
    }

    let mut address: Vec<String> = Vec::new();
    let mut yseq: Vec<String> = Vec::new();

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if name.local_name == "AddressString" {
                    address.clear();
                    yseq.clear();
                } else {
                    yseq.push(name.local_name.to_string());
                }
            }
            Ok(XmlEvent::Characters(s)) => {
                address.push(s);
            }
            Ok(XmlEvent::EndElement { name }) => {
                if name.local_name == "AddressString" {
                    let tokens = tokenize(&address.join(" "));
                    let xseq = get_address_features(&tokens);
                    assert_eq!(xseq.len(), yseq.len());

                    match trainer.append(&xseq, &yseq, 0) {
                        Ok(()) => (),
                        Err(e) => {
                            eprintln!("Error appending data: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {}
        }
    }

    match trainer.train(export_path, -1) {
        Ok(()) => (),
        Err(e) => println!("Error training model: {}", e),
    }

    Ok(())
}
