use json::JsonValue;
use railroad::*;
use std::fs;
use std::fs::File;
use std::io::Write;

//STRING, PATTERN, PREC, PREC_RIGHT, PREC_LEFT, BLANK
//CHOICE, SEQ, SYMBOL, REPEAT, REPEAT1, FIELD,
//
fn rule_to_node(rule: &JsonValue) -> Box<dyn RailroadNode> {
    if rule["type"] == "SYMBOL" {
        return Box::new(NonTerminal::new(rule["name"].to_string()));
    } else if rule["type"] == "FIELD" {
        return rule_to_node(&rule["content"]);
    } else if rule["type"] == "SEQ" {
        let mut seq = Sequence::default();
        for child in rule["members"].members() {
            seq.push(Box::new(rule_to_node(child)));
        }
        return Box::new(seq);
    } else if rule["type"] == "REPEAT1" {
        let inner_node = rule_to_node(&rule["content"]);
        let repeat_node = rule_to_node(&rule["content"]);
        return Box::new(Repeat::new(inner_node, repeat_node));
    } else if rule["type"] == "REPEAT" {
        let inner_node = Empty;
        let repeat_node = rule_to_node(&rule["content"]);
        return Box::new(Repeat::new(inner_node, repeat_node));
    } else if rule["type"] == "CHOICE" {
        let mut children = Choice::default();
        for child in rule["members"].members() {
            children.push(rule_to_node(child));
        }
        return Box::new(children);
    } else if rule["type"] == "PREC" || rule["type"] == "PREC_RIGHT" || rule["type"] == "PREC_LEFT"
    {
        return rule_to_node(&rule["content"]);
    } else if rule["type"] == "BLANK" {
        return Box::new(Empty);
    } else if rule["type"] == "STRING" {
        return Box::new(Terminal::new(rule["value"].to_string()));
    } else if rule["type"] == "PATTERN" {
        return Box::new(Terminal::new(rule["value"].to_string()));
    }
    Box::new(Empty)
}

fn main() {
    let syntax_json_path = "tree-sitter-cobol/src/grammar.json";
    let json = fs::read_to_string(syntax_json_path).expect("Something went wrong reading the file");

    let parsed = json::parse(&json).unwrap();
    let rules = &parsed["rules"];
    println!("{} rules", rules.len());

    let mut count = 0;
    for (key, rule) in rules.entries() {
        let seq = rule_to_node(rule);
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
