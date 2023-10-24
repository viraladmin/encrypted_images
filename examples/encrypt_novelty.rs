  use encrypted_images::encryption::text::encrypts;

  fn main() {
    let plaintext = "This Is Plain Text";
    let encrypted_text = encrypts(plaintext, None, None).unwrap();
    println!("Encrypted text: {}", encrypted_text);
  }
