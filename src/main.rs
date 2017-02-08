extern crate csv;
extern crate itertools;

use std::{env, fmt};
use std::collections::HashMap;

use itertools::Itertools;

type Error = Box<std::error::Error>;

#[derive(Clone, Debug)]
struct Element {
    number: u16,
    symbol: String,
    name: String
}

impl Element { 
    fn new (number: u16, symbol: &str, name: &str) -> Element {
        Element { 
            number: number,
            symbol: symbol.to_string(),
            name: name.to_string()
        }
    }
}

struct Elements {
    elem_map: HashMap<String, Element>
}

impl Elements {
    fn read_from_csv(path: &str) -> Result<Self, Error> {
        let mut csv_reader = csv::Reader::from_file(path).unwrap();
        let elem_map: Result<_, Error> = csv_reader.decode().map(|record| {
            let (number, symbol, name): (u16, String, String) = record?;
            let symbol = symbol.trim();
            let name = name.trim();
            Ok((symbol.to_lowercase().to_string(), Element::new(number, symbol, name)))
        }).collect();

        Ok(Elements { elem_map: elem_map? })
    }

    // Try to spell target_word using elements from elem_map
    fn spell_word(&self, target_word: &str) -> Option<Spelling> {
        let mut result = Vec::new();
        let target_word = target_word.to_lowercase().to_string();
        let mut target_word = target_word.as_str();

        while !target_word.is_empty() {
            // First, try matching the next two letters
            if target_word.len() >= 2 {
                let (head, tail) = target_word.split_at(2);

                if let Some(elem) = self.elem_map.get(head) {
                    result.push(elem);
                    target_word = tail;
                    continue
                }
            }

            // Try matching one letter if we could not match two
            let (head, tail) = target_word.split_at(1);

            if let Some(elem) = self.elem_map.get(head) {
                result.push(elem);
                target_word = tail;
                continue;
            }

            // If no matching one-letter element could be found,
            // this word cannot be spelled
            return None;
        }

        Some(Spelling(result))
    }
}

struct Spelling<'a>(Vec<&'a Element>);

impl<'a> fmt::Display for Spelling<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for elem in &self.0 {
            write!(f, "{}", elem.symbol)?;
        }

        write!(f, " (")?;

        for elem in self.0.iter().map(|e| e.name.as_str()).intersperse(", ") {
            write!(f, "{}", elem)?;
        }

        write!(f, ")")
    }
}

fn main() {
    let elements = Elements::read_from_csv("./elements.csv").expect("Unable to read elements from file");

    for target_word in env::args().skip(1) {
        match elements.spell_word(&target_word) {
            Some(spelling) => println!("{}", spelling),
            None => println!(r#"Could not find a match for word "{}"."#, target_word)
        }
    }
}
