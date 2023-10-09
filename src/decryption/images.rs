  /// Decodes an encoded image and extracts text.
  ///
  /// This function takes an encoded image as input, decodes it, and extracts text from the image
  /// based on the pixel colors. It returns the extracted text as an `Option<String>`.
  ///
  /// # Arguments
  ///
  /// * `encoded_image` - The Base64 encoded image to be decoded and processed.
  ///
  /// # Returns
  ///
  /// An `Option<String>` containing the extracted text if successful, or `None` if there was an
  /// error during image decoding or text extraction.
  ///
  /// # Examples
  ///
  /// ```
  /// use encrypted_images::decryption::images::decode_image_and_extract_text;
  ///
  /// let encoded_image = "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";
  /// let extracted_text = decode_image_and_extract_text(encoded_image);
  /// assert!(extracted_text.is_some());
  /// ```
  use image::io::Reader as ImageReader;
  use crate::char_mappings::maps::mappings::numbers_to_letter;
  pub fn decode_image_and_extract_text(encoded_image: &str) -> Option<String> {
    let image_data = base64::decode(encoded_image).ok()?;
    let img = ImageReader::new(std::io::Cursor::new(image_data))
      .with_guessed_format()
      .unwrap()
      .decode()
      .unwrap()
      .to_rgba8();
    let width = img.width();
    let mut extracted_text = String::with_capacity(width as usize);
    for x in 0..width {
      let pixel = img.get_pixel(x, 0);
      let [r, g, b, _] = pixel.0;
      if let Some(c) = numbers_to_letter(r, g, b) {
        extracted_text.push(c);
      }
    }
    let first_char = extracted_text.remove(0);
    extracted_text.push(first_char);
    Some(extracted_text)
  }
