use crate::crypto::{PublicKey, Signature};
use crate::error::{BtcError, Result};
use crate::sha256::Hash;
use crate::util::MerkleRoot;
use crate::U256;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// according to the whitepaper, we need the following basic entities.
// blockchain, block, blockheader and transaction

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Blockchain {
    pub utxos: HashMap<Hash, TransactionOutput>,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            utxos: HashMap::new(),
            blocks: vec![],
        }
    }

    pub fn add_block(&mut self, block: Block) -> Result<()> {
        // check if the block is valid
        if self.blocks.is_empty() {
            // if this is the first block, check if the block's
            // previous hash is all zeros.
            if block.header.prev_block_hash != Hash::zero() {
                return Err(BtcError::InvalidBlock);
            } else {
                // check if the block's previous hash is the
                // hash of the last block
                let last_block = self.blocks.last().unwrap();
                if block.header.prev_block_hash != last_block.hash() {
                    return Err(BtcError::InvalidBlock);
                }
                // check if the block's hash is less than the target
                if !block.header.hash().matches_target(block.header.target) {
                    return Err(BtcError::InvalidBlock);
                }
                // check if the block's merkle root is correct
                let calculated_merkle_root = MerkleRoot::calculate(&block.transactions);
                if calculated_merkle_root != block.header.merkle_root {
                    return Err(BtcError::InvalidMerkleRoot);
                }
                // check if the block's timestam is after the last block's timestamp
                if block.header.timestamp <= last_block.header.timestamp {
                    return Err(BtcError::InvalidBlock);
                }
                // Verify that all trasactions in the block are valid
                unimplemented!();
            }
        }
        self.blocks.push(block);
        Ok(())
    }

    // Rebuild UTXO set from the blockchain
    pub fn rebuild_utxos(&mut self) {
        for block in &self.blocks {
            for transaction in &block.transactions {
                // for every transaction, we make sure to remove the input from the utxos
                // since they are about to be spent, and we insert the outputs as utxos.
                for input in &transaction.inputs {
                    self.utxos.remove(&input.prev_transaction_output_hash);
                }
                for output in &transaction.outputs {
                    self.utxos.insert(output.hash(), output.clone());
                }
            }
        }
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

    // verify all transactions in the block
    pub fn verify_transactions(&self, utxos: &HashMap<Hash, TransactionOutput>) -> Result<()> {
        if self.transactions.is_empty() {
            return Err(BtcError::InvalidTransaction);
        }

        for transaction in &self.transactions {
            // for every transaction in the block
            let mut input_value = 0;
            let mut output_value = 0;
            // hashmap of current input, later utxo if validated
            let mut inputs: HashMap<Hash, TransactionOutput> = HashMap::new();
            for input in &transaction.inputs {
                // at anytime the utxo set represents the available unspent transaction that we can use as input to spent
                // meaning that if the current input is not inside the utxo set, it is not a valid transaction
                let prev_output = utxos.get(&input.prev_transaction_output_hash);
                if prev_output.is_none() {
                    return Err(BtcError::InvalidTransaction);
                }
                let prev_output = prev_output.unwrap();
                // prevent double spending in same block
                if inputs.contains_key(&input.prev_transaction_output_hash) {
                    return Err(BtcError::InvalidTransaction);
                }
                // check if signature is valid. We need to take the public key associated and the previous hash
                if !input
                    .signature
                    .verify(&input.prev_transaction_output_hash, &prev_output.pubkey)
                {
                    return Err(BtcError::InvalidTransaction);
                }
                // keep track of input values from prev output
                input_value += prev_output.value;
                // add it to inputs to prevent double spending
                inputs.insert(input.prev_transaction_output_hash, prev_output.clone());
            }

            // At this point, the transactions are almost validated, we just need to check that
            // the output value is less or equal than the input
            for output in &transaction.outputs {
                output_value += output.value;
            }

            if input_value < output_value {
                return Err(BtcError::InvalidTransaction);
            }
        }
        Ok(())
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
