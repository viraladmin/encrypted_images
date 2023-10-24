  use encrypted_images::encryption::text::encrypts;
  use encrypted_images::encryption::images::create_img;

  fn main() {
    let key = Some("your_secret_key");
    let ciphertext = "This is a secret message.";
    let style = "h";
    let encrypted = encrypts(ciphertext, key.clone(), None).unwrap();
    let watermark = "bitcoin";
    let image_data = create_img(&encrypted, style, watermark, None, None, None, None, None, None);

    match image_data {
        Some(encoded_image) => println!("Encoded Image:\n{}", encoded_image),
        None => println!("Failed to create the image."),
    }
  }
