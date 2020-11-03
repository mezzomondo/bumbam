use crate::vm::VM;
use core::num::ParseIntError;
use std;
use std::io;
use std::io::Write;

/// Core structure for the REPL for the Assembler
pub struct REPL {
    command_buffer: Vec<String>,
    // The VM the REPL will use to execute code
    vm: VM,
}

impl REPL {
    /// Creates and returns a new assembly REPL
    pub fn new() -> REPL {
        REPL {
            vm: VM::new(),
            command_buffer: vec![],
        }
    }
    pub fn run(&mut self) {
        println!("Welcome to BumBam <3");
        loop {
            // This allocates a new String in which to store whatever the user types each iteration.
            // TODO: Figure out how create this outside of the loop and re-use it every iteration
            let mut buffer = String::new();

            // Blocking call until the user types in a command
            let stdin = io::stdin();

            // Annoyingly, `print!` does not automatically flush stdout like `println!` does, so we
            // have to do that there for the user to see our `>>> ` prompt.
            print!(">>> ");
            io::stdout().flush().expect("Unable to flush stdout");

            // Here we'll look at the string the user gave us.
            stdin
                .read_line(&mut buffer)
                .expect("Unable to read line from user");
            let buffer = buffer.trim();

            match buffer {
                "quit" => {
                    println!("ByeBumBam!");
                    std::process::exit(0);
                }
                "history" => {
                    for command in &self.command_buffer {
                        println!("{}", command);
                    }
                }
                "program" => {
                    println!("Listing instructions currently in VM's program vector:");
                    for instruction in &self.vm.program {
                        println!("{}", instruction);
                    }
                    println!("End of Program Listing");
                }
                "registers" => {
                    println!("Listing registers and all contents:");
                    println!("{:#?}", self.vm);
                    println!("End of Register Listing")
                }
                _ => {
                    let results = self.parse_hex(buffer);
                    match results {
                        Ok(bytes) => {
                            for byte in bytes {
                                self.vm.add_byte(byte)
                            }
                        },
                        Err(_e) => {
                            println!("Unable to decode hex string. Please enter 4 groups of 2 hex characters.")
                        }
                    };
                    self.vm.run_once();
                }
            }

            // This is the line we add to store a copy of each command
            self.command_buffer.push(buffer.to_string());
        }
    }
    /// Accepts a hexadecimal string WITHOUT a leading `0x` and returns a Vec of u8
    /// Example for a LOAD command: 00 01 03 E8
    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut results: Vec<u8> = vec![];
        for hex_string in split {
            let byte = u8::from_str_radix(&hex_string, 16);
            match byte {
                Ok(result) => {
                    results.push(result);
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(results)
    }
}
