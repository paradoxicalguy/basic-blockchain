use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: usize,
}

impl Blockchain {
    // create a new blockchain with the genesis block
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4, // can raise this later
        };

        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        blockchain.chain.push(genesis_block);

        blockchain
    }

    // get the last block in the chain
    pub fn get_last_block(&self) -> &Block {
        self.chain.last().expect("Blockchain should have at least one block")
    }

    // add a new block to the chain (with mining)
    pub fn add_block(&mut self, data: String) {
        let last_block = self.get_last_block();
        let mut new_block = Block::new(
            last_block.index + 1,
            data,
            last_block.hash.clone(),
        );

        println!("\n👾 mining block {} ...", new_block.index);
        new_block.mine_block(self.difficulty);

        self.chain.push(new_block);
    }

    // validate that the entire chain is correct
    pub fn is_chain_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // recalculate hash and compare
            if current_block.hash != current_block.calculate_hash() {
                println!("❌ Invalid hash at block {}", current_block.index);
                return false;
            }

            // compare stored previous hash
            if current_block.previous_hash != previous_block.hash {
                println!("❌ Invalid previous hash link at block {}", current_block.index);
                return false;
            }
        }
        true
    }

    pub fn print_chain(&self) {
        for block in &self.chain {
            println!("{:#?}", block);
        }
    }
}
