use std::env;

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

fn main() {
    let cli_options = CommandLineOptions::get();
    println!("Input: {}, Output: {}", cli_options.input_file, cli_options.output_file);
}
