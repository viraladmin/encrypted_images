  use encrypted_images::decryption::text::decrypts;

  fn main() { 
    let encrypted_text = "VkdocGN5QkpjeUJRYkE9PZNY2MOW01NWpSxCtFG6acHuAWun+CElPQ/IIwd0gy+D+IiBqB/5+qo8Jr9bMBOwoih3amCtjXlkAlRKHX5fhqI=";
    if let Some(decrypted_text) = decrypts(encrypted_text, None) {
      println!("Decrypted text: {}", decrypted_text);
    } else {
      println!("Decryption failed.");
    }
  }
