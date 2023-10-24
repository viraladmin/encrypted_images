  /// Decrypts an encoded result using an optional decryption key.
  ///
  /// This function takes an encoded result and an optional decryption key, and attempts to decrypt
  /// the result using AES-128 CBC decryption. It also verifies the integrity of the data using HMAC.
  ///
  /// Timing Attack Protection:
  /// The decryption process is designed to protect against timing attacks, ensuring secure
  /// operation. It is suitable for standard encryption needs. Please see notes inside the encrypts
  /// function to understand additional security. Default encryption settings are not maximum security.
  ///
  /// # Arguments
  ///
  /// * `encoded_result` - The Base64-encoded result to be decrypted.
  /// * `key` - An optional decryption key. If not provided, the default key "welovenfts" is used.
  ///
  /// # Returns
  ///
  /// An `Option<String>` containing the decrypted plaintext if successful, or `None` if decryption
  /// fails or if the HMAC verification fails.
  ///
  /// # Examples
  ///
  /// ```
  /// use encrypted_images::decryption::text::decrypts;
  ///
  /// let encoded_result = "VkdocGMybHpiWGxqYnc9PbUWoPUFfy9Izm1wkCFZ8gSMWr6EUGW6UwYpnaounDkYmLNDjqWyvjcus2atCStKBOJSCnosjApRrcJrm44hatuaJHSYONbHNOmpk3Rja/xH";
  /// let key = "welovenfts";
  /// let decrypted_data = decrypts(encoded_result, Some(key));
  /// assert!(decrypted_data.is_some());
  /// ```
  use subtle::ConstantTimeEq;
  use openssl::symm::{decrypt, Cipher};
  use crate::encryption::text::hmac::calculate_hmac;
  use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
  const CUSTOM_ENGINE: engine::GeneralPurpose =
      engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
  pub fn decrypts(encoded_result: &str, key: Option<&str>) -> Option<String> {
    let key = key.unwrap_or("welovenfts");
    let mut padded_key = key.as_bytes().to_vec();
    while padded_key.len() < 16 {
        padded_key.push(b'\0');
    }
    let result_bytes = CUSTOM_ENGINE.decode(encoded_result).ok()?;
    let iv = &result_bytes[..16];
    let hmac = &result_bytes[16..48];
    let ciphertext = &result_bytes[48..];
    let hmac_calculated = calculate_hmac(ciphertext, &padded_key);
    if hmac_calculated.ct_eq(hmac).unwrap_u8() == 1 {
        let cipher = Cipher::aes_128_cbc();
        let decrypted_data = decrypt(cipher, &padded_key, Some(iv), ciphertext).ok()?;
        Some(String::from_utf8_lossy(&decrypted_data).to_string())
    } else {
        println!("Decryption Failed");
        None
    }
  }
