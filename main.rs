fn point_doubling(x1: f64, y1: f64, x2: f64, y2: f64, a: f64) -> (f64, f64) {
    // Compute the slope s
    let s = if x1 == x2 && y1 == y2 {
        // Point doubling case
        (3.0 * x1 * x1 + a) / (2.0 * y1)
    } else {
        // Point addition case
        (y2 - y1) / (x2 - x1)
    };

    // Compute the new x-coordinate x3
    let x3 = s * s - x1 - x2;

    // Compute the new y-coordinate y3
    let y3 = s * (x1 - x3) - y1;

    (x3, y3)
}

// fn point_addition(x1: f64, y1: f64, x2: f64, y2: f64, a: f64) -> (f64, f64) {
//     // Compute the slope
//     let s = (y1 - y2) / (x1 - x2);
//     // Compute the new x-coordinate x_r
//     let x_r = s * s - x1 - x2;
//     // Compute the new y-coordinate y_r
//     let y_r = s * (x1 - x_r) - y1;
//     (x_r, y_r)
// }

fn cyclic_group(mut x1: f64, mut y1: f64, a: f64) -> (f64, f64) {
    // Call point doubling
    let k = 19;
    for _ in 0..k {
        let (x_new, y_new) = point_doubling(x1, y1, x1, y1, a);
        x1 = x_new;
        y1 = y_new;
    }
    (x1, y1)
}
fn main() {
    let x1 = 2.0;
    let y1 = 3.0;
    let a = 1.0;
    let result = cyclic_group(x1, y1, a);
    println!("Result: {:?}", result);
}














































// use rand::rngs::OsRng;
// use p256::ecdh::EphemeralSecret;
// use p256::ecdh::SharedSecret;
// use p256::elliptic_curve::sec1::ToEncodedPoint;
// use p256::PublicKey;
// use aes_gcm::aead::{Aead, KeyInit};
// use aes_gcm::{Aes256Gcm, Key, Nonce}; // Or `Aes128Gcm`

// fn main() {
//     // Generate an ephemeral secret key
//     let secret = EphemeralSecret::random(&mut OsRng);

//     // Get the corresponding public key
//     let public_key = PublicKey::from(&secret);

//     // Serialize the public key to a byte array
//     let public_key_bytes = public_key.to_encoded_point(false).as_bytes().to_vec();

//     println!("Public Key: {:?}", public_key_bytes);

//     // For demonstration purposes, we'll use the same secret to derive a shared secret
//     let shared_secret = secret.diffie_hellman(&public_key);

//     // Serialize the shared secret to a byte array
//     let shared_secret_bytes = shared_secret.as_bytes().to_vec();

//     println!("Shared Secret: {:?}", shared_secret_bytes);

//     // Use the shared secret as the key for AES encryption
//     let key = Key::<Aes256Gcm>::from_slice(&shared_secret_bytes[..32]); // Use the first 32 bytes for AES-256
//     let cipher = Aes256Gcm::new(key);

//     // Encrypt a message
//     let nonce = Nonce::from_slice(b"unique nonce"); // 96-bits; unique per message
//     let plaintext = b"Hello, world!";
//     let ciphertext = cipher.encrypt(nonce, plaintext.as_ref())
//         .expect("encryption failure!");

//     println!("Ciphertext: {:?}", ciphertext);

//     // Decrypt the message
//     let decrypted_plaintext = cipher.decrypt(nonce, ciphertext.as_ref())
//         .expect("decryption failure!");

//     println!("Decrypted Plaintext: {:?}", String::from_utf8(decrypted_plaintext).unwrap());
// }