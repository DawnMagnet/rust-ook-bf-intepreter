use clap::{Parser, Subcommand};
use log::{debug, error, info, warn};
use std::fs;
use std::io::Read;
use std::path::Path;

const MAX_DATA_SIZE: usize = 30000;
const MAX_OUTPUT_SIZE: usize = 10000;
struct Intepreter {
    code: Vec<u8>,
    data: Vec<u8>,
    match_stack: Vec<usize>,
    code_ptr: usize,
    data_ptr: usize,
}

impl Intepreter {
    fn new(code: Vec<u8>) -> Self {
        debug!("Creating new interpreter with code length: {}", code.len());
        let mut match_stack = vec![0; code.len()];
        let mut stack = Vec::new();
        for (i, &c) in code.iter().enumerate() {
            if c == 7 {
                debug!("Found loop start at position {}", i);
                stack.push(i);
            } else if c == 8 {
                if let Some(open) = stack.pop() {
                    debug!("Matching loop end at {} with start at {}", i, open);
                    match_stack[open] = i;
                    match_stack[i] = open;
                } else {
                    warn!("Unmatched closing bracket at position {}", i);
                }
            }
        }

        if !stack.is_empty() {
            warn!("Unmatched opening brackets at positions: {:?}", stack);
        }

        Intepreter {
            code,
            data: vec![0; MAX_DATA_SIZE],
            code_ptr: 0,
            data_ptr: MAX_DATA_SIZE / 2,
            match_stack,
        }
    }

    fn run(&mut self) -> String {
        info!("Starting interpreter execution");
        let mut output = Vec::with_capacity(MAX_OUTPUT_SIZE);

        while self.code_ptr < self.code.len() {
            debug!(
                "State: ptr={}, data_ptr={}, current_op={}, current_value={}",
                self.code_ptr, self.data_ptr, self.code[self.code_ptr], self.data[self.data_ptr]
            );

            let mut normal = true;
            match self.code[self.code_ptr] {
                1 => self.data_ptr += 1,                            // >
                2 => self.data_ptr -= 1,                            //<
                3 => self.data[self.data_ptr] += 1,                 //+
                4 => self.data[self.data_ptr] -= 1,                 // -
                5 => output.push(self.data[self.data_ptr] as char), //.
                6 => {
                    warn!("There Shouldn't be any input in this mode");
                    let mut input = [0; 1];
                    std::io::stdin().read_exact(&mut input).unwrap();
                    self.data[self.data_ptr] = input[0]
                } // ,
                7 => {
                    if self.data[self.data_ptr] == 0 {
                        self.code_ptr = self.match_stack[self.code_ptr];
                        normal = false;
                    }
                } // [
                8 => {
                    if self.data[self.data_ptr] != 0 {
                        self.code_ptr = self.match_stack[self.code_ptr];
                        normal = false;
                    }
                } // ]
                _ => {}
            }
            if normal {
                self.code_ptr += 1;
            }
        }
        info!("Execution completed");
        // info!("output: {:?}", output);
        output.into_iter().collect()
    }
}

fn brainfuck_to_bytecode(code: &str) -> Vec<u8> {
    debug!(
        "Converting brainfuck code to bytecode, length: {}",
        code.len()
    );
    let code = code.chars().filter_map(|c| match c {
        '>' => Some(1),
        '<' => Some(2),
        '+' => Some(3),
        '-' => Some(4),
        '.' => Some(5),
        ',' => Some(6),
        '[' => Some(7),
        ']' => Some(8),
        _ => None,
    });
    code.collect()
}

fn ook_to_bytecode(code: &str) -> Vec<u8> {
    debug!("Converting Ook code to bytecode, length: {}", code.len());
    let words: Vec<&str> = code.split_whitespace().collect();

    let mut commands = Vec::new();
    for chunk in words.chunks(2) {
        if chunk.len() == 2 {
            commands.push((chunk[0].to_string(), chunk[1].to_string()));
        }
    }
    commands
        .iter()
        .filter_map(|(first, second)| {
            match (first.as_str(), second.as_str()) {
                ("Ook.", "Ook?") => Some(1), // >
                ("Ook?", "Ook.") => Some(2), // <
                ("Ook.", "Ook.") => Some(3), // +
                ("Ook!", "Ook!") => Some(4), // -
                ("Ook!", "Ook.") => Some(5), // .
                ("Ook.", "Ook!") => Some(6), // ,
                ("Ook!", "Ook?") => Some(7), // [
                ("Ook?", "Ook!") => Some(8), // ]
                _ => None,
            }
        })
        .collect()
}

