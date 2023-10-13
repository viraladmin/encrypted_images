https://encryptedimages.art/

# Encrypted Images

## Overview

`encrypted_images` is a Rust crate that allows you to perform text encryption to images and image decryption to text.

## Supported Characters

The following characters can be used for encryption and decryption:

- A-Z (uppercase letters)
- a-z (lowercase letters)
- 0-9 (digits)
- +,/ (other)

## Min / Max Requirements

The minimum and maximum allowed string sizes for encryption and decryption

- min: 10 character string
- max: u64 (18,446,744,073,709,551,615 characters)

## Watermark Support
- bitcoin
- ethereum
- cardano
- none

## Functions

### `encrypts`

Encrypts a text string using AES-128 CBC encryption with a private key.

```rust
use encrypted_images::encryption::text::encrypts;

let plaintext = "ThisIsPlainText";
let key = Some("YourSecretKey"); 
let encrypted_text = encrypts(plaintext, key);
println!("Encrypted text: {}", encrypted_text.as_ref().unwrap());
```

Encrypts a text string using AES-128 CBC encryption without a private key.

```rust
use encrypted_images::encryption::text::encrypts;
let plaintext = "ThisIsPlainText";
let encrypted_text = encrypts(plaintext);
println!("Encrypted text: {}", encrypted_text.as_ref().unwrap());
```

### `decrypts`

Decrypts an encrypted text string using AES-128 CBC decryption with a private key.

```rust
use encrypted_images::decryption::text::decrypts;

let encrypted_text = "YourEncryptedText";
let key = Some("YourSecretKey");
if let Some(decrypted_text) = decrypts(encrypted_text.as_ref().unwrap(), key) {
    println!("Decrypted text: {}", decrypted_text);
} else {
    println!("Decryption failed.");
}
```

Decrypts an encrypted text string using AES-128 CBC decryption without a private key.

```rust
use encrypted_images::decryption::text::decrypts;

let encrypted_text = "YourEncryptedText";
if let Some(decrypted_text) = decrypts(encrypted_text) {
    println!("Decrypted text: {}", decrypted_text);
} else {
    println!("Decryption failed.");
}
```

### `create_img`

Creates an encoded image from plaintext.

```rust
use encrypted_images::encryption::images::create_img;

let plaintext = "ThisIsPlainText";
let watermark = "Bitcoin"; // optional delete to use no watermark
if let Some(encoded_image) = create_img(plaintext, watermark) {
    println!("Encoded image: {}", encoded_image);
} else {
    println!("Image creation or encoding failed.");
}
```

### `decode_image_and_extract_text`

Decode an image and extracts original text from it.

```rust
use encrypted_images::decryption::images::decode_image_and_extract_text;

let encoded_image = "YourEncodedImage";
if let Some(extracted_text) = decode_image_and_extract_text(encoded_image) {
    println!("Extracted text: {}", extracted_text);
} else {
    println!("Image decoding or text extraction failed.");
}
```

### Advanced Usage 

1: Encrypt and Create Image

```rust
use encrypted_images::encryption::text::encrypts;
use encrypted_images::encryption::images::create_img;

fn main() {
    // Your encryption key
    let key = Some("your_secret_key"); //optional

    // Input text to be encrypted
    let input_text = "This is a secret message.";

    // Encrypt the input text
    let encrypted = encrypts(input_text, key.clone());

    // Create an image from the encrypted text
    let watermark = "bitcoin"; // Optional watermark text
    let image_data = create_img(encrypted.as_ref().unwrap(), watermark);

    match image_data {
        Some(encoded_image) => println!("Encoded Image:\n{}", encoded_image),
        None => println!("Failed to create the image."),
    }
}
```

2: Decode Image and Decrypt

```rust
use encrypted_images::encryption::images::decode_image_and_extract_text;
use encrypted_images::decryption::text::decrypts;

fn main() {
    // Encoded image containing the encrypted text
    let encoded_image = "your_encoded_image_here";

    // Decode the encoded image and extract the text
    let extracted_text = decode_image_and_extract_text(encoded_image);

    match extracted_text {
        Some(encrypted_text) => {
            // Your encryption key
            let key = Some("your_secret_key"); //optional

            // Decrypt the extracted text
            let decrypted_text = decrypts(encrypted.as_ref().unwrap(), key.clone());

            match decrypted_text {
                Some(text) => println!("Decrypted Text:\n{}", text),
                None => println!("Decryption failed."),
            }
        }
        None => println!("Failed to decode the image and extract text."),
    }
}
```
