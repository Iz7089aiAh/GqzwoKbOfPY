 * Features:
 * - Password encryption and decryption
 * - Error handling
 * - Comments and documentation
 * - Adherence to Rust best practices
 */

use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::str;
use ring::{aead, rand::{SecureRandom, SystemRandom},
          error::Unspecified};
use ring::rand::SecureRandom;
use ring::aead::{SealingKey, OpeningKey};
use base64::encode;

// Define the request body structure for encryption and decryption
#[derive(Serialize, Deserialize)]
struct PasswordRequest {
    password: String,
    mode: String, // 'encrypt' or 'decrypt'
}

#[derive(Serialize, Deserialize)]
struct PasswordResponse {
    result: String,
    message: String,
}

// AES-GCM encryption and decryption function
fn aes_gcm_encrypt_decrypt(password: &str, mode: &str) -> Result<String, String> {
    let key = ring::agreement::EphemeralPrivateKey::generate(&ring::agreement::X25519)?;
    let key_bytes = key.to_bytes();
    let nonce = ring::agree::agree_ephemeral(
        &ring::agreement::X25519,
        &key,
        &ring::agreement::UnprotectedPublicKey::new(&ring::agreement::X25519, &key_bytes).unwrap(),
    )?;
    let nonce_bytes = nonce.as_ref().to_vec();
    let sealing_key = SealingKey::new(aead::AES_256_GCM, &key_bytes).map_err(|_| "Failed to create sealing key")?;
    let opening_key = OpeningKey::new(aead::AES_256_GCM, &key_bytes).map_err(|_| "Failed to create opening key")?;

    let data = password.as_bytes();
    match mode {
        "encrypt" => {
            let in_out = sealing_key.seal_in_place(nonce_bytes.as_slice(), data).map_err(|_| "Encryption failed")?;
            Ok(encode(in_out))
        },
        "decrypt" => {
            let in_out = opening_key.open_in_place(nonce_bytes.as_slice(), data).map_err(|_| "Decryption failed")?;
            Ok(str::from_utf8(in_out).unwrap_or("Invalid UTF-8").to_string())
        },
        _ => Err("Invalid mode".to_string()),
    }
}

// Handler for encrypting a password
#[post("/encrypt")]
async fn encrypt_password(req_body: web::Json<PasswordRequest>) -> impl Responder {
    let result = aes_gcm_encrypt_decrypt(&req_body.password, &req_body.mode).unwrap_or_else(|e| e);
    HttpResponse::Ok().json(PasswordResponse {
        result,
        message: "Password encrypted successfully".to_string(),
    })
}

// Handler for decrypting a password
#[post("/decrypt")]
async fn decrypt_password(req_body: web::Json<PasswordRequest>) -> impl Responder {
    let result = aes_gcm_encrypt_decrypt(&req_body.password, &req_body.mode).unwrap_or_else(|e| e);
    HttpResponse::Ok().json(PasswordResponse {
        result,
        message: "Password decrypted successfully".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(encrypt_password)
            .service(decrypt_password)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
