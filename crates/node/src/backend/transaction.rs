use alloy::{
    consensus::{
        transaction::{eip4844::TxEip4844Variant, TxEip7702},
        Signed, TxEip1559, TxEip2930, TxEnvelope, TxLegacy, Typed2718,
    },
    eips::eip2718::{Decodable2718, Eip2718Error, Encodable2718},
    primitives::{
        Address, Bloom, Bytes, Log, PrimitiveSignature, SignatureError, TxHash, TxKind, B256, U256,
        U64,
    },
    rlp::{self, Decodable, Encodable, Header},
    rpc::types::AccessList,
};
use bytes::BufMut;
use serde::{Deserialize, Serialize};
use std::ops::Mul;

/// Container type for signed, typed transactions.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum TypedTransaction {
    /// Legacy transaction type
    Legacy(Signed<TxLegacy>),
    /// EIP-2930 transaction
    EIP2930(Signed<TxEip2930>),
    /// EIP-1559 transaction
    EIP1559(Signed<TxEip1559>),
    /// EIP-4844 transaction
    EIP4844(Signed<TxEip4844Variant>),
    /// EIP-7702 transaction
    EIP7702(Signed<TxEip7702>),
}

impl TypedTransaction {
    /// Returns true if the transaction uses dynamic fees: EIP1559, EIP4844 or EIP7702
    pub fn is_dynamic_fee(&self) -> bool {
        matches!(self, Self::EIP1559(_) | Self::EIP4844(_) | Self::EIP7702(_))
    }

    pub fn gas_price(&self) -> u128 {
        match self {
            Self::Legacy(tx) => tx.tx().gas_price,
            Self::EIP2930(tx) => tx.tx().gas_price,
            Self::EIP1559(tx) => tx.tx().max_fee_per_gas,
            Self::EIP4844(tx) => tx.tx().tx().max_fee_per_gas,
            Self::EIP7702(tx) => tx.tx().max_fee_per_gas,
        }
    }

    pub fn gas_limit(&self) -> u64 {
        match self {
            Self::Legacy(tx) => tx.tx().gas_limit,
            Self::EIP2930(tx) => tx.tx().gas_limit,
            Self::EIP1559(tx) => tx.tx().gas_limit,
            Self::EIP4844(tx) => tx.tx().tx().gas_limit,
            Self::EIP7702(tx) => tx.tx().gas_limit,
        }
    }

    pub fn value(&self) -> U256 {
        U256::from(match self {
            Self::Legacy(tx) => tx.tx().value,
            Self::EIP2930(tx) => tx.tx().value,
            Self::EIP1559(tx) => tx.tx().value,
            Self::EIP4844(tx) => tx.tx().tx().value,
            Self::EIP7702(tx) => tx.tx().value,
        })
    }

    pub fn data(&self) -> &Bytes {
        match self {
            Self::Legacy(tx) => &tx.tx().input,
            Self::EIP2930(tx) => &tx.tx().input,
            Self::EIP1559(tx) => &tx.tx().input,
            Self::EIP4844(tx) => &tx.tx().tx().input,
            Self::EIP7702(tx) => &tx.tx().input,
        }
    }

