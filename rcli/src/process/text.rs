use crate::{process_genpass, TextSignFormat};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::collections::HashMap;
use std::io::Read;


pub trait TextSigner {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>>;
}

pub trait TextVerifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Blake3Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSigner for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes().to_vec())
    }
}

impl TextSigner for Blake3Signer {
    fn sign(&self, reader: &mut dyn Read) -> anyhow::Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let signature = self.key.sign(&buf);
        Ok(signature.to_bytes().to_vec())
    }
}
impl TextVerifier for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let ret = blake3::keyed_hash(&self.key, &buf);
        Ok(ret.as_bytes() == sig)
    }
}

impl TextVerifier for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = ed25519_dalek::Signature::from_bytes(&sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}

impl TextVerifier for Blake3Signer {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> anyhow::Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = (&sig[..64]).try_into()?;
        let signature = Signature::from_bytes(sig);
        Ok(self.key.verify(&buf, &signature).is_ok())
    }
}


impl Blake3 {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = (&key[0..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: [u8; 32]) -> Self {
        Self {
            key
        }
    }

    fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let mut map = HashMap::new();
        map.insert("blake3.txt", key.as_bytes().to_vec());
        Ok(map)
    }
}


impl Blake3Signer {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = (&key[0..32]).try_into()?;
        Ok(Self::new(key))
    }

    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    fn generate() -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
        let mut csprng = OsRng;
        let sk: SigningKey = SigningKey::generate(&mut csprng);
        let pk: VerifyingKey = (&sk).try_into()?;

        let mut map = HashMap::new();
        map.insert("ed25119.sk", sk.to_bytes().to_vec());
        map.insert("ed2511c.pk", pk.to_bytes().to_vec());

        Ok(map)
    }
}

impl Ed25519Verifier {
    pub fn try_new(key: impl AsRef<[u8]>) -> anyhow::Result<Self> {
        let key = key.as_ref();
        let key = (&key[0..32]).try_into()?;
        let key = VerifyingKey::from_bytes(key)?;
        Ok(Self { key })
    }
}

pub fn process_text_sign(reader: &mut dyn Read, key: &[u8], format: TextSignFormat) -> anyhow::Result<Vec<u8>> {
    let signer: Box<dyn TextSigner> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Blake3Signer::try_new(key)?),
    };

    signer.sign(reader)
}

pub fn process_text_verifier(reader: &mut dyn Read, key: &[u8], sig: &[u8], format: TextSignFormat) -> anyhow::Result<bool> {
    let verify: Box<dyn TextVerifier> = match format {
        TextSignFormat::Blake3 => Box::new(Blake3::try_new(key)?),
        TextSignFormat::Ed25519 => Box::new(Ed25519Verifier::try_new(key)?),
    };
    verify.verify(reader, sig)
}

pub fn process_text_key_generate(format: TextSignFormat) -> anyhow::Result<HashMap<&'static str, Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Blake3Signer::generate(),
    }
}


#[cfg(test)]
mod tests {
    use base64::Engine;
    use base64::prelude::BASE64_URL_SAFE_NO_PAD;
    use super::*;
    const KEY: &[u8] = include_bytes!("../../fixtures/blake3.txt");
    #[test]
    fn test_process_text_sign() -> anyhow::Result<()> {
        let mut reader = "hello".as_bytes();
        let mut reader1 = "hello".as_bytes();

        let format = TextSignFormat::Blake3;

        let sig = process_text_sign(&mut reader, KEY, format)?;
        let ret = process_text_verifier(&mut reader1, KEY, &sig, format)?;
        assert!(ret);
        Ok(())
    }

    #[test]
    fn test_process_text_verifier() -> anyhow::Result<()> {
        let mut reader = "hello".as_bytes();
        let format = TextSignFormat::Blake3;
        let sig = "33Ypo4rveYpWmJKAiGnnse-wHQhMVujjmcVkV4Tl43k";
        let sig = BASE64_URL_SAFE_NO_PAD.decode(sig)?;

        let ret = process_text_verifier(&mut reader,KEY,&sig, format)?;

        assert!(ret);
        Ok(())
    }
}


