// Copyright (c) 2026 Emirhan CAMCI. All rights reserved.

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use std::sync::Arc;

pub struct EncryptedVfs {
    cipher: Arc<XChaCha20Poly1305>,
}

pub struct EncryptedFile {
    cipher: Arc<XChaCha20Poly1305>,
}

impl EncryptedVfs {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = XChaCha20Poly1305::new(key.into());
        Self {
            cipher: Arc::new(cipher),
        }
    }
}

fn derive_nonce(offset: u64) -> XNonce {
    let mut nonce_bytes = [0u8; 24];
    nonce_bytes[0..8].copy_from_slice(&offset.to_le_bytes());
    *XNonce::from_slice(&nonce_bytes)
}

impl EncryptedFile {
    pub fn read(&mut self, buf: &mut [u8], offset: u64, raw_data: &[u8]) -> Result<usize, &'static str> {
        let nonce = derive_nonce(offset);
        let decrypted = self.cipher
            .decrypt(&nonce, raw_data)
            .map_err(|_| "Decryption failed")?;
            
        buf[..decrypted.len()].copy_from_slice(&decrypted);
        Ok(decrypted.len())
    }

    pub fn write(&mut self, buf: &[u8], offset: u64) -> Result<Vec<u8>, &'static str> {
        let nonce = derive_nonce(offset);
        let encrypted = self.cipher
            .encrypt(&nonce, buf)
            .map_err(|_| "Encryption failed")?;
            
        Ok(encrypted)
    }
}
