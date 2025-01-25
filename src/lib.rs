use log::{debug, info, warn};
use std::io::Read;

pub struct Intepreter {
    code: Vec<u8>,
    data: Vec<u8>,
    match_stack: Vec<usize>,
    code_ptr: usize,
    data_ptr: usize,
    max_output_size: usize,
}

impl Intepreter {
    pub fn new(code: Vec<u8>, max_data_size: usize, max_output_size: usize) -> Self {
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
            data: vec![0; max_data_size],
            code_ptr: 0,
            data_ptr: max_data_size / 2,
            match_stack,
            max_output_size,
        }
    }

    pub fn run(&mut self) -> String {
        info!("Starting interpreter execution");
        let mut output = Vec::with_capacity(self.max_output_size);

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

pub fn brainfuck_to_bytecode(code: &str) -> Vec<u8> {
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

pub fn ook_to_bytecode(code: &str) -> Vec<u8> {
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

pub fn parse_short_ook_commands(code: &str) -> Vec<u8> {
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
