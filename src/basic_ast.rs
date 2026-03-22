use std::collections::HashMap;

trait IsIdentifier {
    fn is_identifier(&self) -> bool;
}

impl IsIdentifier for str {
    fn is_identifier(&self) -> bool {
        self.contains(|c: char| c.is_alphabetic())
    }
}

// For now, I'm only going to implement the ones that are trivial to implement
// because i am lazy lol
pub enum BasicAST<'a> {
    Init(&'a str, u8),
    Set(&'a str, u8),
    Increment(&'a str),
    Decrement(&'a str),
    For(&'a str, Block<'a>),
    Function(&'a str, Vec<&'a str>, Block<'a>),
    CallFunction(&'a str, Vec<&'a str>),
}

pub struct Block<'a> {
    local_vars: HashMap<&'a str, usize>,
    body: Vec<BasicAST<'a>>,
}

fn parse_block<'a>(tokens: &'a [&'a str]) -> (Block<'a>, usize) {
    let mut block = Block {
        local_vars: HashMap::new(),
        body: Vec::new(),
    };

    let mut current_var_offset = 0;

    let mut i = 0;
    while i < tokens.len() {
        let mut current_statement = &tokens[i..i];
        let statement_start = i;
        let mut child_block = Option::None;

        loop {
            if tokens[i] == ";" {
                i += 1;
                break;
            } else if tokens[i] == "{" {
                i += 1;
                let (child, size) = parse_block(&tokens[i..]);

                child_block = Some(child);
                i += size;
                break;
            } else {
                current_statement = &tokens[statement_start..i];
                i += 1;
            }
        }

        if current_statement.is_empty() {
            continue;
        }

        if current_statement[0] == "for" && child_block.is_some() {
            block
                .body
                .push(BasicAST::For(current_statement[0], child_block.unwrap()));
        } else if current_statement[0] == "function" && child_block.is_some() {
            let function_name = current_statement[1];
            let params = current_statement[2..].to_vec();
            block.body.push(BasicAST::Function(function_name, params, child_block.unwrap()));
        } else if current_statement[0].is_identifier() {
            if current_statement[1] == "=" {
                if block.local_vars.get(current_statement[0]).is_some() {
                    block.body.push(BasicAST::Set(
                        current_statement[0],
                        current_statement[2].parse().unwrap(),
                    ))
                } else {
                    block
                        .local_vars
                        .insert(current_statement[0], current_var_offset);
                    current_var_offset += 1;
                }
            } else if current_statement[1] == "+" && current_statement[2] == "+" {
                block.body.push(BasicAST::Increment(current_statement[0]));
            } else if current_statement[1] == "-" && current_statement[2] == "-" {
                block.body.push(BasicAST::Decrement(current_statement[0]));
            }
        }
    }

    (block, i)
}

// Oh god the lifetimes are spreading everywhere
