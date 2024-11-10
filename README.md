# Block validation in Rust
This Rust program simulates a simple blockchain system with functionality for creating and validating blocks. It uses SHA-256 hashing to secure blocks and checks for block validity based on attributes like hash, previous hash, and nonce. The purpose of this project is to demonstrate basic blockchain concepts such as block creation, mining, and validation.

## Code Overview
### Structs
- `Blockchain`: Represents the entire blockchain, storing blocks in a vector.
- `Block`: Represents an individual block, with fields for ID, nonce, data, hash, previous hash, and timestamp.

### Functions
1. Blockchain::new(): Initializes a new empty blockchain.

2. Blockchain::starting_block(): Adds the genesis (first) block to the blockchain with predefined values.

3. Blockchain::try_add_block(block: Block): Attempts to add a new block to the blockchain, verifying its validity before adding.

4. Blockchain::is_block_valid(new_block: &Block, latest_block: &Block): Validates a block by checking:
- `previous_hash` of the new block matches the hash of the latest block.
- `hash` of the new block starts with "0000".
- `id` of the new block is the next in sequence.
- Hash integrity by recomputing the hash with block data and checking it matches.

5. Blockchain::validate_blockchain(): Verifies the entire blockchain by checking if all blocks are valid and correctly linked.

6. Block::new(id: u64, previous_hash: String, data: String): Creates a new block, mines a valid hash, and returns the block with a unique nonce and timestamp.

7. Block::mine_block(id: u64, timestamp: i64, previous_hash: &String, data: &String): Mines the block by finding a nonce such that the resulting hash starts with "0000".
