use std::collections::{HashMap, VecDeque};
use crate::helper::parse::parse_shina_from_yaml;

#[derive(Debug, Clone)]
pub enum Condition {
    Equals(String, String),              // key == "value"
    NotEquals(String, String),           // key != "value"

    Superior(String, String),            // key > "value"
    SuperiorEquals(String, String),      // key >= "value"
    Inferior(String, String),            // key < "value"
    InferiorEquals(String, String),      // key <= "value"

    Contains(String, String),            // key CONTAINS "value"
    NotContains(String, String),         // key NOTCONTAINS "value"

    ContainsRaw(String),                 // raw RCONTAINS "value"
    NotContainsRaw(String),              // raw NOTCONTAINS "value"


    And(Box<Condition>, Box<Condition>), // condition && condition
    Or(Box<Condition>, Box<Condition>),  // condition || condition
}


//                       
//   _____ _   _         
//  |   __| |_|_|___ ___ 
//  |__   |   | |   | .'|
//  |_____|_|_|_|_|_|__,|
//                  
#[derive(Debug, Clone)]     
pub struct Shina {
    pub name: String,
    pub description: String,
    pub metadata: HashMap<String, String>,
    pub severity: String,
    pub tags: Vec<String>,
    pub conditions_raw: String,
    pub conditions_parsed: Condition
}

impl Shina {
    /**
     * Create a new Shina instance.
     * 
     * @param name: The name of the Shina.
     * @param description: The description of the Shina.
     * @param metadata: The metadata of the Shina.
     * @param conditions_raw: The raw conditions string.
     *
     * @return: A new Shina instance.
     */
    pub fn new(name: &str, description: &str, metadata: HashMap<String, String>, conditions_raw: String, severity: &str, tags: Vec<String>) -> Shina {
        let conditions_parsed = Shina::tokenize(&conditions_raw);
        Shina {
            name: name.to_string(),
            description: description.to_string(),
            metadata,
            severity: severity.to_string(),
            tags,
            conditions_raw,
            conditions_parsed: conditions_parsed.unwrap_or_else(|err| {
                println!("Error parsing conditions: {}", err);
                Condition::Equals("error".to_string(), "error".to_string())
            }),
        }
    } 


    /**
     * Detect if the rules is matched
     * 
     * @param rule: The rule to match.
     * @param raw: The raw string to match against.
     * @param parsed: The parsed values to match against in the HashMap.
     * 
     * @return: A boolean indicating if the rule is matched.
     */
    pub fn detect(rule:String, raw: String, parsed: HashMap<String, String>) -> bool {
        let tokens = Shina::tokenize(&rule).unwrap();
        Shina::eval(
            &tokens, 
            &raw,
            &parsed,
        )
    }

    /**
     * Tokenize the input string into a Condition.
     * 
     * @param input: The input string to tokenize.
     * @return: A Result containing the Condition or an error message.
     */
    pub fn tokenize(input: &str) -> Result<Condition, String> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
    
        while let Some(&c) = chars.peek() {
            match c {
                ' ' => {
                    chars.next();
                }
                '(' | ')' => {
                    tokens.push(c.to_string());
                    chars.next();
                }
                '"' => {
                    chars.next();
                    let mut value = String::new();
                    while let Some(ch) = chars.next() {
                        if ch == '"' {
                            break;
                        }
                        value.push(ch);
                    }
                    tokens.push(format!("\"{}\"", value));
                }
                '=' => {
                    chars.next();
                    if chars.next() == Some('=') {
                        tokens.push("==".to_string());
                    } else {
                        return Err("Unexpected '='".to_string());
                    }
                }
                '!' => {
                    chars.next();
                    if chars.next() == Some('=') {
                        tokens.push("!=".to_string());
                    } else {
                        return Err("Expected '!='".to_string());
                    }
                }
                '>' => {
                    chars.next();
                    if chars.next() == Some('=') {
                        tokens.push(">=".to_string());
                    } else {
                        tokens.push(">>".to_string());
                    }
                }
                '<' => {
                    chars.next();
                    if chars.next() == Some('=') {
                        tokens.push("<=".to_string());
                    } else {
                        tokens.push("<<".to_string());
                    }
                }

                '&' => {
                    chars.next();
                    if chars.next() == Some('&') {
                        tokens.push("&&".to_string());
                    } else {
                        return Err("Expected '&&'".to_string());
                    }
                }
                '|' => {
                    chars.next();
                    if chars.next() == Some('|') {
                        tokens.push("||".to_string());
                    } else {
                        return Err("Expected '||'".to_string());
                    }
                }
                _ => {
                    let mut word = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                            word.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(word);
                }
            }
        }
    
