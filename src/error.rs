#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Generic(String),

    #[error("could not parse SocketAddr `{0}`")]
    AddrParse(String),

    #[error("Poison {0}")]
    Poison(String),

    #[error("Aes {0}")]
    Aes(String),

    #[error(transparent)]
    InvalidMnemonic(#[from] bip39::Error),

    #[error(transparent)]
    Bitcoin(#[from] elements::bitcoin::Error),

    #[error(transparent)]
    BitcoinHashes(#[from] elements::bitcoin::hashes::error::Error),

    #[error(transparent)]
    BitcoinBIP32Error(#[from] elements::bitcoin::bip32::Error),

    #[error(transparent)]
    BitcoinConsensus(#[from] elements::bitcoin::consensus::encode::Error),

    #[error(transparent)]
    JsonFrom(#[from] serde_json::Error),

    #[error(transparent)]
    StdIOError(#[from] std::io::Error),

    #[error(transparent)]
    Hex(#[from] hex::FromHexError),

    #[error(transparent)]
    ClientError(#[from] electrum_client::Error),

    #[error(transparent)]
    SliceConversionError(#[from] std::array::TryFromSliceError),

    #[error(transparent)]
    ElementsEncode(#[from] elements::encode::Error),

    #[error(transparent)]
    ElementsPset(#[from] elements::pset::Error),

    #[error(transparent)]
    PsetBlindError(#[from] elements::pset::PsetBlindError),

    #[error(transparent)]
    Send(#[from] std::sync::mpsc::SendError<()>),

    #[error(transparent)]
    Secp256k1(#[from] elements::bitcoin::secp256k1::Error),

    #[error(transparent)]
    Secp256k1Zkp(#[from] elements::secp256k1_zkp::Error),

    #[error(transparent)]
    HexBitcoinHashes(#[from] elements::bitcoin::hashes::hex::Error),

    #[error(transparent)]
    SerdeCbor(#[from] serde_cbor::Error),
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

// cannot derive automatically with this error because of lifetime
impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Self::Poison(err.to_string())
    }
}

// cannot derive automatically with this error because of trait bound
impl From<aes_gcm_siv::aead::Error> for Error {
    fn from(err: aes_gcm_siv::aead::Error) -> Self {
        Self::Aes(err.to_string())
    }
}