    /// Returns the transaction type
    pub fn r#type(&self) -> Option<u8> {
        match self {
            Self::Legacy(_) => None,
            Self::EIP2930(_) => Some(1),
            Self::EIP1559(_) => Some(2),
            Self::EIP4844(_) => Some(3),
            Self::EIP7702(_) => Some(4),
        }
    }

    /// Max cost of the transaction
    /// It is the gas limit multiplied by the gas price,
    /// and if the transaction is EIP-4844, the result of (total blob gas cost * max fee per blob
    /// gas) is also added
    pub fn max_cost(&self) -> u128 {
        let mut max_cost = (self.gas_limit() as u128).saturating_mul(self.gas_price());

        if self.is_eip4844() {
            max_cost = max_cost.saturating_add(
                self.blob_gas()
                    .map(|g| g as u128)
                    .unwrap_or(0)
                    .mul(self.max_fee_per_blob_gas().unwrap_or(0)),
            )
        }

        max_cost
    }

    pub fn blob_gas(&self) -> Option<u64> {
        match self {
            Self::EIP4844(tx) => Some(tx.tx().tx().blob_gas()),
            _ => None,
        }
    }

    pub fn max_fee_per_blob_gas(&self) -> Option<u128> {
        match self {
            Self::EIP4844(tx) => Some(tx.tx().tx().max_fee_per_blob_gas),
            _ => None,
        }
    }

    /// Returns a helper type that contains commonly used values as fields
    pub fn essentials(&self) -> TransactionEssentials {
        match self {
            Self::Legacy(t) => TransactionEssentials {
                kind: t.tx().to,
                input: t.tx().input.clone(),
                nonce: t.tx().nonce,
                gas_limit: t.tx().gas_limit,
                gas_price: Some(t.tx().gas_price),
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: None,
                value: t.tx().value,
                chain_id: t.tx().chain_id,
                access_list: Default::default(),
            },
            Self::EIP2930(t) => TransactionEssentials {
                kind: t.tx().to,
                input: t.tx().input.clone(),
                nonce: t.tx().nonce,
                gas_limit: t.tx().gas_limit,
                gas_price: Some(t.tx().gas_price),
                max_fee_per_gas: None,
                max_priority_fee_per_gas: None,
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: None,
                value: t.tx().value,
                chain_id: Some(t.tx().chain_id),
                access_list: t.tx().access_list.clone(),
            },
            Self::EIP1559(t) => TransactionEssentials {
                kind: t.tx().to,
                input: t.tx().input.clone(),
                nonce: t.tx().nonce,
                gas_limit: t.tx().gas_limit,
                gas_price: None,
                max_fee_per_gas: Some(t.tx().max_fee_per_gas),
                max_priority_fee_per_gas: Some(t.tx().max_priority_fee_per_gas),
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: None,
                value: t.tx().value,
                chain_id: Some(t.tx().chain_id),
                access_list: t.tx().access_list.clone(),
            },
            Self::EIP4844(t) => TransactionEssentials {
                kind: TxKind::Call(t.tx().tx().to),
                input: t.tx().tx().input.clone(),
                nonce: t.tx().tx().nonce,
                gas_limit: t.tx().tx().gas_limit,
                gas_price: Some(t.tx().tx().max_fee_per_blob_gas),
                max_fee_per_gas: Some(t.tx().tx().max_fee_per_gas),
                max_priority_fee_per_gas: Some(t.tx().tx().max_priority_fee_per_gas),
                max_fee_per_blob_gas: Some(t.tx().tx().max_fee_per_blob_gas),
                blob_versioned_hashes: Some(t.tx().tx().blob_versioned_hashes.clone()),
                value: t.tx().tx().value,
                chain_id: Some(t.tx().tx().chain_id),
                access_list: t.tx().tx().access_list.clone(),
            },
            Self::EIP7702(t) => TransactionEssentials {
                kind: TxKind::Call(t.tx().to),
                input: t.tx().input.clone(),
                nonce: t.tx().nonce,
                gas_limit: t.tx().gas_limit,
                gas_price: Some(t.tx().max_fee_per_gas),
                max_fee_per_gas: Some(t.tx().max_fee_per_gas),
                max_priority_fee_per_gas: Some(t.tx().max_priority_fee_per_gas),
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: None,
                value: t.tx().value,
                chain_id: Some(t.tx().chain_id),
                access_list: t.tx().access_list.clone(),
            },
        }
    }

    pub fn nonce(&self) -> u64 {
        match self {
            Self::Legacy(t) => t.tx().nonce,
            Self::EIP2930(t) => t.tx().nonce,
            Self::EIP1559(t) => t.tx().nonce,
            Self::EIP4844(t) => t.tx().tx().nonce,
            Self::EIP7702(t) => t.tx().nonce,
        }
    }

    pub fn chain_id(&self) -> Option<u64> {
        match self {
            Self::Legacy(t) => t.tx().chain_id,
            Self::EIP2930(t) => Some(t.tx().chain_id),
            Self::EIP1559(t) => Some(t.tx().chain_id),
            Self::EIP4844(t) => Some(t.tx().tx().chain_id),
            Self::EIP7702(t) => Some(t.tx().chain_id),
        }
    }

    pub fn as_legacy(&self) -> Option<&Signed<TxLegacy>> {
        match self {
            Self::Legacy(tx) => Some(tx),
            _ => None,
        }
    }

    /// Returns true whether this tx is a legacy transaction
    pub fn is_legacy(&self) -> bool {
        matches!(self, Self::Legacy(_))
    }

    /// Returns true whether this tx is a EIP1559 transaction
    pub fn is_eip1559(&self) -> bool {
        matches!(self, Self::EIP1559(_))
    }

    /// Returns true whether this tx is a EIP2930 transaction
    pub fn is_eip2930(&self) -> bool {
        matches!(self, Self::EIP2930(_))
    }

    /// Returns true whether this tx is a EIP4844 transaction
    pub fn is_eip4844(&self) -> bool {
        matches!(self, Self::EIP4844(_))
    }

    /// Returns the hash of the transaction.
    pub fn hash(&self) -> B256 {
        match self {
            Self::Legacy(t) => *t.hash(),
            Self::EIP2930(t) => *t.hash(),
            Self::EIP1559(t) => *t.hash(),
            Self::EIP4844(t) => *t.hash(),
            Self::EIP7702(t) => *t.hash(),
        }
    }

    /// Recovers the Ethereum address which was used to sign the transaction.
    pub fn recover(&self) -> Result<Address, SignatureError> {
        match self {
            Self::Legacy(tx) => tx.recover_signer(),
            Self::EIP2930(tx) => tx.recover_signer(),
            Self::EIP1559(tx) => tx.recover_signer(),
            Self::EIP4844(tx) => tx.recover_signer(),
            Self::EIP7702(tx) => tx.recover_signer(),
        }
    }

    /// Returns what kind of transaction this is
    pub fn kind(&self) -> TxKind {
        match self {
            Self::Legacy(tx) => tx.tx().to,
            Self::EIP2930(tx) => tx.tx().to,
            Self::EIP1559(tx) => tx.tx().to,
            Self::EIP4844(tx) => TxKind::Call(tx.tx().tx().to),
            Self::EIP7702(tx) => TxKind::Call(tx.tx().to),
        }
    }

    /// Returns the callee if this transaction is a call
    pub fn to(&self) -> Option<Address> {
        self.kind().to().copied()
    }

    /// Returns the Signature of the transaction
    pub fn signature(&self) -> PrimitiveSignature {
        match self {
            Self::Legacy(tx) => *tx.signature(),
            Self::EIP2930(tx) => *tx.signature(),
            Self::EIP1559(tx) => *tx.signature(),
            Self::EIP4844(tx) => *tx.signature(),
            Self::EIP7702(tx) => *tx.signature(),
        }
    }
}

