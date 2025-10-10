use sha2::{Sha256, Digest};
use chrono::prelude::*;
use hex;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: String,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}


impl Block {
    pub fn new(index: u64, data: String, previous_hash: String) -> Block {
        let timestamp = Utc::now().timestamp() as u64;
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash: String::new(),    
            nonce: 0,
        };
        block.hash = block.calculate_hash(); 
        block
    }

    pub fn calculate_hash(&self) -> String {
        let input = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        let mut hasher = Sha256::new();
        hasher.update(input);
        let result = hasher.finalize();
        hex::encode(result)
    }

    pub fn mine_block(&mut self, difficulty: usize) {
        let target = "0".repeat(difficulty); // hash must start with "00.." depending on difficulty
        while &self.hash[..difficulty] != target {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }
        println!("Block mined: {}", self.hash);
    }
}