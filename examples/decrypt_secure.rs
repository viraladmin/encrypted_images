  use encrypted_images::decryption::text::decrypts;

  fn main() {
    let encrypted_text = "OWFNTGpvaGFMbWtTUkE9PcjB/klKI3ix+Z0uVuYbd3zRqaTjMgxotQu4hz1FRSfPWRQMOBhLSI6+KFPl8qldeCPoUYvezvVMOScWll9OzCA=";
    let key = Some("16characterslong");
    if let Some(decrypted_text) = decrypts(encrypted_text, key) {
      println!("Decrypted text: {}", decrypted_text);
    } else {
      println!("Decryption failed.");
    }
  }
