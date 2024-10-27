// according to the whitepaper, we need the following basic entities.
// blockchain, block, blockheader and transaction
use crate::U256;
pub struct Blockchain {
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Self {
            header,
            transactions,
        }
    }
    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

pub struct BlockHeader {
    pub timestamp: u64,
    pub nonce: u64,
    // we use an array of u8 (each element is a 8-bit integer) with 32 elements
    // meaning we have 32*8=256 bits which correspond to the output of sha256
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub target: U256,
}
pub struct Transaction;
