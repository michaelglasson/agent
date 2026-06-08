// --- Write to AWS S3 ---

use crate::structs::*;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use aws_config::Region;
use aws_credential_types::Credentials;
use aws_sdk_s3::Client;
use aws_sdk_s3::primitives::ByteStream;
use chrono::{DateTime, Local};
use rpassword::prompt_password;
use std::env;
use std::time::SystemTime;

const ENCRYPTED_KEY: &[u8] = &[
    61, 46, 250, 160, 139, 150, 210, 174, 30, 113, 236, 85, 207, 159, 180, 139, 209, 0, 227, 183,
    35, 76, 166, 236, 135, 172, 150, 73, 78, 235, 216, 126, 80, 174, 193, 0, 102, 132, 131, 215,
    241, 123, 250, 35, 2, 174, 60, 125, 114, 36, 173, 185, 240, 215, 159, 42,
];

// Pass &profile.system.host_name into the function parameters
pub async fn write_to_s3(
    profile: &MachineProfile,
    host_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let aws_access_key_id = r"AKIAZAMXGDOZP2L2JN6M";
    let password = prompt_password("Enter your password: ").expect("Failed to read password");
    let aws_secret_access_key = decrypt_aws_access_key(&password);
    let aws_region = r"ap-southeast-2";

    let credentials = Credentials::new(
        aws_access_key_id,
        aws_secret_access_key,
        None,     // session token (optional)
        None,     // expiry (optional)
        "manual", // provider name
    );

    let config = aws_config::from_env()
        .region(Region::new(aws_region))
        .credentials_provider(credentials)
        .load()
        .await;

    let client = Client::new(&config);

    let bucket_name = "mynym-619329231794-ap-southeast-2-an".to_string();

    // 1. Clean the host_name to ensure it contains safe characters for an S3 URI path
    let safe_host_name = if host_name.is_empty() {
        "unknown_host"
    } else {
        host_name
    };

    let datetime_local: DateTime<Local> = SystemTime::now().into();
    let custom_string = datetime_local.format("%Y-%m-%d %H:%M:%S").to_string();

    let object_key = env::var("S3_OBJECT_KEY")
        .unwrap_or_else(|_| format!("agent/{} at {}.json", safe_host_name, custom_string));

    let json_bytes = serde_json::to_vec(profile)?;

    client
        .put_object()
        .bucket(bucket_name)
        .key(object_key)
        .body(ByteStream::from(json_bytes))
        .content_type("application/json")
        .send()
        .await?;

    println!("Successfully uploaded machine profile data payload target directly to S3!");
    Ok(())
}

fn decrypt_aws_access_key(password: &str) -> String {
    // 2. Format the password key
    let mut key_bytes = [0u8; 32];
    let pwd_bytes = password.as_bytes();
    let amt = pwd_bytes.len().min(32);
    key_bytes[..amt].copy_from_slice(&pwd_bytes[..amt]);
    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    // 3. Decrypt using the same static nonce
    let static_nonce = Nonce::from_slice(b"0123456789ab");
    let cipher = Aes256Gcm::new(key);
    let decrypted_bytes = cipher
        .decrypt(static_nonce, ENCRYPTED_KEY)
        .expect("Decryption failed! Wrong password provided.");
    String::from_utf8(decrypted_bytes).expect("Invalid UTF-8")
}
