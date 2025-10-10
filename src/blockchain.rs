use crate::block::Block;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
}
impl Blockchain {
    // Create a new blockchain with genesis block
    pub fn new() -> Blockchain {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            blocks: vec![genesis_block],
        }
    }

    // Add a new block with mining
    pub fn add_block(&mut self, data: String, difficulty: usize) {
        let last_hash = self.blocks.last().unwrap().hash.clone();
        let mut new_block = Block::new(self.blocks.len() as u64, data, last_hash);
        new_block.mine_block(difficulty);
        self.blocks.push(new_block);
    }

    // Validate the blockchain
    pub fn is_valid(&self, difficulty: usize) -> bool {
        for i in 1..self.blocks.len() {
            let current = &self.blocks[i];
            let previous = &self.blocks[i - 1];

            // Check current hash
            if current.hash != current.calculate_hash() {
                return false;
            }

            // Check previous hash
            if current.previous_hash != previous.hash {
                return false;
            }

            // Check difficulty
            if &current.hash[..difficulty] != "0".repeat(difficulty) {
                return false;
            }
        }
        true
    }
}
