use std::fs::File;
use std::io::BufReader;

use crfsuite::{Algorithm, GraphicalModel, Trainer};
use xml::reader::{EventReader, XmlEvent};

use crate::{get_address_features, tokenize};

pub fn train_model() -> std::io::Result<()> {
    let file = File::open("training/labeled.xml")?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut trainer = Trainer::new(false);

    let mut address: Vec<String> = Vec::new();
    let mut yseq: Vec<String> = Vec::new(); // make Vec<AddressComponent>
                                            // let mut yseq: &[String];

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
                    println!("Address: {:?}", address);
                    let tokens = tokenize(&address.join(" "));
                    let xseq = get_address_features(&tokens);
                    println!("Tags: {:?}", yseq);

                    match trainer.append(&xseq, &yseq, 0) {
                        Ok(()) => println!("Appended data"),
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

    match trainer.select(Algorithm::LBFGS, GraphicalModel::CRF1D) {
        Ok(()) => println!("Selected algorithm"),
        Err(e) => println!("Error selecting algorithm: {}", e),
    }

    match trainer.train("usaddr.crfsuite", -1) {
        Ok(()) => println!("Trained model"),
        Err(e) => println!("Error training model: {}", e),
    }

    Ok(())
}
