use std::fs::File;
use std::io::BufReader;

use crfsuite::{Algorithm, GraphicalModel, Item, Trainer};
use xml::reader::{EventReader, XmlEvent};

pub fn train_model() -> std::io::Result<()> {
    let file = File::open("training/labeled.xml")?;
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;

    let xseq: &[Item];
    let yseq: &[String];

    let mut trainer = Trainer::new(false);

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                println!("{:spaces$}+{name}", "", spaces = depth * 2);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{:spaces$}-{name}", "", spaces = depth * 2);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
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
