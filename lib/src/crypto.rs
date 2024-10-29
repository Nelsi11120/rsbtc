/*
What algorithm do we use, and what do we name the module? Real bitcoin uses
ECDSA (Elliptic Curve Digital Signature Algorithm), and we can do the same.
ECDSA can use different elliptical curve parameters, and we can use the so-called
secp256k1, just like bitcoin.
*/

use ecdsa::{
    signature::{Signer, Verifier},
    Signature as ECDSASignature, SigningKey, VerifyingKey,
};
use k256::Secp256k1;
use serde::{Deserialize, Serialize};

use crate::sha256::Hash;

// The signature serves as a proof that a particular entity has autorizhed a transaction.
// -> The sender of the transaction is indeed the owner of the funds being spent.
// Here, we use ECDSA with the secp256k1 curve.
// When a user creates a transaction, they create a hash of the transaction details
// (exluding the signature), and then sign this hash using their private key.
// The resulting signature (r,s) is stored in the signature field of the TransactionInput.
// When a transaction is verified, the nodes will hash the transaction (excluding the signature),
// using the public key associated with the TransactionOutput, they verify the signature.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Signature(ECDSASignature<Secp256k1>);

impl Signature {
    // sign a TransactionOutput
    pub fn sign_output(output_hash: &Hash, private_key: &PrivateKey) -> Self {
        let signing_key = &private_key.0;
        let signature = signing_key.sign(&output_hash.as_bytes());
        Signature(signature)
    }
    // verify a signature
    pub fn verify(&self, output_hash: &Hash, public_key: &PublicKey) -> bool {
        public_key
            .0
            .verify(&output_hash.as_bytes(), &self.0)
            .is_ok()
    }
}

// PublicKey is used to verify the signature made with the corresponding private key.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PublicKey(VerifyingKey<Secp256k1>);
// PrivateKey is used to create digital signature for transactions.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrivateKey(#[serde(with = "signkey_serde")] SigningKey<Secp256k1>);

impl PrivateKey {
    pub fn new_key() -> Self {
        Self(SigningKey::random(&mut rand::thread_rng()))
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key().clone())
    }
}

mod signkey_serde {
    use serde::Deserialize;

    pub fn serialize<S>(
        key: &super::SigningKey<super::Secp256k1>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(&key.to_bytes())
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<super::SigningKey<super::Secp256k1>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let bytes = Vec::<u8>::deserialize(deserializer)?;
        Ok(super::SigningKey::from_slice(&bytes).unwrap())
    }
}
