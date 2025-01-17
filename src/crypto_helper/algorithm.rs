use picky::hash::HashAlgorithm;
use picky::key::{PrivateKey, PublicKey};
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use crate::serde::*;

pub const MD5: &str = "MD5";
pub const SHA1: &str = "SHA1";
pub const SHA256: &str = "SHA256";
pub const SHA512: &str = "SHA512";
pub const AES128_CTS_HMAC_SHA1_96: &str = "AES128-CTS-HMAC-SHA1-96";
pub const AES256_CTS_HMAC_SHA1_96: &str = "AES256-CTS-HMAC-SHA1-96";
pub const HMAC_SHA1_96_AES128: &str = "HMAC-SHA1-96-AES128";
pub const HMAC_SHA1_96_AES256: &str = "HMAC-SHA1-96-AES256";
pub const RSA: &str = "RSA";
pub const SHA384: &str = "SHA384";
pub const BCRYPT: &str = "BCRYPT";
pub const ZLIB: &str = "ZLIB";

pub const SUPPORTED_ALGORITHMS: [&str; 12] = [
    MD5,
    SHA1,
    SHA256,
    SHA512,
    AES128_CTS_HMAC_SHA1_96,
    AES256_CTS_HMAC_SHA1_96,
    HMAC_SHA1_96_AES128,
    HMAC_SHA1_96_AES256,
    RSA,
    SHA384,
    BCRYPT,
    ZLIB,
];

pub const HASHING_ALGOS: [&str; 6] = [MD5, SHA1, SHA256, SHA384, SHA512, BCRYPT];

pub const ENCRYPTION_ALGOS: [&str; 3] = [AES128_CTS_HMAC_SHA1_96, AES256_CTS_HMAC_SHA1_96, RSA];

pub const HMAC_ALGOS: [&str; 2] = [HMAC_SHA1_96_AES128, HMAC_SHA1_96_AES256];

pub const COMPRESSION_ALGOS: [&str; 1] = [ZLIB];

const RSA_ACTIONS: [&str; 4] = ["Sign", "Verify", "Encrypt", "Decrypt"];

pub const RSA_HASH_MD5: &str = "MD5";
pub const RSA_HASH_SHA1: &str = "SHA1";
pub const RSA_HASH_SHA2_224: &str = "SHA2_224";
pub const RSA_HASH_SHA2_256: &str = "SHA2_256";
pub const RSA_HASH_SHA2_384: &str = "SHA2_384";
pub const RSA_HASH_SHA2_512: &str = "SHA2_512";
pub const RSA_HASH_SHA3_384: &str = "SHA3_384";
pub const RSA_HASH_SHA3_512: &str = "SHA3_512";

pub const RSA_HASH_ALGOS: [&str; 8] = [
    RSA_HASH_MD5,
    RSA_HASH_SHA1,
    RSA_HASH_SHA2_224,
    RSA_HASH_SHA2_256,
    RSA_HASH_SHA2_384,
    RSA_HASH_SHA2_512,
    RSA_HASH_SHA3_384,
    RSA_HASH_SHA3_512,
];

const DEFAULT_RSA_PRIVATE_KEY: &str = include_str!("../../public/assets/rsa_private_key.pem");
const DEFAULT_RSA_PUBLIC_KEY: &str = include_str!("../../public/assets/rsa_public_key.pem");

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct KrbInputData {
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub key: Vec<u8>,
    pub key_usage: i32,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub payload: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Serialize, Deserialize)]
pub enum KrbMode {
    #[default]
    Encrypt,
    Decrypt,
}

impl From<KrbMode> for bool {
    fn from(mode: KrbMode) -> Self {
        match mode {
            KrbMode::Encrypt => false,
            KrbMode::Decrypt => true,
        }
    }
}