impl Encodable for TypedTransaction {
    fn encode(&self, out: &mut dyn bytes::BufMut) {
        if !self.is_legacy() {
            Header {
                list: false,
                payload_length: self.encode_2718_len(),
            }
            .encode(out);
        }

        self.encode_2718(out);
    }
}

impl Decodable for TypedTransaction {
    fn decode(buf: &mut &[u8]) -> rlp::Result<Self> {
        let mut h_decode_copy = *buf;
        let header = rlp::Header::decode(&mut h_decode_copy)?;

        // Legacy TX
        if header.list {
            return Ok(TxEnvelope::decode(buf)?.into());
        }

        Ok(TxEnvelope::decode(buf)?.into())
    }
}

impl Typed2718 for TypedTransaction {
    fn ty(&self) -> u8 {
        match self {
            Self::Legacy(tx) => tx.ty(),
            Self::EIP2930(tx) => tx.ty(),
            Self::EIP1559(tx) => tx.ty(),
            Self::EIP4844(tx) => tx.ty(),
            Self::EIP7702(tx) => tx.ty(),
        }
    }
}

impl Decodable2718 for TypedTransaction {
    fn typed_decode(ty: u8, buf: &mut &[u8]) -> Result<Self, Eip2718Error> {
        match TxEnvelope::typed_decode(ty, buf)? {
            TxEnvelope::Eip2930(tx) => Ok(Self::EIP2930(tx)),
            TxEnvelope::Eip1559(tx) => Ok(Self::EIP1559(tx)),
            TxEnvelope::Eip4844(tx) => Ok(Self::EIP4844(tx)),
            TxEnvelope::Eip7702(tx) => Ok(Self::EIP7702(tx)),
            _ => unreachable!(),
        }
    }

    fn fallback_decode(buf: &mut &[u8]) -> Result<Self, Eip2718Error> {
        match TxEnvelope::fallback_decode(buf)? {
            TxEnvelope::Legacy(tx) => Ok(Self::Legacy(tx)),
            _ => unreachable!(),
        }
    }
}

impl Encodable2718 for TypedTransaction {
    fn type_flag(&self) -> Option<u8> {
        self.r#type()
    }

    fn encode_2718_len(&self) -> usize {
        match self {
            Self::Legacy(tx) => TxEnvelope::from(tx.clone()).encode_2718_len(),
            Self::EIP2930(tx) => TxEnvelope::from(tx.clone()).encode_2718_len(),
            Self::EIP1559(tx) => TxEnvelope::from(tx.clone()).encode_2718_len(),
            Self::EIP4844(tx) => TxEnvelope::from(tx.clone()).encode_2718_len(),
            Self::EIP7702(tx) => TxEnvelope::from(tx.clone()).encode_2718_len(),
        }
    }

    fn encode_2718(&self, out: &mut dyn BufMut) {
        match self {
            Self::Legacy(tx) => TxEnvelope::from(tx.clone()).encode_2718(out),
            Self::EIP2930(tx) => TxEnvelope::from(tx.clone()).encode_2718(out),
            Self::EIP1559(tx) => TxEnvelope::from(tx.clone()).encode_2718(out),
            Self::EIP4844(tx) => TxEnvelope::from(tx.clone()).encode_2718(out),
            Self::EIP7702(tx) => TxEnvelope::from(tx.clone()).encode_2718(out),
        }
    }
}

impl From<TxEnvelope> for TypedTransaction {
    fn from(value: TxEnvelope) -> Self {
        match value {
            TxEnvelope::Legacy(tx) => Self::Legacy(tx),
            TxEnvelope::Eip2930(tx) => Self::EIP2930(tx),
            TxEnvelope::Eip1559(tx) => Self::EIP1559(tx),
            TxEnvelope::Eip4844(tx) => Self::EIP4844(tx),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TransactionEssentials {
    pub kind: TxKind,
    pub input: Bytes,
    pub nonce: u64,
    pub gas_limit: u64,
    pub gas_price: Option<u128>,
    pub max_fee_per_gas: Option<u128>,
    pub max_priority_fee_per_gas: Option<u128>,
    pub max_fee_per_blob_gas: Option<u128>,
    pub blob_versioned_hashes: Option<Vec<B256>>,
    pub value: U256,
    pub chain_id: Option<u64>,
    pub access_list: AccessList,
}
