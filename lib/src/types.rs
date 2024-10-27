use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use crate::U256;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::{timestamp, Uuid};

// according to the whitepaper, we need the following basic entities.
// blockchain, block, blockheader and transaction

#[derive(Serialize, Deserialize, Clone, Debug)]
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
#[derive(Serialize, Deserialize, Clone, Debug)]
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
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BlockHeader {
    // Timestamp of the block
    pub timestamp: DateTime<Utc>,
    // When a miner attempts to mine a new block, they include a nonce
    //(a number that they can vary) in the BlockHeader and hash the entire header.
    // It is used to mine the block
    pub nonce: u64,
    // we use an array of u8 (each element is a 8-bit integer) with 32 elements
    // meaning we have 32*8=256 bits which correspond to the output of sha256
    pub prev_block_hash: Hash,
    pub merkle_root: MerkleRoot,
    // a number representing the difficulty. The target is a 256-bit value that represents
    // the maximum allowed hash value for a valid block. The lower the target value, the harder
    // it is to find a valid hash, effectively increasing the difficulty of mining.
    // example: 0x00000000FFFF0000000000000000000000000000000000000000000000000000
    // The U256 type is used because the target requires a full 256-bit number
    pub target: U256,
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: Hash,
        merkle_root: MerkleRoot,
        target: U256,
    ) -> Self {
        Self {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Self { inputs, outputs }
    }
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionInput {
    // the hash of the transaction output, which we are linking
    // into this transaction as input.
    pub prev_transaction_output_hash: Hash,
    // this is how the user proves they can use the output of the previous transaction.
    pub signature: Signature,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionOutput {
    pub value: u64,
    // unique identifier that helps us ensure that the hash of each transaction output is unique.
    pub unique_id: Uuid,
    pub pubkey: PublicKey,
}
impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