impl From<bool> for KrbMode {
    fn from(mode: bool) -> Self {
        match mode {
            true => KrbMode::Decrypt,
            false => KrbMode::Encrypt,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct KrbInput {
    pub mode: KrbMode,
    pub data: KrbInputData,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct RsaHashAlgorithm(pub HashAlgorithm);

impl TryFrom<&str> for RsaHashAlgorithm {
    type Error = String;

    fn try_from(raw: &str) -> Result<Self, Self::Error> {
        if RSA_HASH_MD5 == raw {
            Ok(Self(HashAlgorithm::MD5))
        } else if RSA_HASH_SHA1 == raw {
            Ok(Self(HashAlgorithm::SHA1))
        } else if RSA_HASH_SHA2_224 == raw {
            Ok(Self(HashAlgorithm::SHA2_224))
        } else if RSA_HASH_SHA2_256 == raw {
            Ok(Self(HashAlgorithm::SHA2_256))
        } else if RSA_HASH_SHA2_384 == raw {
            Ok(Self(HashAlgorithm::SHA2_384))
        } else if RSA_HASH_SHA2_512 == raw {
            Ok(Self(HashAlgorithm::SHA2_512))
        } else if RSA_HASH_SHA3_384 == raw {
            Ok(Self(HashAlgorithm::SHA3_384))
        } else if RSA_HASH_SHA3_512 == raw {
            Ok(Self(HashAlgorithm::SHA3_512))
        } else {
            Err(format!(
                "Invalid RSA hash algorithm: {}. Supported: {:?}.",
                raw, RSA_HASH_ALGOS
            ))
        }
    }
}

impl From<&RsaHashAlgorithm> for &str {
    fn from(rsa_hash_algorithm: &RsaHashAlgorithm) -> Self {
        match &rsa_hash_algorithm.0 {
            HashAlgorithm::MD5 => RSA_HASH_ALGOS[0],
            HashAlgorithm::SHA1 => RSA_HASH_ALGOS[1],
            HashAlgorithm::SHA2_224 => RSA_HASH_ALGOS[2],
            HashAlgorithm::SHA2_256 => RSA_HASH_ALGOS[3],
            HashAlgorithm::SHA2_384 => RSA_HASH_ALGOS[4],
            HashAlgorithm::SHA2_512 => RSA_HASH_ALGOS[5],
            HashAlgorithm::SHA3_384 => RSA_HASH_ALGOS[6],
            HashAlgorithm::SHA3_512 => RSA_HASH_ALGOS[7],
            _ => "Other",
        }
    }
}

impl PartialEq<&str> for RsaHashAlgorithm {
    fn eq(&self, other: &&str) -> bool {
        let as_str: &str = self.into();

        as_str == *other
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RsaSignInput {
    pub hash_algorithm: RsaHashAlgorithm,
    #[serde(
        serialize_with = "serialize_private_key",
        deserialize_with = "deserialize_private_key"
    )]
    pub rsa_private_key: PrivateKey,
}

impl Default for RsaSignInput {
    fn default() -> Self {
        Self {
            hash_algorithm: RsaHashAlgorithm(HashAlgorithm::SHA1),
            rsa_private_key: PrivateKey::from_pem_str(DEFAULT_RSA_PRIVATE_KEY).unwrap(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct RsaVerifyInput {
    pub hash_algorithm: RsaHashAlgorithm,
    #[serde(serialize_with = "serialize_public_key", deserialize_with = "deserialize_public_key")]
    pub rsa_public_key: PublicKey,
    pub signature: Vec<u8>,
}

impl Default for RsaVerifyInput {
    fn default() -> Self {
        RsaVerifyInput {
            hash_algorithm: RsaHashAlgorithm(HashAlgorithm::SHA1),
            rsa_public_key: PublicKey::from_pem_str(DEFAULT_RSA_PUBLIC_KEY).unwrap(),
            signature: Vec::new(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum RsaAction {
    #[serde(
        serialize_with = "serialize_rsa_public_key",
        deserialize_with = "deserialize_rsa_public_key"
    )]
    Encrypt(RsaPublicKey),
    #[serde(
        serialize_with = "serialize_rsa_private_key",
        deserialize_with = "deserialize_rsa_private_key"
    )]
    Decrypt(RsaPrivateKey),
    Sign(RsaSignInput),
    Verify(RsaVerifyInput),
}

impl RsaAction {
    pub fn enumerate_actions() -> &'static [&'static str; 4] {
        &RSA_ACTIONS
    }
}

impl TryFrom<&str> for RsaAction {
    type Error = ();

    fn try_from(action_literal: &str) -> Result<Self, Self::Error> {
        if action_literal == RSA_ACTIONS[0] {
            Ok(RsaAction::Sign(Default::default()))
        } else if action_literal == RSA_ACTIONS[1] {
            Ok(RsaAction::Verify(Default::default()))
        } else if action_literal == RSA_ACTIONS[2] {
            Ok(RsaAction::Encrypt(
                RsaPublicKey::from_pkcs1_pem(DEFAULT_RSA_PUBLIC_KEY).unwrap(),
            ))
        } else if action_literal == RSA_ACTIONS[3] {
            Ok(RsaAction::Decrypt(
                RsaPrivateKey::from_pkcs1_pem(DEFAULT_RSA_PRIVATE_KEY).unwrap(),
            ))
        } else {
            Err(())
        }
    }
}

impl AsRef<str> for RsaAction {
    fn as_ref(&self) -> &str {
        match self {
            RsaAction::Encrypt(_) => "Encrypt",
            RsaAction::Decrypt(_) => "Decrypt",
            RsaAction::Sign(_) => "Sign",
            RsaAction::Verify(_) => "Verify",
        }
    }
}

impl PartialEq<&str> for RsaAction {
    fn eq(&self, other: &&str) -> bool {
        self.as_ref() == *other
    }
}

impl Default for RsaAction {
    fn default() -> Self {
        Self::Sign(Default::default())
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
pub struct RsaInput {
    pub action: RsaAction,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub payload: Vec<u8>,
}

impl Default for BcryptInput {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            action: BcryptAction::Hash(Default::default()),
        }
    }
}

impl Default for BcryptHashAction {
    fn default() -> Self {
        Self {
            rounds: 8,
            salt: Vec::new(),
        }
    }
}

#[derive(Eq, Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct BcryptHashAction {
    pub rounds: u32,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub salt: Vec<u8>,
}

#[derive(Eq, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum BcryptAction {
    Hash(BcryptHashAction),
    Verify(String),
}

impl From<&BcryptAction> for bool {
    fn from(action: &BcryptAction) -> Self {
        match action {
            BcryptAction::Hash(_) => false,
            BcryptAction::Verify(_) => true,
        }
    }
}

impl From<bool> for BcryptAction {
    fn from(action: bool) -> Self {
        match action {
            true => Self::Verify(Default::default()),
            false => Self::Hash(Default::default()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct BcryptInput {
    pub action: BcryptAction,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Default)]
pub enum ZlibMode {
    #[default]
    Compress,
    Decompress,
}

impl From<ZlibMode> for bool {
    fn from(mode: ZlibMode) -> Self {
        match mode {
            ZlibMode::Compress => false,
            ZlibMode::Decompress => true,
        }
    }
}

impl From<bool> for ZlibMode {
    fn from(mode: bool) -> Self {
        match mode {
            true => ZlibMode::Decompress,
            false => ZlibMode::Compress,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Default)]
pub struct ZlibInput {
    pub mode: ZlibMode,
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    pub data: Vec<u8>,
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum Algorithm {
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Md5(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha1(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha256(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha384(Vec<u8>),
    #[serde(serialize_with = "serialize_bytes", deserialize_with = "deserialize_bytes")]
    Sha512(Vec<u8>),
    Aes128CtsHmacSha196(KrbInput),
    Aes256CtsHmacSha196(KrbInput),
    HmacSha196Aes128(KrbInputData),
    HmacSha196Aes256(KrbInputData),
    Rsa(RsaInput),
    Bcrypt(BcryptInput),
    Zlib(ZlibInput),
}

impl TryFrom<&str> for Algorithm {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == MD5 {
            return Ok(Algorithm::Md5(Default::default()));
        } else if value == SHA1 {
            return Ok(Algorithm::Sha1(Default::default()));
        } else if value == SHA256 {
            return Ok(Algorithm::Sha256(Default::default()));
        } else if value == SHA384 {
            return Ok(Algorithm::Sha384(Default::default()));
        } else if value == SHA512 {
            return Ok(Algorithm::Sha512(Default::default()));
        } else if value == AES128_CTS_HMAC_SHA1_96 {
            return Ok(Algorithm::Aes128CtsHmacSha196(Default::default()));
        } else if value == AES256_CTS_HMAC_SHA1_96 {
            return Ok(Algorithm::Aes256CtsHmacSha196(Default::default()));
        } else if value == HMAC_SHA1_96_AES128 {
            return Ok(Algorithm::HmacSha196Aes128(Default::default()));
        } else if value == HMAC_SHA1_96_AES256 {
            return Ok(Algorithm::HmacSha196Aes256(Default::default()));
        } else if value == RSA {
            return Ok(Algorithm::Rsa(Default::default()));
        } else if value == BCRYPT {
            return Ok(Algorithm::Bcrypt(Default::default()));
        } else if value == ZLIB {
            return Ok(Algorithm::Zlib(Default::default()));
        }

        Err(format!(
            "Invalid algorithm name: {}. Supported: {:?}.",
            value, SUPPORTED_ALGORITHMS
        ))
    }
}

impl From<&Algorithm> for &str {
    fn from(algorithm: &Algorithm) -> Self {
        match algorithm {
            Algorithm::Md5(_) => MD5,
            Algorithm::Sha1(_) => SHA1,
            Algorithm::Sha256(_) => SHA256,
            Algorithm::Sha384(_) => SHA384,
            Algorithm::Sha512(_) => SHA512,
            Algorithm::Aes128CtsHmacSha196(_) => AES128_CTS_HMAC_SHA1_96,
            Algorithm::Aes256CtsHmacSha196(_) => AES256_CTS_HMAC_SHA1_96,
            Algorithm::HmacSha196Aes128(_) => HMAC_SHA1_96_AES128,
            Algorithm::HmacSha196Aes256(_) => HMAC_SHA1_96_AES256,
            Algorithm::Rsa(_) => RSA,
            Algorithm::Bcrypt(_) => BCRYPT,
            Algorithm::Zlib(_) => ZLIB,
        }
    }
}

impl PartialEq<&str> for &Algorithm {
    fn eq(&self, other: &&str) -> bool {
        let as_str: &str = (*self).into();

        as_str == *other
    }
}

impl Default for Algorithm {
    fn default() -> Self {
        Algorithm::Zlib(Default::default())
    }
}
