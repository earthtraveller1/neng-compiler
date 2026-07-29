use std::env;

#[cfg(test)]
mod tests;

mod assembler;
mod basic_ast;
mod codegen;

struct CommandLineOptions {
    input_file: String,
    output_file: String,
}

impl CommandLineOptions {
    fn get() -> CommandLineOptions {
        let mut options = CommandLineOptions {
            input_file: String::new(),
            output_file: "a.bf".to_string(),
        };

        for arg in env::args() {
            if arg.starts_with("-i=") {
                // We can safely unwrap here as we already ensures that the prefix exists
                options.input_file = arg.strip_prefix("-i=").unwrap().to_string();
            } else if arg.starts_with("-o=") {
                options.output_file = arg.strip_prefix("-o=").unwrap().to_string();
            }
        }

        options
    }
}

fn tokenize_string(string: &str) -> Vec<&str> {
    let mut tokens = Vec::new();

    // Every alphanumberical word is a token
    // Every random character is a token
    // idk

    let mut current_token_start = 0;
    // the token starts out as empty
    let mut current_token = &string[current_token_start..current_token_start];

    for (i, c) in string.char_indices() {
        if c.is_alphanumeric() {
            current_token =
                &string[current_token_start..(current_token_start + current_token.len() + 1)];
        } else {
            // This implies something is in the current token already
            if !current_token.is_empty() {
                tokens.push(current_token);
            }

            // All non-alphanumberical characters are individual tokens
            if !c.is_whitespace() {
                tokens.push(&string[i..i + 1]);
            }

            // Move it forward to the next one
            current_token_start = i + 1;
            current_token = &string[current_token_start..current_token_start];
        }
    }

    tokens
}

fn main() {
    let cli_options = CommandLineOptions::get();
    println!(
        "Input: {}, Output: {}",
        cli_options.input_file, cli_options.output_file
    );

    if cli_options.input_file.is_empty() {
        eprintln!("[ERROR]: No input file specified.");
        return;
    }

    let source_file = match std::fs::read_to_string(&cli_options.input_file) {
        Ok(x) => x,
        Err(e) => {
            eprintln!("[ERROR]: Failed to read {}: {}", &cli_options.input_file, e);
            return;
        }
    };

    let tokens = tokenize_string(&source_file);
    println!("Tokens: {:?}", tokens);
}
