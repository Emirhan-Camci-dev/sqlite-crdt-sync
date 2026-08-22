// Copyright (c) 2026 Emirhan CAMCI. All rights reserved.

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305, XNonce,
};
use rusqlite::vfs::{File, OpenOptions};
use std::sync::Arc;

pub struct EncryptedVfs {
    cipher: Arc<XChaCha20Poly1305>,
}

pub struct EncryptedFile {
    inner: Box<dyn File>,
    cipher: Arc<XChaCha20Poly1305>,
}

impl EncryptedVfs {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = XChaCha20Poly1305::new(key.into());
        Self {
            cipher: Arc::new(cipher),
        }
    }
    
    // In a full implementation, you would register this struct as a rusqlite Vfs 
    // and implement the `Vfs` trait methods to open/delete files.
}

fn derive_nonce(offset: u64) -> XNonce {
    // A simplistic derivation for demonstration. 
    // Secure implementations often hash the offset with a file-specific salt.
    let mut nonce_bytes = [0u8; 24];
    nonce_bytes[0..8].copy_from_slice(&offset.to_le_bytes());
    *XNonce::from_slice(&nonce_bytes)
}

impl File for EncryptedFile {
    fn read(&mut self, buf: &mut [u8], offset: u64) -> rusqlite::Result<usize> {
        let mut encrypted_buf = vec![0u8; buf.len()];
        let bytes_read = self.inner.read(&mut encrypted_buf, offset)?;
        
        if bytes_read == 0 {
            return Ok(0);
        }

        let nonce = derive_nonce(offset);
        
        let decrypted = self.cipher
            .decrypt(&nonce, &encrypted_buf[..bytes_read])
            .map_err(|_| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR_READ),
                Some("Decryption failed".to_string()),
            ))?;
            
        buf[..decrypted.len()].copy_from_slice(&decrypted);
        Ok(decrypted.len())
    }

    fn write(&mut self, buf: &[u8], offset: u64) -> rusqlite::Result<usize> {
        let nonce = derive_nonce(offset);
        let encrypted = self.cipher
            .encrypt(&nonce, buf)
            .map_err(|_| rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR_WRITE),
                Some("Encryption failed".to_string()),
            ))?;
            
        self.inner.write(&encrypted, offset)
    }

    fn file_size(&self) -> rusqlite::Result<u64> {
        self.inner.file_size()
    }

    fn sync(&mut self) -> rusqlite::Result<()> {
        self.inner.sync()
    }
}
