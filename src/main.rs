use chrono::Utc;
use sha256::digest;

// Transaction struct => sender, receiver, amount
#[derive(Debug, Clone)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f64,
}

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
    transactions: Vec<Transaction>,
}

impl Transaction {
    fn new(sender: &str, receiver: &str, amount: f64) -> Self {
        Self {
            sender: sender.to_string(),
            receiver: receiver.to_string(),
            amount,
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Self {
            receiver: "system".to_string(),
            sender: "genesis".to_string(),
            amount: 0.0,
        }
    }
}

impl Blockchain {
    fn new() -> Self {
        Self { blocks: vec![] }
    }

    fn starting_block(&mut self, data: String) {
        let id = 1;
        let data = data;
        let previous_hash =
            String::from("0000000000000000000000000000000000000000000000000000000000000000");
        let timestamp = Utc::now().timestamp();

        // Genesis block has no transactions
        let transactions = vec![Transaction::default()];

        let (nonce, hash) = Block::mine_block(id, timestamp, &previous_hash, &data, &transactions);

        let genesis_block = Block {
            id,
            data,
            previous_hash,
            nonce,
            hash,
            timestamp,
            transactions,
        };

        self.blocks.push(genesis_block);
    }

    fn try_add_block(&mut self, data: String, transactions: Vec<Transaction>) {
        let previous_hash = self
            .blocks
            .last()
            .map(|b| b.hash.clone())
            .unwrap_or_default();

        let id = self.blocks.len() as u64 + 1;

        let new_block = Block::new(id, previous_hash, data, transactions);

        match self.blocks.last() {
            Some(latest_block) => {
                if self.is_block_valid(&new_block, latest_block).unwrap() {
                    self.blocks.push(new_block);
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

    fn is_block_valid(&self, new_block: &Block, latest_block: &Block) -> Result<bool, String> {
        if new_block.previous_hash != latest_block.hash {
            return Err(format!(
                "block with id {}, has wrong previous hash",
                new_block.id
            ));
        } else if !new_block.hash.starts_with("0000") {
            return Err(format!("the hash not validate: {}", new_block.hash));
        } else if new_block.id != latest_block.id + 1 {
            return Err(format!(
                "block with id {}, is not the next block after the latest block with id {}",
                new_block.id, latest_block.id
            ));
        } else if new_block.calculate_hash() != new_block.hash {
            return Err(format!("block with id {} has invalid input", new_block.id));
        }

        Ok(true)
    }

    fn validate_blockchain(&self) -> bool {
        if self.blocks.is_empty() {
            return false;
        }

        for i in 1..self.blocks.len() {
            if let Err(err) = self.is_block_valid(&self.blocks[i], &self.blocks[i - 1]) {
                println!("Blockchain validation error: {}", err);
                return false;
            }
        }

        true
    }
}

impl Block {
    fn new(id: u64, previous_hash: String, data: String, transactions: Vec<Transaction>) -> Self {
        let now_timestamp = Utc::now().timestamp();

        let (nonce, hash) =
            Block::mine_block(id, now_timestamp, &previous_hash, &data, &transactions);

        Self {
            data,
            id,
            hash,
            nonce,
            previous_hash,
            timestamp: now_timestamp,
            transactions,
        }
    }

    fn calculate_hash(&self) -> String {
        digest(format!(
            "{}{}{}{}{}{:?}",
            self.id, self.previous_hash, self.data, self.timestamp, self.nonce, self.transactions
        ))
    }

    fn mine_block(
        id: u64,
        timestamp: i64,
        previous_hash: &String,
        data: &String,
        transactions: &Vec<Transaction>,
    ) -> (u64, String) {
        println!("mining block...");
        let mut nonce = 1;

        loop {
            let block_string = format!(
                "{}{}{}{}{}{:?}",
                id, previous_hash, data, timestamp, nonce, transactions
            );

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
    blockchain.starting_block("Genesis Block".to_string());
    println!("the first block: {:?}", blockchain.blocks[0]);

    let transaction = Transaction::new("sender_address", "receiver_address", 5.0);

    let transactions: Vec<Transaction> = vec![transaction];

    blockchain.try_add_block(String::from("Second block"), transactions.clone());
    println!("the second block: {:?}", blockchain.blocks[1]);

    blockchain.try_add_block(String::from("Third block"), transactions.clone());
    println!("the third block: {:?}", blockchain.blocks[2]);

    println!("status blockchain: {}", blockchain.validate_blockchain());
}
