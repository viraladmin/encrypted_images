  use encrypted_images::encryption::text::encrypts;

  fn main() { 
    let plaintext = "This Is Plain Text";
    let key = Some("16characterslong");
    let strength = Some("advanced");
    let encrypted_text = encrypts(plaintext, key, strength).unwrap();
    println!("Encrypted text: {}", encrypted_text);
  }
