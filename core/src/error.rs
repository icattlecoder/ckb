use bigint::{H256, H512, U256};
use crypto::secp::Error as CrypError;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidTimestamp(u64, u64),
    InvalidTransactionsRoot(H256, H256),
    InvalidPublicKey(H512),
    InvalidProof,
    InvalidDifficulty(U256, U256),
    InvalidSignature(CrypError),
    InvalidHash(H256, H256),
}

impl From<CrypError> for Error {
    fn from(e: CrypError) -> Self {
        Error::InvalidSignature(e)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TxError {
    OutofBound,
    NotMatch,
    EmptyGroup,
    WrongFormat,
}