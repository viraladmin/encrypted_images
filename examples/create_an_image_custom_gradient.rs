  use encrypted_images::encryption::images::create_img;

  fn main() { 
    let ciphertext = "VkdocGN5QkpjeUJRYkE9PZNY2MOW01NWpSxCtFG6acHuAWun+CElPQ/IIwd0gy+D+IiBqB/5+qo8Jr9bMBOwoih3amCtjXlkAlRKHX5fhqI=";
    let style = "h";
    let watermark = "bitcoin";
    let r = Some(46);
    let g = Some(115);
    let b = Some(82);
    if let Some(encoded_image) = create_img(ciphertext, style, watermark, r, g, b, None, None, None) {
      println!("Encoded image: {}", encoded_image);
    } else {
      println!("Image creation or encoding failed.");
    }
  }