fn parse_short_ook_commands(code: &str) -> Vec<u8> {
    let iter: Vec<char> = code
        .chars()
        .filter(|c| matches!(c, '.' | '?' | '!'))
        .collect();
    let mut commands = vec![];
    for chunk in iter.chunks(2) {
        if chunk.len() == 2 {
            commands.push((chunk[0], chunk[1]));
        }
    }
    commands
        .iter()
        .filter_map(|(first, second)| match (*first, *second) {
            ('.', '?') => Some(1),
            ('?', '.') => Some(2),
            ('.', '.') => Some(3),
            ('!', '!') => Some(4),
            ('!', '.') => Some(5),
            ('.', '!') => Some(6),
            ('!', '?') => Some(7),
            ('?', '!') => Some(8),
            _ => None,
        })
        .collect()
}

// Add a test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_brainfuck_to_bytecode() {
        let code = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
        let expected = "Hello World!\n";
        let bytecode = brainfuck_to_bytecode(code);
        let mut interpreter = Intepreter::new(bytecode);
        let output = interpreter.run();
        assert_eq!(output, expected);
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run brainfuck code
    #[command(alias = "b")]
    Brainfuck {
        /// Input file (use - for stdin)
        file: String,
    },
    /// Run ook code
    #[command(alias = "o")]
    Ook {
        /// Input file (use - for stdin)
        file: String,
    },
    /// Run in short ook mode
    #[command(alias = "so")]
    ShortOok {
        /// Input file (use - for stdin)
        file: String,
        /// char mapping to ".?!" (default: ".?!")
        #[arg(short, long)]
        mapping: Option<String>,
    },
}

fn read_input(file_path: String) -> std::io::Result<String> {
    if !std::path::Path::new(&file_path).exists() {
        Ok(file_path)
    } else {
        let path = Path::new(&file_path);
        let content = fs::read_to_string(path)?;
        Ok(content)
    }
}

fn main() {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    info!("Starting Brainfuck/Ook interpreter");

    let cli = Cli::parse();

    match cli.command {
        Commands::Brainfuck { file } => {
            info!("Running in brainfuck mode");
            match read_input(file) {
                Ok(code) => {
                    debug!("Code length: {}", code.len());

                    let bytecode = brainfuck_to_bytecode(&code);

                    let mut interpreter = Intepreter::new(bytecode);
                    let res = interpreter.run();
                    info!("Output:");
                    println!("{}", res);
                }
                Err(e) => error!("Failed to read input: {}", e),
            }
        }
        Commands::Ook { file } => {
            info!("Running in ook mode");
            match read_input(file) {
                Ok(code) => {
                    debug!("Code length: {}", code.len());
                    let bytecode = ook_to_bytecode(&code);
                    let mut interpreter = Intepreter::new(bytecode);
                    let res = interpreter.run();
                    info!("Output:");
                    println!("{}", res);
                }
                Err(e) => error!("Failed to read input: {}", e),
            }
        }
        Commands::ShortOok { file, mapping } => {
            info!("Running in short ook mode");
            match read_input(file) {
                Ok(mut code) => {
                    debug!("Code length: {}", code.len());

                    if let Some(rmp) = mapping {
                        let rmpch = rmp.chars().collect::<Vec<char>>();
                        let new_code = code
                            .chars()
                            .map(|c| {
                                if c == rmpch[0] {
                                    '.'
                                } else if c == rmpch[1] {
                                    '?'
                                } else if c == rmpch[2] {
                                    '!'
                                } else {
                                    c
                                }
                            })
                            .collect::<String>();
                        debug!("Mapping Done! New code: {}", new_code);
                        code = new_code;
                    }
                    let bytecode = parse_short_ook_commands(&code);
                    let mut interpreter = Intepreter::new(bytecode);
                    let res = interpreter.run();
                    info!("Output:");
                    println!("{}", res);
                }
                Err(e) => error!("Failed to read input: {}", e),
            }
        }
    }
}