        let mut queue = VecDeque::from(tokens);
        // parse or and and
        let condition = parse_or(&mut queue)?;
        return Ok(condition);
    }

    /**
     * Evaluate the condition against the raw string and parsed values.
     * 
     * @param cond: The condition to evaluate.
     * @param raw: The raw string to evaluate against.
     * @param parsed: The parsed values to evaluate against in the HashMap.
     */
    pub fn eval(cond: &Condition, raw: &str, parsed: &HashMap<String, String>) -> bool {
        match cond {
            // Equality checks
            Condition::Equals(k, v) => parsed.get(k).map_or(false, |val| val == v),
            Condition::NotEquals(k, v) => parsed.get(k).map_or(false, |val| val == v),
    
            // Comparison checks
            Condition::Superior(k, v) => parsed.get(k).map_or(false, |val| val > v),
            Condition::SuperiorEquals(k, v) => parsed.get(k).map_or(false, |val| val >= v),
            Condition::Inferior(k, v) => parsed.get(k).map_or(false, |val| val < v),
            Condition::InferiorEquals(k, v) => parsed.get(k).map_or(false, |val| val <= v),
    
            // String checks
            Condition::Contains(k, v) => parsed.get(k).map_or(false, |val| val.contains(v)),
            Condition::NotContains(k, v) => parsed.get(k).map_or(false, |val| !val.contains(v)),
    
            // Raw string checks
            Condition::ContainsRaw(v) => raw.contains(v),
            Condition::NotContainsRaw(v) => !raw.contains(v),
    
            // Logical operations
            Condition::And(a, b) => Shina::eval(a, raw, parsed) && Shina::eval(b, raw, parsed),
            Condition::Or(a, b) => Shina::eval(a, raw, parsed) || Shina::eval(b, raw, parsed),
        }
    }


    /**
     * Calculate the weight of a tokenized rules
     */
    pub fn weight(c: &Condition) -> usize {
        let mut weight = 0;
        match c {
            Condition::Equals(_, _) => weight += 1,
            Condition::NotEquals(_, _) => weight += 1,
            Condition::Superior(_, _) => weight += 2,
            Condition::SuperiorEquals(_, _) => weight += 2,
            Condition::Inferior(_, _) => weight += 2,
            Condition::InferiorEquals(_, _) => weight += 2,
            Condition::Contains(_, _) => weight += 3,
            Condition::NotContains(_, _) => weight += 3,
            Condition::ContainsRaw(_) => weight += 2,
            Condition::NotContainsRaw(_) => weight += 2,
            Condition::And(a, b) => {
                weight += Shina::weight(a);
                weight += Shina::weight(b);
                weight += 3; // Logical AND
            }
            Condition::Or(a, b) => {
                weight += Shina::weight(a);
                weight += Shina::weight(b);
                weight += 2; // Logical OR
            }
        }
        weight
    }


    /**
     * Import shina from yaml file
     */
    pub fn import_from_yaml(yaml: &str) -> Result<Vec<Shina>, Box<dyn std::error::Error>> {
        parse_shina_from_yaml(yaml)
    }
}

//                                                          
//   _____                ___             _   _             
//  |     |___ ___ ___   |  _|_ _ ___ ___| |_|_|___ ___ ___ 
//  |   --| . |  _| -_|  |  _| | |   |  _|  _| | . |   |_ -|
//  |_____|___|_| |___|  |_| |___|_|_|___|_| |_|___|_|_|___|
//                                                          
fn parse_or(tokens: &mut VecDeque<String>) -> Result<Condition, String> {
    let mut left = parse_and(tokens)?;
    while let Some(op) = tokens.front() {
        if op == "||" {
            tokens.pop_front();
            let right = parse_and(tokens)?;
            left = Condition::Or(Box::new(left), Box::new(right));
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_and(tokens: &mut VecDeque<String>) -> Result<Condition, String> {
    let mut left = parse_primary(tokens)?;
    while let Some(op) = tokens.front() {
        if op == "&&" {
            tokens.pop_front();
            let right = parse_primary(tokens)?;
            left = Condition::And(Box::new(left), Box::new(right));
        } else {
            break;
        }
    }
    Ok(left)
}

fn parse_primary(tokens: &mut VecDeque<String>) -> Result<Condition, String> {
    if let Some(token) = tokens.pop_front() {
        if token == "(" {
            let expr = parse_or(tokens)?;
            if tokens.pop_front() != Some(")".to_string()) {
                return Err("Expected ')'".to_string());
            }
            return Ok(expr);
        }

        let left = token;
        if let Some(op) = tokens.pop_front() {
            match op.as_str() {
                "==" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::Equals(left, val.trim_matches('"').to_string()));
                    }
                }
                "!=" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::NotEquals(left, val.trim_matches('"').to_string()));
                    }
                }
                ">>" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::Superior(left, val.trim_matches('"').to_string()));
                    }
                }
                ">=" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::SuperiorEquals(left, val.trim_matches('"').to_string()));
                    }
                }
                "<<" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::Inferior(left, val.trim_matches('"').to_string()));
                    }
                }
                "<=" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::InferiorEquals(left, val.trim_matches('"').to_string()));
                    }
                }

                "CONTAINS" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::Contains(left, val.trim_matches('"').to_string()));
                    }
                }

                "NOTCONTAINS" => {
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::NotContains(left, val.trim_matches('"').to_string()));
                    }
                }

                "RCONTAINS" => {
                    if left != "raw" {
                        return Err("RCONTAINS only applies to 'raw'".to_string());
                    }
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::ContainsRaw(val.trim_matches('"').to_string()));
                    }
                }
                "RNOTCONTAINS" => {
                    if left != "raw" {
                        return Err("NOTRCONTAINS only applies to 'raw'".to_string());
                    }
                    if let Some(val) = tokens.pop_front() {
                        return Ok(Condition::NotContainsRaw(val.trim_matches('"').to_string()));
                    }
                }
                _ => {}
            }
        }
    }
    Err("Invalid expression".to_string())
}
