use super::PullStatementTrait;
use std::collections::HashMap;

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

fn parse_code(tokens: &[&str]) -> Vec<Statement> {
    let mut start_token = 0;
    let mut variables = HashMap::new();

    let mut statements = Vec::new();

    loop {
        let statement = &tokens[start_token..].pull_statement();
        statements.push(parse_statement(statement, &mut variables));

        if start_token >= tokens.len() {
            break;
        }

        start_token += statement.len() + 1;
    }

    statements
}

fn parse_statement(tokens: &[&str], variables: &mut HashMap<&str, u64>) -> Statement {
    todo!()
}
