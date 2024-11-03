use chrono::Utc;
use sha256::digest;

#[derive(Debug, Clone)]
struct Blockchain {
    blocks: Vec<Block>,
}

#[derive(Debug, Clone)]
struct Block {
    id: u64,
    nonce: u64,
    data: String,
    hash: String,
    previous_hash: String,
    timestamp: i64,
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn starting_block(&mut self) {
        let genesis_block = Block {
            id: 1,
            data: String::from("Genesis Block"),
            previous_hash: String::from(
                "0000000000000000000000000000000000000000000000000000000000000000",
            ),
            nonce: 91493,
            hash: String::from("0000b3d1d90e1c6675403a95af8a63751ebfb0f43b6c528b05d4bb54849eefee"),
            timestamp: Utc::now().timestamp(),
        };

        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, block: Block) {
        match self.blocks.last() {
            Some(latest_block) => {
                if self.is_block_valid(&block, latest_block) {
                    self.blocks.push(block);
                    println!("Block has been successfully added");
                } else {
                    println!("Invalid Block!");
                }
            }

            None => {
                println!("the blockchain need the first block");
            }
        }
    }

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> bool {
        if new_block.previous_hash != latest_block.hash {
            println!("block with id {}, has wrong previous hash", new_block.id);
            return false;
        } else if !new_block.hash.starts_with("0000") {
            println!("the hash not validate: {}", new_block.hash);
            return false;
        } else if new_block.id != latest_block.id + 1 {
            println!(
                "block with id {}, is not the next block after the latest block with id {}",
                new_block.id, latest_block.id
            );
            return false;
        } else if digest(format!(
            "{}{}{}{}{}",
            new_block.id,
            &new_block.previous_hash,
            &new_block.data,
            new_block.timestamp,
            &new_block.nonce
        )) != new_block.hash
        {
            print!("block with id {} has invalid input", new_block.id);
            return false;
        }

        true
    }

    fn validate_blockchain(self) -> bool {
        if self.blocks.is_empty() {
            return false;
        }

        for i in 1..self.blocks.len() {
            if !self.is_block_valid(&self.blocks[i], &self.blocks[i - 1]) {
                return false;
            }
        }
        true
    }
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String) -> Self {
        let _now = Utc::now();
        let now_timestamp = Utc::now().timestamp();

        let (nonce, hash) = Block::mine_block(id, now_timestamp, &previous_hash, &data);

        Self {
            data,
            id,
            hash,
            nonce,
            previous_hash,
            timestamp: now_timestamp,
        }
    }

    fn mine_block(id: u64, timestamp: i64, previous_hash: &String, data: &String) -> (u64, String) {
        println!("mining block...");
        let mut nonce = 1;

        loop {
            let block_string = format!("{}{}{}{}{}", id, previous_hash, data, timestamp, nonce);

            let hash = digest(block_string);

            if hash.starts_with("0000") {
                println!("mined! nonce: {}, hash: {}", nonce, hash);
                return (nonce, hash);
            }
            nonce += 1;
        }
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.starting_block();

    println!("the first block {:?}", blockchain);

    let new_block = Block::new(2, blockchain.blocks[0].hash.to_owned(), "Test".to_string());
    blockchain.try_add_block(new_block);

    let new_block2 = Block::new(3, blockchain.blocks[1].hash.to_owned(), "Test2".to_string());
    blockchain.try_add_block(new_block2);

    let new_block3 = Block::new(4, blockchain.blocks[2].hash.to_owned(), "Test3".to_string());
    blockchain.try_add_block(new_block3);

    println!("status blockchain: {}", blockchain.validate_blockchain());
}
