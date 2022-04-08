use serde_json::Value;
use std::fmt::{Display, Formatter};

/// Draft block structure
#[derive(Debug)]
pub struct Block {
    header: Header,
    txs: Vec<Transaction>,
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum BlockConstructionError {
    /// When `node_id` field could not be found in the supplied JSON block
    MissingNodeId,
    /// When `node_maintainer` field could not be found in the supplied JSON block
    MissingNodeMaintainer,
    /// When the json block provided is not a valid JSON
    InvalidBlockStructure,
    /// When the transaction JSON is invalid and could not be parsed
    InvalidTxStructure,
    /// When the JSON value cannot be converted to the expected type
    ConversionError,
}

impl Display for BlockConstructionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Block {
    pub fn from_json(json: serde_json::Value) -> Result<Self, BlockConstructionError> {
        if let serde_json::Value::Object(mut json) = json {
            let node_id = json
                .get("node_id")
                .ok_or(BlockConstructionError::MissingNodeId)?
                .as_u64()
                .ok_or(BlockConstructionError::ConversionError)? as u16;
            let node_maintainer_account_id = json
                .get("node_maintainer")
                .ok_or(BlockConstructionError::MissingNodeMaintainer)?
                .as_str()
                .ok_or(BlockConstructionError::ConversionError)?
                .to_string();
            let header = Header::new(node_id, node_maintainer_account_id);
            let txs: Vec<Transaction> =
                if let Some(serde_json::Value::Array(transactions)) = json.remove("txs") {
                    transactions
                        .into_iter()
                        .filter_map(|t| t.try_into().ok())
                        .collect::<_>()
                } else {
                    Vec::new()
                };
            Ok(Block { header, txs })
        } else {
            Err(BlockConstructionError::InvalidBlockStructure)
        }
    }

    pub fn txs(&self) -> &Vec<Transaction> {
        &self.txs
    }
}

#[derive(Debug)]
pub struct Header {
    node_id: u16,
    node_maintainer: String, // for now, store node_maintainer_account_id as ss58-encoded address
}

impl Header {
    pub fn new(node_id: u16, node_maintainer: String) -> Self {
        Self {
            node_id,
            node_maintainer,
        }
    }
}

#[derive(Debug)]
pub struct Transaction {
    pub author_node_id: u16,
    pub author_node_maintainer: String,
    pub recipient_node_id: u16,
    pub recipient_node_maintainer: String,
    pub tx_proof: String, // 0x-prefixed hex repr of a 256-bit hash,
    pub value: u128,
}

impl TryFrom<serde_json::Value> for Transaction {
    type Error = BlockConstructionError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let maybe_tx: fn(Value) -> Option<Transaction> = move |value| {
            if let serde_json::Value::Object(tx) = value {
                let (
                    author_node_id,
                    author_node_maintainer,
                    recipient_node_id,
                    recipient_node_maintainer,
                    tx_proof,
                    value,
                ) = (
                    tx.get("author_node_id")?,
                    tx.get("author_node_maintainer")?,
                    tx.get("recipient_node_id")?,
                    tx.get("recipient_node_maintainer")?,
                    tx.get("tx_proof")?,
                    tx.get("value")?,
                );

                Some(Transaction {
                    author_node_id: author_node_id.as_u64()? as u16,
                    author_node_maintainer: author_node_maintainer.as_str()?.to_string(),
                    recipient_node_id: recipient_node_id.as_u64()? as u16,
                    recipient_node_maintainer: recipient_node_maintainer.as_str()?.to_string(),
                    tx_proof: tx_proof.as_str()?.to_string(),
                    value: u128::from_le_bytes(
                        (hex::decode(&value.as_str()?[2..]).ok()?).try_into().ok()?,
                    ),
                    // value: 0,
                })
            } else {
                None
            }
        };

        maybe_tx(value).ok_or(BlockConstructionError::InvalidTxStructure)
    }
}

#[cfg(test)]
mod tests {
    use crate::block::Block;
    use serde_json::json;

    #[test]
    fn valid_block_should_be_ok() {
        #[allow(overflowing_literals)]
        let block_json: serde_json::Value = json!(
            {
                "node_id": 1,
                "node_maintainer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                "txs": [{
                    "author_node_id": 1,
                    "author_node_maintainer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    "recipient_node_id": 2,
                    "recipient_node_maintainer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                    "tx_proof": "0x0000000000000000000000000000000000000000000000000000000000000000", // 0x-prefixed hex repr of a 256-bit hash,
                    "value": "0x0000a0dec5adc9353600000000000000"
            }]
            }
        );
        let parse_result = Block::from_json(block_json);
        assert!(parse_result.is_ok());
        println!("Block: {:?}", parse_result.unwrap())
    }

    #[test]
    fn should_filter_malformed_transactions() {
        #[allow(overflowing_literals)]
        // missing recipient_node_id in tx
        let block_json: serde_json::Value = json!(
            {
                "node_id": 1,
                "node_maintainer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                "txs": [{
                    "author_node_id": 1,
                    "author_node_maintainer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    "recipient_node_maintainer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                    "tx_proof": "0x0000000000000000000000000000000000000000000000000000000000000000", // 0x-prefixed hex repr of a 256-bit hash,
                    "value": "0x0000a0dec5adc9353600000000000000"
            }]
            }
        );
        let parse_result = Block::from_json(block_json);
        assert!(parse_result.is_ok());
        assert!(parse_result.unwrap().txs.is_empty());
    }

    #[test]
    fn should_be_err_if_missing_header_values() {
        #[allow(overflowing_literals)]
        // missing node_maintainer
        let block_json: serde_json::Value = json!(
            {
                "node_id": 1,
                "txs": [{
                    "author_node_id": 1,
                    "author_node_maintainer": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY",
                    "recipient_node_id": 2,
                    "recipient_node_maintainer": "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty",
                    "tx_proof": "0x0000000000000000000000000000000000000000000000000000000000000000", // 0x-prefixed hex repr of a 256-bit hash,
                    "value": "0x0000a0dec5adc9353600000000000000",
            }]
            }
        );
        let parse_result = Block::from_json(block_json);
        assert!(parse_result.is_err());
    }
}
