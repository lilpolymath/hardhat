use std::sync::OnceLock;

use revm_primitives::{keccak256, ruint::aliases::U64, Bytes, B256, U256};
use secp256k1::SecretKey;

use crate::{
    access_list::AccessListItem,
    signature::Signature,
    transaction::{kind::TransactionKind, signed::EIP2930SignedTransaction},
    utils::envelop_bytes,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "fastrlp",
    derive(open_fastrlp::RlpEncodable, open_fastrlp::RlpDecodable)
)]
pub struct EIP2930TransactionRequest {
    pub chain_id: u64,
    pub nonce: u64,
    pub gas_price: U256,
    pub gas_limit: u64,
    pub kind: TransactionKind,
    pub value: U256,
    pub input: Bytes,
    pub access_list: Vec<AccessListItem>,
}

impl EIP2930TransactionRequest {
    /// Computes the hash of the transaction.
    pub fn hash(&self) -> B256 {
        let encoded = rlp::encode(self);

        keccak256(&envelop_bytes(1, &encoded))
    }

    /// Signs the transaction with the provided private key.
    pub fn sign(self, private_key: &SecretKey) -> EIP2930SignedTransaction {
        let hash = self.hash();

        let signature = Signature::new(hash, private_key);

        EIP2930SignedTransaction {
            chain_id: self.chain_id,
            nonce: self.nonce,
            gas_price: self.gas_price,
            gas_limit: self.gas_limit,
            kind: self.kind,
            value: self.value,
            input: self.input,
            access_list: self.access_list.into(),
            odd_y_parity: signature.odd_y_parity(),
            r: signature.r,
            s: signature.s,
            hash: OnceLock::new(),
        }
    }
}

impl From<&EIP2930SignedTransaction> for EIP2930TransactionRequest {
    fn from(tx: &EIP2930SignedTransaction) -> Self {
        Self {
            chain_id: tx.chain_id,
            nonce: tx.nonce,
            gas_price: tx.gas_price,
            gas_limit: tx.gas_limit,
            kind: tx.kind,
            value: tx.value,
            input: tx.input.clone(),
            access_list: tx.access_list.0.clone(),
        }
    }
}

impl rlp::Encodable for EIP2930TransactionRequest {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_list(8);
        s.append(&U64::from(self.chain_id));
        s.append(&U64::from(self.nonce));
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.kind);
        s.append(&self.value);
        s.append(&self.input.as_ref());
        s.append_list(&self.access_list);
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use revm_primitives::Address;

    use super::*;

    fn dummy_request() -> EIP2930TransactionRequest {
        let to = Address::from_str("0xc014ba5ec014ba5ec014ba5ec014ba5ec014ba5e").unwrap();
        let input = hex::decode("1234").unwrap();
        EIP2930TransactionRequest {
            chain_id: 1,
            nonce: 1,
            gas_price: U256::from(2),
            gas_limit: 3,
            kind: TransactionKind::Call(to),
            value: U256::from(4),
            input: Bytes::from(input),
            access_list: vec![AccessListItem {
                address: Address::zero(),
                storage_keys: vec![B256::zero(), B256::from(U256::from(1))],
            }],
        }
    }

    #[test]
    fn test_eip2930_transaction_request_encoding() {
        // Generated by Hardhat
        // QUESTION: What is considered a valid RLP-encoding? With the prepending type? or without?
        let expected =
            hex::decode("f87a0101020394c014ba5ec014ba5ec014ba5ec014ba5ec014ba5e04821234f85bf859940000000000000000000000000000000000000000f842a00000000000000000000000000000000000000000000000000000000000000000a00000000000000000000000000000000000000000000000000000000000000001")
                .unwrap();

        let request = dummy_request();

        let encoded = rlp::encode(&request);
        assert_eq!(expected, encoded.to_vec());
    }

    #[test]
    fn test_eip2930_transaction_request_hash() {
        // Generated by hardhat
        let expected = B256::from_slice(
            &hex::decode("bc070f66a83bf3513c9db59e7ccaf68870b148cc40b3da9bf20a53918489cfc7")
                .unwrap(),
        );

        let request = dummy_request();
        assert_eq!(expected, request.hash());
    }
}
