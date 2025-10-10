mod block;
mod blockchain;

use blockchain::Blockchain;
use std::io::{self, Write};

fn main() {
    let mut chain = Blockchain::new();
    let difficulty = 2; // mining difficulty

    loop {
        print!("Enter command (add/print/validate/exit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.starts_with("add ") {
            let data = input[4..].to_string();
            chain.add_block(data, difficulty);
        } else if input == "print" {
            for block in &chain.blocks {
                println!("Block {}: {}", block.index, block.hash);
                println!("  Data: {}", block.data);
                println!("  Previous Hash: {}", block.previous_hash);
                println!("  Nonce: {}", block.nonce);
                println!("  Timestamp: {}", block.timestamp);
            }
        } else if input == "validate" {
            let valid = chain.is_valid(difficulty);
            println!("Blockchain valid? {}", valid);
        } else if input == "exit" {
            break;
        } else {
            println!("Unknown command!");
        }
    }
}
