  pub mod char_mappings;
  pub mod encryption;
  pub mod decryption;


  #[cfg(test)]
  mod tests {
    use crate::char_mappings::maps::mappings::{get_color, numbers_to_letter};
    use crate::encryption::text::encrypts;
    use crate::decryption::text::decrypts;
    use crate::encryption::images::create_img;
    use crate::decryption::images::decode_image_and_extract_text;
    use std::time::{Instant, Duration};

    #[test]
    fn test_char_conversion() {
      // Define valid characters for testing
      let valid_characters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/=";
      for character in valid_characters.chars() {
        // Get the color for the character
        let color_result = get_color(character);
        // Get the character back from the color
        let character_result = numbers_to_letter(color_result.unwrap().0, color_result.unwrap().1, color_result.unwrap().2);
        // Check if character_result matches the input character
        assert_eq!(character_result.unwrap(), character);
      }

      // Test invalid characters
      let invalid_characters = "!@#$%^&*()_";
      for character in invalid_characters.chars() {
        // Attempt to get a color for the invalid character
        let color_result = get_color(character);

        // Ensure the result is None for invalid characters
        assert_eq!(color_result, None);
      }
    }

    #[test]
    fn test_encrypts_decrypts_with_key() {
      // Input data to encrypt
      let input = "ThisIsJustaTestString";
      // Your encryption key (replace with the actual key you use)
      let key = Some("your_secret_key");
      // Encrypt the input
      let encrypted = encrypts(input, key.clone(), None);
      // Decrypt the encrypted data
      let decrypted = decrypts(encrypted.as_ref().unwrap(), key);
      // Assert that decryption matches the original input
      assert_eq!(decrypted, Some(input.to_string()));
    }

    #[test]
    fn test_encrypts_decrypts_without_key() {
      // Input data to encrypt
      let input = "ThisIsJustsTestString";
      // Encrypt the input without a key
      let encrypted = encrypts(input, None, None);
      // Decrypt the encrypted data without a key
      let decrypted = decrypts(encrypted.as_ref().unwrap(), None);
      // Assert that decryption matches the original input
      assert_eq!(decrypted, Some(input.to_string()));
    }

    #[test]
    fn test_maximum_length() {
      // Your encryption key
      let key = Some("your_secret_key");
  
      let mut length = 1844674406; // Start with an initial length
      let max_length = 1844674407; // Set the maximum length
      while length <= max_length {
        let encrypted = encrypts(&"A".repeat(length), key.clone(), None);
        let decrypted = decrypts(encrypted.as_ref().unwrap(), key.clone());

        if decrypted.is_none() {
            println!("Maximum size: {} characters", length);
            break;
        }
        length += 1;
      }
    }

    #[test]
    fn test_decryption_timing() {
      let encoded_result = "Your encrypted data"; // Replace with an actual encoded result
      let key = Some("Your secret key"); // Replace with an actual key
      // Measure the time for decryption
      let start_time = Instant::now();
      let _ = decrypts(encoded_result, key);
      let end_time = Instant::now();
      let decryption_time = end_time - start_time;
      // Ensure that decryption time is within an acceptable range (e.g., less than 1 second)
      assert!(decryption_time < Duration::from_secs(1));
    }

    #[test]
    fn test_minimum_length() {
        // minimum 10 characters required to encrypt
        let input = "aaaaaaaaaa";
        // Your encryption key
        let key = Some("your_secret_key");
        // Encrypt the input
        let encrypted = encrypts(input, key.clone(), None);
        // Decrypt the encrypted data
        let decrypted = decrypts(encrypted.as_ref().unwrap(), key);
        // Assert that decryption matches the original input
        assert_eq!(decrypted, Some(input.to_string()));
    }

#[test]
fn test_create_img_and_decode() {
    let ciphertext = "ThisIsJustaTestString";
    let watermark = "Bitcoin"; // Text watermark
    let style = "h";

    // Create the image
    let encoded_image = create_img(ciphertext, style, watermark, None, None, None, None, None, None);

    // Print out the encoded image for debugging
    println!("Encoded Image: {:?}", encoded_image);

    // Decode the image and extract text
    let extracted_text = decode_image_and_extract_text(&encoded_image.unwrap());

    // Print out the extracted text for debugging
    println!("Extracted Text: {:?}", extracted_text);

    // Check if extracted text matches the original ciphertext
    assert_eq!(extracted_text, Some(ciphertext.to_string()));
}

    #[test]
    fn test_encrypts_decrypts() {
      // Input data to encrypt
      let input = "ThisIsJustaTestString";
      // Your encryption key (replace with the actual key you use)
      let key = Some("your_secret_key");
      // Encrypt the input
      let encrypted = encrypts(input, key.clone(), None);
      // Print the encrypted string for testing
      println!("Encrypted: {}", encrypted.as_ref().unwrap());
      // Decrypt the encrypted data
      let decrypted = decrypts(encrypted.as_ref().unwrap(), key);
      // Assert that decryption matches the original input
      assert_eq!(decrypted, Some(input.to_string()));
    }
  }
