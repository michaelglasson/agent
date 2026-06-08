use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};

// --- ENCRYPT ---
pub fn quick_encrypt(plaintext: &str, password: &str) -> Vec<u8> {
    // Force the password into a 32-byte key (padded with 0s if short)
    let mut key_bytes = [0u8; 32];
    let pwd_bytes = password.as_bytes();
    let amt = pwd_bytes.len().min(32);
    key_bytes[..amt].copy_from_slice(&pwd_bytes[..amt]);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // Use a fixed or simple static nonce to entirely avoid nonce tracking
    let static_nonce = Nonce::from_slice(b"bbfR^ToP3LdD"); // Exactly 12 bytes
    let cipher = Aes256Gcm::new(key);

    cipher
        .encrypt(static_nonce, plaintext.as_bytes())
        .expect("Encryption failed")
}

// --- DECRYPT ---
pub fn quick_decrypt(ciphertext: &[u8], password: &str) -> String {
    let mut key_bytes = [0u8; 32];
    let pwd_bytes = password.as_bytes();
    let amt = pwd_bytes.len().min(32);
    key_bytes[..amt].copy_from_slice(&pwd_bytes[..amt]);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    let static_nonce = Nonce::from_slice(b"0123456789ab");
    let cipher = Aes256Gcm::new(key);

    let decrypted_bytes = cipher
        .decrypt(static_nonce, ciphertext)
        .expect("Decryption failed! Wrong password or corrupted data.");

    String::from_utf8(decrypted_bytes).expect("Invalid UTF-8")
}

// fn main() {
//     let encrypted = quick_encrypt("my-secret-key", "mypass");
//     let decrypted = quick_decrypt(&encrypted, "mypass");
//     println!("Decrypted output: {}", decrypted);
// }
