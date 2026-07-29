use std::{collections::HashMap, num::ParseIntError};

enum Statement {
    Assign {
        addr: u64,
        value: u8,
    },
    Copy {
        src: u64,
        dst: u64,
    },
    Add {
        addr: u64,
        value: u8,
    },
    Subtract {
        addr: u64,
        value: u8,
    },

    Print {
        addr: u64,
    },
    Read {
        addr: u64,
    },

    If {
        target_addr: u64,
        body: Vec<Statement>,
    },
    While {
        target_addr: u64,
        body: Vec<Statement>,
    },
}

struct Stackframe<'a> {
    variables: HashMap<&'a str, u64>,
    base_addr: u64,
    top_addr: u64,
}

trait Stack<'a> {
    fn push_new_frame(&mut self);
    fn find_variable_addr(&self, name: &str) -> Option<u64>;
    fn find_or_create_var_addr(&mut self, name: &'a str) -> u64;
}

impl<'a> Stack<'a> for Vec<Stackframe<'a>> {
    fn push_new_frame(&mut self) {
        let base_addr = if let Some(previous_frame) = self.last() {
            previous_frame.top_addr + 2
        } else {
            0
        };

        self.push(Stackframe {
            variables: HashMap::new(),
            base_addr,
            top_addr: base_addr,
        })
    }

    fn find_variable_addr(&self, name: &str) -> Option<u64> {
        for frame in self.iter().rev() {
            if frame.variables.contains_key(name) {
                return frame.variables.get(name).map(|x| *x);
            }
        }

        None
    }

    fn find_or_create_var_addr(&mut self, name: &'a str) -> u64 {
        if let Some(addr) = self.find_variable_addr(name) {
            addr
        } else {
            let frame = self.last_mut().unwrap();
            frame.variables.insert(name, frame.top_addr + 2);
            frame.top_addr + 2
        }
    }
}

fn pull_statement<'a>(tokens: &'a [&str]) -> Result<&'a [&'a str], &'static str> {
    let mut open_braces = 0;
    let mut closing_braces = 0;
    let mut i = 0;

    while tokens[i] != ";" || open_braces != closing_braces {
        if i >= tokens.len() {
            return Err("Statement unconcluded");
        }

        if tokens[i] == "{" {
            open_braces += 1;
        } else if tokens[i] == "}" {
            closing_braces += 1;
        }

        i += 1;
    }

    Ok(&tokens[..i])
}

fn parse_code<'a>(tokens: &'a [&'a str], stack: &mut Vec<Stackframe<'a>>) -> Result<Vec<Statement>, String> {
    let mut start_token = 0;
    let mut statements = Vec::new();

    loop {
        if start_token >= tokens.len() {
            break;
        }

        let statement = pull_statement(&tokens[start_token..])?;
        statements.push(parse_statement(statement, stack)?);

        start_token += statement.len() + 1;
    }

    Ok(statements)
}

fn is_number(string: &str) -> bool {
    for c in string.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn parse_statement<'a>(
    tokens: &[&'a str],
    stack: &mut Vec<Stackframe<'a>>,
) -> Result<Statement, String> {
    let first_token = *tokens.first().ok_or("Empty statements are not allowed.")?;
    if first_token == "if" {
        todo!();
    } else if first_token == "whble" {
        todo!();
    }

    let second_token = *tokens.get(1).ok_or("Invalid statement")?;

    if second_token == "=" {
        let source_operand = tokens.get(2).ok_or("Invalid statement")?;
        let dest_address = stack.find_or_create_var_addr(first_token);

        // If the value is a constant integer, it is a simple assign
        // operation
        if is_number(source_operand) {
            return Ok(Statement::Assign {
                addr: dest_address,
                value: (*source_operand)
                    .parse()
                    .map_err(|x: ParseIntError| x.to_string())?,
            });
        }
        // Copy from another variable
        else {
            let source_address = stack
                .find_variable_addr(source_operand)
                .ok_or("Non-existent variable")?;

            return Ok(Statement::Copy {
                src: source_address,
                dst: dest_address,
            });
        }
    } else if *tokens.get(2).ok_or("Invalid statement")? == "=" {
        let operation = *tokens.get(3).ok_or("Invalid statement")?;
        let dest_address = stack.find_or_create_var_addr(first_token);
        let source = *tokens.get(4).ok_or("Invalid statement")?;

        if is_number(source) {
            let value = source.parse::<u8>().map_err(|x| x.to_string())?;

            if operation == "+" {
                return Ok(Statement::Add {
                    addr: dest_address,
                    value,
                });
            } else if operation == "-" {
                return Ok(Statement::Subtract {
                    addr: dest_address,
                    value,
                });
            }
        }
    }

    Err("Invalid statement".to_string())
}
