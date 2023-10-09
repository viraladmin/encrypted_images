  use openssl::symm::{encrypt, Cipher};
  use base64::encode;
  use crate::encryption::text::hmac::calculate_hmac;
  /// Encrypts the input text with an optional encryption key.
  ///
  /// The `input` parameter is the text to be encrypted.
  ///
  /// The `key` parameter is an optional encryption key. If not provided, a default key is used.
  ///
  /// # Examples
  ///
  /// Encrypt a text with a custom key:
  ///
  /// ```
  /// use encrypted_images::encryption::text::encrypts;
  ///
  /// let input = "ThisIsJustaTestString";
  /// let key = Some("your_secret_key");
  /// let encrypted = encrypts(input, key);
  ///
  /// assert!(encrypted.len() > 0);
  /// ```
  ///
  /// Encrypt a text without providing a custom key:
  ///
  /// ```
  /// use encrypted_images::encryption::text::encrypts;
  ///
  /// let input = "ThisIsJustaTestString";
  /// let encrypted = encrypts(input, None);
  ///
  /// assert!(encrypted.len() > 0);
  /// ```
  pub fn encrypts(input: &str, key: Option<&str>) -> String {
    let cipher = Cipher::aes_128_cbc();
    let key = key.unwrap_or("welovenfts");
    let iv_bytes = &input.to_string()[..10];
    let iv = base64::encode(iv_bytes);
    let mut padded_key = key.as_bytes().to_vec();
    while padded_key.len() < 16 {
      padded_key.push(b'\0');
    }
    let ciphertext = encrypt(cipher, &padded_key, Some(iv.as_bytes()), input.as_bytes()).unwrap();
    let hmac = calculate_hmac(&ciphertext, &padded_key);
    let mut result = iv.into_bytes();
    result.extend_from_slice(&hmac);
    result.extend_from_slice(&ciphertext);
    let encoded_result = encode(&result);
    encoded_result
  }
  pub mod hmac {
    pub(crate) fn calculate_hmac(data: &[u8], key: &[u8]) -> Vec<u8> {
      use openssl::hash::MessageDigest;
      use openssl::pkey::PKey;
      use openssl::sign::Signer;
      let pkey = PKey::hmac(key).unwrap();
      let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
      signer.update(data).unwrap();
      signer.sign_to_vec().unwrap()
    }
  }
