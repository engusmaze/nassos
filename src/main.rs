use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

#[derive(Clone)]
enum Instruction {
    Define(String),
    Set(String, bool),
    CopyTo(String, String),
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
    Not(String, String),
    Print(String),
    Ascii(String, String, String, String, String, String, String),
    NewLine,
    GoTo(String),
    GoToIf(String, String),
}

fn main() {
    let mut instructions = Vec::<Instruction>::new();
    let mut labels = HashMap::<String, usize>::new();
    if env::args().len() != 2 {
        eprintln!("Invalid arguments");
        exit(1);
    }
    let filename = env::args().collect::<Vec<String>>()[1].clone();
    let filename = filename.as_str();
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            for (index, line) in content.lines().into_iter().enumerate() {
                let keywords = line.split("#").collect::<Vec<&str>>()[0]
                    .split_whitespace()
                    .collect::<Vec<&str>>();

                let error_line = || {
                    eprintln!("Syntax error at line {}: {}", index + 1, line);
                    exit(2);
                };

                if keywords.len() > 0 {
                    match keywords[0] {
                        "define" => {
                            if keywords.len() == 2 {
                                instructions.push(Instruction::Define(keywords[1].to_string()));
                            } else {
                                error_line();
                            }
                        }
                        "set" => {
                            if keywords.len() == 3 {
                                let value = match keywords[2] {
                                    "false" => false,
                                    "true" => true,
                                    _ => error_line(),
                                };
                                instructions.push(Instruction::Set(keywords[1].to_string(), value));
                            } else {
                                error_line();
                            }
                        }
                        "copy" => {
                            if keywords.len() == 4 && keywords[2] == "to" {
                                instructions.push(Instruction::CopyTo(
                                    keywords[1].to_string(),
                                    keywords[3].to_string(),
                                ));
                            } else {
                                error_line();
                            }
                        }
                        "and" => {
                            if keywords.len() == 4 {
                                instructions.push(Instruction::And(
                                    keywords[1].to_string(),
                                    keywords[2].to_string(),
                                    keywords[3].to_string(),
                                ));
                            } else {
                                error_line();
                            }
                        }
                        "or" => {
                            if keywords.len() == 4 {
                                instructions.push(Instruction::Or(
                                    keywords[1].to_string(),
                                    keywords[2].to_string(),
                                    keywords[3].to_string(),
                                ));
                            } else {
                                error_line();
                            }
                        }
                        "xor" => {
                            if keywords.len() == 4 {
                                instructions.push(Instruction::Xor(
                                    keywords[1].to_string(),
                                    keywords[2].to_string(),
                                    keywords[3].to_string(),
                                ));
                            } else {
                                error_line();
                            }
                        }
                        "not" => {
                            if keywords.len() == 3 {
                                instructions.push(Instruction::Not(
                                    keywords[1].to_string(),
                                    keywords[2].to_string(),
                                ));
                            } else {
                                error_line();
                            }
                        }
                        "print" => {
                            if keywords.len() == 2 {
                                instructions.push(Instruction::Print(keywords[1].to_string()));
                            } else {
                                error_line();
                            }
                        }
                        "ascii" => {
                            if keywords.len() == 8 {
                                instructions.push(Instruction::Ascii(
                                    keywords[1].to_string(),
                                    keywords[2].to_string(),
                                    keywords[3].to_string(),
                                    keywords[4].to_string(),
                                    keywords[5].to_string(),
                                    keywords[6].to_string(),
                                    keywords[7].to_string(),
                                ));
                            } else {
                                error_line();
                            }
                        }
                        "newline" => {
                            if keywords.len() == 1 {
                                instructions.push(Instruction::NewLine);
                            } else {
                                error_line();
                            }
                        }
                        "label" => {
                            if keywords.len() == 2 {
                                labels.insert(keywords[1].to_string(), instructions.len());
                            } else {
                                error_line();
                            }
                        }
                        "goto" => {
                            if keywords.len() == 2 {
                                instructions.push(Instruction::GoTo(keywords[1].to_string()));
                            } else if keywords.len() == 4 {
                                if keywords[2] == "if" {
                                    instructions.push(Instruction::GoToIf(
                                        keywords[1].to_string(),
                                        keywords[3].to_string(),
                                    ));
                                } else {
                                    error_line();
                                }
                            } else {
                                error_line();
                            }
                        }
                        _ => {
                            error_line();
                        }
                    }
                }
            }
        }
        Err(error) => {
            eprintln!("Error opening file {}: {}", filename, error);
            exit(1);
        }
    }
    let mut variables = HashMap::<String, bool>::new();
    let ensure_variable_exists = |variables: &HashMap<String, bool>, var: &String| {
        if !variables.contains_key(var) {
            eprintln!("Variable {} is not defined", var);
            exit(3);
        }
    };
    let mut i = 0;
    let end = instructions.len();
    while i < end {
        match instructions[i].clone() {
            Instruction::Define(var) => {
                if variables.contains_key(&var) {
                    eprintln!("Variable {} is already defined", var);
                    exit(4);
                }
                variables.insert(var, false);
            }
            Instruction::Set(var, value) => {
                ensure_variable_exists(&variables, &var);
                variables.insert(var, value);
            }
            Instruction::CopyTo(var1, var2) => {
                ensure_variable_exists(&variables, &var1);
                ensure_variable_exists(&variables, &var2);
                variables.insert(var2, variables[&var1]);
            }
            Instruction::And(var1, var2, var_out) => {
                ensure_variable_exists(&variables, &var1);
                ensure_variable_exists(&variables, &var2);
                ensure_variable_exists(&variables, &var_out);
                variables.insert(var_out, variables[&var1] && variables[&var2]);
            }
            Instruction::Or(var1, var2, var_out) => {
                ensure_variable_exists(&variables, &var1);
                ensure_variable_exists(&variables, &var2);
                ensure_variable_exists(&variables, &var_out);
                variables.insert(var_out, variables[&var1] || variables[&var2]);
            }
            Instruction::Xor(var1, var2, var_out) => {
                ensure_variable_exists(&variables, &var1);
                ensure_variable_exists(&variables, &var2);
                ensure_variable_exists(&variables, &var_out);
                variables.insert(var_out, variables[&var1] ^ variables[&var2]);
            }
            Instruction::Not(var, var_out) => {
                ensure_variable_exists(&variables, &var);
                ensure_variable_exists(&variables, &var_out);
                variables.insert(var_out, !variables[&var]);
            }
            Instruction::Print(var) => {
                ensure_variable_exists(&variables, &var);
                print!("{} ", variables[&var]);
            }
            Instruction::Ascii(var1, var2, var3, var4, var5, var6, var7) => {
                ensure_variable_exists(&variables, &var1);
                ensure_variable_exists(&variables, &var2);
                ensure_variable_exists(&variables, &var3);
                ensure_variable_exists(&variables, &var4);
                ensure_variable_exists(&variables, &var5);
                ensure_variable_exists(&variables, &var6);
                ensure_variable_exists(&variables, &var7);
                let mut byte = 0u8;
                byte = (byte << 1) | (variables[&var1] as u8);
                byte = (byte << 1) | (variables[&var2] as u8);
                byte = (byte << 1) | (variables[&var3] as u8);
                byte = (byte << 1) | (variables[&var4] as u8);
                byte = (byte << 1) | (variables[&var5] as u8);
                byte = (byte << 1) | (variables[&var6] as u8);
                byte = (byte << 1) | (variables[&var7] as u8);
                if byte.is_ascii() {
                    print!("{}", byte as char);
                }
            }
            Instruction::NewLine => {
                println!("");
            }
            Instruction::GoTo(label_name) => {
                if labels.contains_key(&label_name) {
                    i = labels[&label_name];
                    continue;
                } else {
                    eprintln!("No label {} defined", label_name);
                    exit(5);
                }
            }
            Instruction::GoToIf(label_name, var) => {
                ensure_variable_exists(&variables, &var);
                if labels.contains_key(&label_name) {
                    if variables[&var] {
                        i = labels[&label_name];
                        continue;
                    }
                } else {
                    eprintln!("No label {} defined", label_name);
                    exit(5);
                }
            }
        }
        i += 1;
    }
}
