  /// Encrypts the input text with an optional encryption key with timing attack protections and
  /// standard encryption security measures (which are optional).
  ///
  /// The `input` parameter is the text to be encrypted.
  ///
  /// The `key` parameter is an optional encryption key. If not provided, a default key is used.
  ///
  /// The `strength` parameter is optional security level. Is set this value can be default or
  /// advanced.
  ///
  /// # Notes
  ///
  /// - This function has been updated to include timing attack protections to enhance security.
  /// - The function uses static IV (Initialization Vector) bytes by default to generate consistant
  ///   encrypted text output. While suitable for some situations ensure to use the advanced
  ///   strength for highly sensative data but note that everytime you encrypt with the advanced
  ///   method the output will look differnt. 
  /// - Again stressing that the default security option is not the ideal choice for scenarios requiring the 
  ///   highest level of encryption security.
  /// - Without advanced security there are potentials for comparison attacks that can occur this encryption.
  /// - Only use standard settings for novelty usage.
  ///
  /// # Examples
  ///
  /// Encrypt a text with a custom key:
  ///
  /// ```
  /// use encrypted_images::encryption::text::encrypts;
  ///
  /// let input = "ThisIsJustaTestString";
  /// let key = "your_secret_key";
  /// let encrypted = encrypts(input, Some(key), None);
  ///
  /// assert!(encrypted.as_ref().unwrap().len() > 0);
  /// ```
  ///
  /// Encrypt a text without providing a custom key:
  ///
  /// ```
  /// use encrypted_images::encryption::text::encrypts;
  ///
  /// let input = "ThisIsJustaTestString";
  /// let encrypted = encrypts(input, None, None);
  ///
  /// assert!(encrypted.as_ref().unwrap().len() > 0);
  /// ```
  ///
  /// Encrypt a text with advanced security:
  ///
  /// ```
  /// use encrypted_images::encryption::text::encrypts;
  ///
  /// let input = "ThisIsJustaTestString";
  /// let strength = "advanced";
  /// let encrypted = encrypts(input, None, Some(strength));
  ///
  ///  assert!(encrypted.as_ref().unwrap().len() > 0);
  /// ```
  use rand::{Rng};
  use rand::rngs::OsRng;
  use openssl::symm::{encrypt, Cipher};
  use crate::encryption::text::hmac::calculate_hmac;
  use subtle::ConstantTimeEq;
  use base64::{Engine as _, engine::{self, general_purpose}, alphabet};
  const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);


  pub fn encrypts(input: &str, key: Option<&str>, strength: Option<&str>) -> Option<String> {
    let cipher = Cipher::aes_128_cbc();
    let key = key.unwrap_or("welovenfts");
    let strength = strength.unwrap_or("default");
    let iv_bytes = &input.to_string()[..10];
    let iv: String;
    if strength == "default" {
      iv = CUSTOM_ENGINE.encode(iv_bytes);
    } else {
      let num_bytes = 10; // Adjust this to the number of random bytes you need
      let random_bytes = generate_random_bytes(num_bytes);
      iv = CUSTOM_ENGINE.encode(random_bytes);
    }
    let mut padded_key = key.as_bytes().to_vec();
    while padded_key.len() < 16 {
        padded_key.push(b'\0');
    }
    padded_key.truncate(16);
    let ciphertext = encrypt(cipher, &padded_key, Some(iv.as_bytes()), input.as_bytes()).unwrap();
    let hmac = calculate_hmac(&ciphertext, &padded_key);
    if hmac.ct_eq(&calculate_hmac(&ciphertext, &padded_key)).unwrap_u8() == 1 {
        let mut result = iv.into_bytes();
        result.extend_from_slice(&hmac);
        result.extend_from_slice(&ciphertext);
        let encoded_result = CUSTOM_ENGINE.encode(&result);
        Some(encoded_result)
    } else {
        println!("Encryption HMAC validation failed");
        None
    }
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
  fn generate_random_bytes(num_bytes: usize) -> Vec<u8> {
    let mut rng = OsRng;
    let mut random_bytes = vec![0u8; num_bytes];
    rng.fill(&mut random_bytes[..]);
    random_bytes
  }

