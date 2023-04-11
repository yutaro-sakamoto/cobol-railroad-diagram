#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

use json::JsonValue;
use railroad::*;
use std::fs;
use std::fs::File;
use std::io::Write;

fn rule_to_sequence(rule: &JsonValue) -> Sequence {
    let mut seq = Sequence::default();
    if rule["type"] == "SYMBOL" {
        seq.push(Box::new(NonTerminal::new(rule["name"].to_string())));
    }
    seq
}

fn main() {
    let syntax_json_path = "tree-sitter-cobol/src/grammar.json";
    let json = fs::read_to_string(syntax_json_path).expect("Something went wrong reading the file");

    let parsed = json::parse(&json).unwrap();
    let rules = &parsed["rules"];
    println!("{} rules", rules.len());

    let mut count = 0;
    for (key, rule) in rules.entries() {
        let seq = rule_to_sequence(rule);
        let mut dia = Diagram::new(seq);
        dia.add_element(
            svg::Element::new("style")
                .set("type", "text/css")
                .raw_text(DEFAULT_CSS),
        );

        let mut f = File::create(format!("out/{:05}_{}.svg", count, key)).unwrap();
        f.write_all(dia.to_string().as_bytes()).unwrap();
        count += 1;
    }
}
