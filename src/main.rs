extern crate shina;
use std::{collections::HashMap, fs};

use shina::helper::dsl::Shina;

fn main() {

    let rule = fs::read_to_string("tests/test-01.yml").unwrap();
    let parsed_rule = Shina::import_from_yaml(&rule).unwrap();

    match fs::read_to_string("utils/ascii.art") {
        Ok(content) => println!("{}", content),
        Err(e) => eprintln!("Error reading file: {}", e),
    }
    println!("->> Shina - Rule Engine");
    println!("-----------------------------------");
    for rule in &parsed_rule {
        println!("Parsed Rule: {:?}", rule.name);
        println!("Parsed Rule: {:?}", rule.conditions_parsed);
        println!("Rule weight: {}", Shina::weight(&rule.conditions_parsed));
    
        let r = Shina::eval(
            &parsed_rule[0].conditions_parsed, 
            &"2025-05-01 - 200 GET [704e9ad3-78f1-44ff-a4ef-5ae0fcade0ef] /admin".to_string(),
            &HashMap::from([
                ("date".to_string(), "2025-05-01".to_string()),
                ("user-agent".to_string(), "fox".to_string()),
                ("path".to_string(), "/admin".to_string()),
                ("method".to_string(), "GET".to_string()),
                ("status".to_string(), "200".to_string()),
                ("user".to_string(), "704e9ad3-78f1-44ff-a4ef-5ae0fcade0ef".to_string()),
            ]),
        );
    
        println!("Matched: {:?}", r);
        println!("-----------------------------------");
    }
}
