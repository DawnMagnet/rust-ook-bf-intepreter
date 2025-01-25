use clap::{Parser, Subcommand};
use log::{debug, error, info};
use std::fs;
use std::path::Path;

use rob::{brainfuck_to_bytecode, ook_to_bytecode, parse_short_ook_commands, Intepreter};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    #[arg(long, default_value_t = 50000)]
    max_data_size: usize,

    #[arg(long, default_value_t = 50000)]
    max_output_size: usize,
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
    let cli = Cli::parse();

    env_logger::init_from_env(
        env_logger::Env::default().default_filter_or(if cli.verbose { "trace" } else { "error" }),
    );
    info!("Starting Brainfuck/Ook interpreter");

    match cli.command {
        Commands::Brainfuck { file } => {
            info!("Running in brainfuck mode");
            match read_input(file) {
                Ok(code) => {
                    debug!("Code length: {}", code.len());

                    let bytecode = brainfuck_to_bytecode(&code);

                    let mut interpreter =
                        Intepreter::new(bytecode, cli.max_data_size, cli.max_output_size);
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
                    let mut interpreter =
                        Intepreter::new(bytecode, cli.max_data_size, cli.max_output_size);
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
                    let mut interpreter =
                        Intepreter::new(bytecode, cli.max_data_size, cli.max_output_size);
                    let res = interpreter.run();
                    info!("Output:");
                    println!("{}", res);
                }
                Err(e) => error!("Failed to read input: {}", e),
            }
        }
    }
}
