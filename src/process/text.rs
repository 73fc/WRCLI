use crate::cli::text::*;
use crate::process_genpass;
use crate::utils::read_data;
use anyhow::{Ok, Result};
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use base64::Engine;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::io::Read;
use std::path::Path;
use std::{fs, vec};

trait TextSign {
    /// Sign the data from the reader and return the signature
    fn sign(&self, reader: &mut impl Read) -> Result<Vec<u8>>;
}

trait TextVerify {
    /// Verify the data from the reader with the signature
    fn verify(&self, reader: &mut impl Read, sign: &[u8]) -> Result<bool>;
}

pub trait KeyGenerator {
    fn generate_key() -> Result<Vec<Vec<u8>>>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub struct Blake3 {
    key: [u8; 32],
}
pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut impl Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sign)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut impl Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}
impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut impl Read, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sign.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(key)
    }
}

impl KeyGenerator for Blake3 {
    fn generate_key() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.trim().as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate_key() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.as_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = key[..32].try_into()?;
        Ok(Self::new(key))
    }
}

impl Ed25519Signer {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = SigningKey::from_bytes(&key.clone());
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}

impl Ed25519Verifier {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = VerifyingKey::from_bytes(&key.clone()).unwrap();
        Self { key }
    }

    pub fn try_new(key: impl AsRef<[u8]>) -> Result<Self> {
        let key = key.as_ref();
        let key = (&key[..32]).try_into()?;
        Ok(Self::new(key))
    }
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> anyhow::Result<String> {
    let mut reader = read_data(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    println!("sign:{:?}", signed);
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn process_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sign: &str,
) -> anyhow::Result<bool> {
    let mut reader = read_data(input)?;
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let verify = Blake3::load(key)?;
            verify.verify(&mut reader, &sign[..32])?
        }
        TextSignFormat::Ed25519 => {
            let verify = Ed25519Verifier::load(key)?;
            verify.verify(&mut reader, &sign[..32])?
        }
    };
    Ok(signed)
}

pub fn process_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate_key(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate_key(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    //use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
    const KEY: &str = include_str!("../../fixtures/blake3.txt");

    #[test]
    fn test_process_sign() -> Result<()> {
        let data = b"hello world!";
        let signer = Blake3::try_new(KEY)?;
        let sign = signer.sign(&mut &data[..]);
        assert!(sign.is_ok());
        Ok(())
    }

    #[test]
    fn text_process_text_verify() -> Result<()> {
        let data = b"hello world!";
        let signer = Blake3::try_new(KEY)?;
        let sign = signer.sign(&mut &data[..])?;
        let ret = signer.verify(&mut &data[..], &sign);
        assert!(ret.is_ok());
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/Ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/Ed25519.pk")?;
        let data = b"hello world!";
        let sig = sk.sign(&mut &data[..])?;
        assert!(pk.verify(&mut &data[..], &sig).is_ok());
        Ok(())
    }
}
