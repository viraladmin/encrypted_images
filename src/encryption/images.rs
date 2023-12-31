/// Creates an image from ciphertext with optional watermark and additional styling options.
///
/// This function takes ciphertext and an optional watermark as input and generates an image
/// where the ciphertext is visually represented. You can customize the image's style, overlay
/// an optional watermark (e.g., Bitcoin, Ethereum, Cardano, or none), adjust color, and more.
/// The generated image is encoded as a PNG image and then Base64 encoded before being returned
/// as an `Option<String>`.
///
/// # Arguments
///
/// * `ciphertext` - The ciphertext to be represented in the image.
/// * `style` - The style of the generated image:
///     - "h": Default position (no change).
///     - "h2": Vertical flip.
///     - "v": 90-degree rotation.
///     - "v2": 270-degree rotation with a vertical flip.
/// * `watermark` - The watermark to overlay on the image:
///     - "empty": No watermark.
///     - "bitcoin": Bitcoin watermark.
///     - "ethereum": Ethereum watermark.
///     - "cardano": Cardano watermark.
///     - __your own base64 encoded watermark__
/// * `r` - Custom red color component (0-255) for gradient.
/// * `g` - Custom green color component (0-255) for gradient.
/// * `b` - Custom blue color component (0-255) for gradient.
/// * `a` - Custom alpha (opacity) value (0-255). Should be None unless using custom watermark.
/// * `w` - Custom width for the watermark image. Should be None unless using custom watermark.
/// * `h` - Custom height for the watermark image. Should be None unless using custom watermark.
///
/// # Returns
///
/// An `Option<String>` containing the Base64 encoded image if successful, or `None` if there
/// was an error during image creation or encoding.
///
/// # Examples
///
/// ```
/// use encrypted_images::encryption::images::create_img;
///
/// let ciphertext = "ThisIsCiphertext";
/// let style = "h2";
/// let watermark = "Bitcoin";
/// let image_data = create_img(ciphertext, style, watermark, Some(100), Some(134), Some(131), None, None, None);
/// assert!(image_data.is_some());
/// ```
  use image::{ColorType, DynamicImage, RgbaImage, Rgba, imageops};
  use image::png::PngEncoder;
  use image::io::Reader as ImageReader;
  use crate::char_mappings::maps::mappings::get_color;
  use std::io::Cursor;
  use base64::{Engine as _, engine::{self, general_purpose}, alphabet};

  fn load_watermark(
      watermark: &str,
      alpha: Option<u8>,
      width: Option<u32>,
      height: Option<u32>,
  ) -> Option<DynamicImage> {
      let custom_engine: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
      let decoded = custom_engine.decode(watermark);
      let is_base64 = decoded.is_ok();
      if is_base64 {
          if let (Some(_a), Some(w), Some(h)) = (alpha, width, height) {
              let img = match image::load_from_memory(&decoded.ok()?) {
                  Ok(img) => img,
                  Err(_) => return None,
              };
              Some(img.resize(w, h, image::imageops::FilterType::Nearest))
          } else {
              None
          }
      } else {
          let watermark: &str = match watermark {
              "empty" => "",
              "bitcoin" => "iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAABhWlDQ1BJQ0MgcHJvZmlsZQAAKJF9kT1Iw0AcxV9TxQ8qInYQUchQXbQgKuIoVSyChdJWaNXB5NIvaNKQpLg4Cq4FBz8Wqw4uzro6uAqC4AeIq4uToouU+L+k0CLGg+N+vLv3uHsHCLUSU822CUDVLCMRjYjpzKrY8YouBNCHMQxLzNRjycUUPMfXPXx8vQvzLO9zf44eJWsywCcSzzHdsIg3iGc2LZ3zPnGQFSSF+Jx43KALEj9yXXb5jXPeYYFnBo1UYp44SCzmW1huYVYwVOJp4pCiapQvpF1WOG9xVksV1rgnf2Egq60kuU5zCFEsIYY4RMiooIgSLIRp1UgxkaD9iId/0PHHySWTqwhGjgWUoUJy/OB/8LtbMzc16SYFIkD7i21/jAAdu0C9atvfx7ZdPwH8z8CV1vSXa8DsJ+nVphY6Anq3gYvrpibvAZc7wMCTLhmSI/lpCrkc8H5G35QB+m+B7jW3t8Y+Th+AFHW1fAMcHAKjecpe93h3Z2tv/55p9PcDpKVyu4rx+1IAAAAGYktHRAD/AP8A/6C9p5MAAAAJcEhZcwAALiMAAC4jAXilP3YAAAAHdElNRQfnCQwTBQINStQTAAAAGXRFWHRDb21tZW50AENyZWF0ZWQgd2l0aCBHSU1QV4EOFwAAAoZJREFUWMPtl81LVUEYxn9XKsRyKAejlZaWeSstwV0r21QE7fI/iKRVjYuIImxhqxohWhStIyIhIgpbFUgFUV0/Em8bQxSRaiQmslDktmikyzj36D3n0MoXzmKe886Z5zzzfszAusUwq2Wf1XLYanm/CDsU51sVMTk0A61Ak1u8FchZLQtWy1P/iwDAuDcGmCjnQxtWkboJeAXkgTFg2C2607nkAwRGyyGQWYXASeBxhMukI1QPZIEJoUxjagoA34G3wD5gS+B9vXuWrcFqOQncA24JZWYSKfBPiZpKyPQDJ8r4uXngilDmRuIgFGruN/DBg6eBOuAIcAl4572vAq5bLU8n2YJQ5C9bXigzBUwBL4BrVsszwG3Prxe4m0YariCwUilzB3jmwbVWy21pEGjxxqXSbWlFoFUUbCICVsuWADwS8OsAjnlwrvrc3FLSGGgOYG+sljngK/AFaAP2B/yuphGE2RJ4W8ScceCCUOZJGr3AV+AjoF2VnC4xpz4QN6ltwaBQprto7/cCXcBZYFNRHei1WlYJZS4nVcDv9cNe+n0SypwHjgbmdlstt8YmYLXMBkr2WLhimpfAkAdXAo1lbYHVUgINLtc7Ai75qMqdRjfsAB5GNJmfJYhfdMSLbTHqjFCKwOEI0lXAvNXyM2BcFtQCe4DtAf9HQpmFcglY4D1wMMJnl3vaV2nJPYnOA1bL3cBx4KaDZoEda9jeWaBTKDOYwoFEdgIP3LA9AyOFv6mYdcey5556r4EBocyv1AtRBkarlVm0WtY5aEAo0xX3jlFuKZ6oVmbBLb7ZYUNJLjlrVaAfmHHy+qV5PAmBTJxJP/pqNhYKmVbgAPBUKPNt/cIa1/4AtWS4hwYohc4AAAAASUVORK5CYII=",
              "ethereum" => "iVBORw0KGgoAAAANSUhEUgAAACAAAAAgCAYAAABzenr0AAABhWlDQ1BJQ0MgcHJvZmlsZQAAKJF9kT1Iw0AcxV9TxQ8qInYQUchQXbQgKuIoVSyChdJWaNXB5NIvaNKQpLg4Cq4FBz8Wqw4uzro6uAqC4AeIq4uToouU+L+k0CLGg+N+vLv3uHsHCLUSU822CUDVLCMRjYjpzKrY8YouBNCHMQxLzNRjycUUPMfXPXx8vQvzLO9zf44eJWsywCcSzzHdsIg3iGc2LZ3zPnGQFSSF+Jx43KALEj9yXXb5jXPeYYFnBo1UYp44SCzmW1huYVYwVOJp4pCiapQvpF1WOG9xVksV1rgnf2Egq60kuU5zCFEsIYY4RMiooIgSLIRp1UgxkaD9iId/0PHHySWTqwhGjgWUoUJy/OB/8LtbMzc16SYFIkD7i21/jAAdu0C9atvfx7ZdPwH8z8CV1vSXa8DsJ+nVphY6Anq3gYvrpibvAZc7wMCTLhmSI/lpCrkc8H5G35QB+m+B7jW3t8Y+Th+AFHW1fAMcHAKjecpe93h3Z2tv/55p9PcDpKVyu4rx+1IAAAAGYktHRAD/AP8A/6C9p5MAAAAJcEhZcwAALiMAAC4jAXilP3YAAAAHdElNRQfnCQwSFQ84+9DIAAAAGXRFWHRDb21tZW50AENyZWF0ZWQgd2l0aCBHSU1QV4EOFwAAA1ZJREFUWMO9l01oXFUUx3/3vXkz+TYwpgQEF8ZN/IqSgrHXBI2LcWNJzUYskTpxIQimcayCQoSi3XTevAo2Wl1LoZsqLmpWWchtQ11odQilGSsGgk1N22RoOjO+9+a6MA3T5mvmZWYuvMU799xz/+fc8wm7WMmk6u7tVd27kRHazWEh+CIUQgCDQWUYQQ/atnpNawY9jxf7+tTrdQWQSqkmrXHu/rsuTiymmusGQGuOAp0lpD3LyxytCwDbVt1ac/h+uu8zLmXlDmkE0P4rwNyELlyXUzUFYNvqIDCw1b7v09/Xp0ZqAmByUjVrTWonPtfFHhlRLVUHkMvxKbCnDNaOuTk+qyoAx1GPa81YuUJdl3cHBtQTVQPg+5wCRCVvm8/zdVUA2LZ6A5D307NZSKfBuwWeuyno5/btU4d2TOfbbZ48qVpzOf4AOu7Slpbg6lWYn1/zjczaRhPQCqGGe0QsdXXx6JkzciVQMcrnOQZ0FItw7RpkMnD9+hbMd/7/vAjQBmYjCMGD8/McA96p2AKplHqyUOC3hQW4fBlu394iOjLbqPYAGE3Q3s7T09PyUkUWmJ1ldG4OCoWAZc4DbkDxFty8w5uwMX3v6AP9/epgsUjS9+8pPOVZAMBikTCJS3/JbyuOAsdRLwwNkfU8uiyL44ZBvmztLQq0cJwcjzQ/xOrgoIoFigLbVt9pjaU17509ixACx/d5eVsLRJiiyGGjExEOc0Jr/r14Ub4SNA/EgR4h+GV4mEMHDjAciTBkGGQ20TpDO0OhTl5teJh4KMSvWvOUYRAPnIgSCXkTeAto1JoPgSv799OUz9NjWUysB18bEyzT0xClxTD40/M4UiwSNk3iMzPyn8CJqOQpklqTKCH9ZJq8fe4cq4uLEI3Sms3ypdY8v/4SEU6cPy/Hq1ILXJcJIF0aIL5POhbj/cZGPlhZ4ffSy4VgdjXLx2V11uU6tuOox3yfn9eS7vo6fXpjTmxr49npaZmuaj8wPi5nheCjnfjCYT4p9/KKW7JEQn4O/LjNoDJ14YJM1rQptSziwAbPNgxuRKOM1rwrHhuTfwux8SLTZHRqSi7UZTBJJOQPwOR6RQvxzcyM/L7ew+kR4CXTRHje5pWu5iuZVM/s3at6dyPjP1yoK/XOy4qhAAAAAElFTkSuQmCC",
              "cardano" => "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t8Y+Th+AFHW1fAMcHAKjecpe93h3Z2tv/55p9PcDpKVyu4IiMZUAAA12aVRYdFhNTDpjb20uYWRvYmUueG1wAAAAAAA8P3hwYWNrZXQgYmVnaW49Iu+7vyIgaWQ9Ilc1TTBNcENlaGlIenJlU3pOVGN6a2M5ZCI/Pgo8eDp4bXBtZXRhIHhtbG5zOng9ImFkb2JlOm5zOm1ldGEvIiB4OnhtcHRrPSJYTVAgQ29yZSA0LjQuMC1FeGl2MiI+CiA8cmRmOlJERiB4bWxuczpyZGY9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkvMDIvMjItcmRmLXN5bnRheC1ucyMiPgogIDxyZGY6RGVzY3JpcHRpb24gcmRmOmFib3V0PSIiCiAgICB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIKICAgIHhtbG5zOnN0RXZ0PSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VFdmVudCMiCiAgICB4bWxuczpkYz0iaHR0cDovL3B1cmwub3JnL2RjL2VsZW1lbnRzLzEuMS8iCiAgICB4bWxuczpHSU1QPSJodHRwOi8vd3d3LmdpbXAub3JnL3htcC8iCiAgICB4bWxuczp0aWZmPSJodHRwOi8vbnMuYWRvYmUuY29tL3RpZmYvMS4wLyIKICAgIHhtbG5zOnhtcD0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wLyIKICAgeG1wTU06RG9jdW1lbnRJRD0iZ2ltcDpkb2NpZDpnaW1wOjJkOWJhMWJjLTU5ODUtNGI2YS04NWMxLWNiNDU3MmY1Y2EyOCIKICAgeG1wTU06SW5zdGFuY2VJRD0ieG1wLmlpZDpiMzMzZmM1ZS1kZTJjLTRkNDEtOGYwNy1mMmVlYzgzOWUwNGEiCiAgIHhtcE1NOk9yaWdpbmFsRG9jdW1lbnRJRD0ieG1wLmRpZDplZDFjN2Q0OC01MmEwLTRhY2UtYWI0Ni1mOWQwOTk5ZGE0OGMiCiAgIGRjOkZvcm1hdD0iaW1hZ2UvcG5nIgogICBHSU1QOkFQST0iMi4wIgogICBHSU1QOlBsYXRmb3JtPSJXaW5kb3dzIgogICBHSU1QOlRpbWVTdGFtcD0iMTY5NDU0NTY4Nzg3MTk2NSIKICAgR0lNUDpWZXJzaW9uPSIyLjEwLjMyIgogICB0aWZmOk9yaWVudGF0aW9uPSIxIgogICB4bXA6Q3JlYXRvclRvb2w9IkdJTVAgMi4xMCIKICAgeG1wOk1ldGFkYXRhRGF0ZT0iMjAyMzowOToxMlQxMzowODowNy0wNjowMCIKICAgeG1wOk1vZGlmeURhdGU9IjIwMjM6MDk6MTJUMTM6MDg6MDctMDY6MDAiPgogICA8eG1wTU06SGlzdG9yeT4KICAgIDxyZGY6U2VxPgogICAgIDxyZGY6bGkKICAgICAgc3RFdnQ6YWN0aW9uPSJzYXZlZCIKICAgICAgc3RFdnQ6Y2hhbmdlZD0iLyIKICAgICAgc3RFdnQ6aW5zdGFuY2VJRD0ieG1wLmlpZDoxMGY3MGJmYy04OTJhLTQzNTktOWRhNi1lMTdkMmNkOTNkMTkiCiAgICAgIHN0RXZ0OnNvZnR3YXJlQWdlbnQ9IkdpbXAgMi4xMCAoV2luZG93cykiCiAgICAgIHN0RXZ0OndoZW49IjIwMjMtMDktMTJUMTM6MDg6MDciLz4KICAgIDwvcmRmOlNlcT4KICAgPC94bXBNTTpIaXN0b3J5PgogIDwvcmRmOkRlc2NyaXB0aW9uPgogPC9yZGY6UkRGPgo8L3g6eG1wbWV0YT4KICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgIAo8P3hwYWNrZXQgZW5kPSJ3Ij8+P3y/GgAAAAZiS0dEAP8A/wD/oL2nkwAAAAlwSFlzAAALEwAACxMBAJqcGAAAAAd0SU1FB+cJDBMIB8iOXtEAAALnSURBVFjD3ZdLSJVBFMd/hkHeRYty0YMem7BbUNlUO4tKslpENRGCkhGlbcIHEbSSoCh6eF300h4EXVOEySgio9ootEiHiiCDVi7F2mnZQ2zRuTV83jvffbjIDnx8M+c535z/nDkfTGtS5jzKnM7FxYyQAGUo89GjsRaIeuyHUWadL0R+yAItcCq1VJeG2NcD75n+pEwlyvSgzPI0dBXK9KNMZTqu89Ncwn6gBCgGPkigQqBH5BuwekTGUUCJfjw3EP6lA8AWrG53eEslWBRY7OAiDqyX/Ge81a0ocyYD/TqUqctAP4YyLb4UFAGRECc7gN1AG1Y3C28zUA50YXW3x3pOcNfzQgEFR4CzWD2IMhWBvJbJgrsc3j6sNihTDBwFbmJ1X7YgjAmYFgC7gJ0BuU5isw0wwAVgq2BkY7YL6BAHnTJ/l6RQzQrwBuR9B5gL3PYFyMsAQAuBceCwg4HYHzD+PqqPgFtAAVYPTmUhakaZCXmqPXo1jt71yfL7MyfXAWWaBDCJKjaKMlccp0VAbQAXPswkqAZloo6fuzAxhjIlTqy2GcA852hE5JntKVa+4jURmI8741FgDPgi85/AULJtLEjCu+ZsbY0nBdUoMy56TelkNxMQLgG+Cgj3AO1Y3SSyeqACeADckJI8lPsClDkI1AEXsTqOMg3AJUejFvgGuIA7idXn5DaMAY1YfTXby+gQsBqockq1SyulK3JplXODFkqJzvo2PA60Aidk/jwgf+IUqQQ9lvcxoCXsVswLbPk94AdWV3nSsl0w0InVL4RXIl/6EKufemzjwCKs3pSqFI8CI/4uUXcD3eKwAfiO1ZeB3jQwN+gcwwxOgTIRYAVW9zu8vXLpILIBR1YKvMbqz1PVEcWAvkCf1yPt2Vv5skTwcuAZ0DaVPWGv9HkDTio+pfgneCP6Hf9FOx5Jt732tPOFuWBgjXQ2qQK8QhnjsW+U1GWJAatfAvNDft2GPfbL/vks/wIuPeijyMLzBQAAAABJRU5ErkJggg==",
              _ => "", 
          };
          let decoded =  custom_engine.decode(watermark).ok()?;
          let cursor = Cursor::new(decoded);
          ImageReader::with_format(cursor, image::ImageFormat::Png)
              .decode()
              .ok()
      }
  }

  fn adjust_alpha(image: &mut RgbaImage, custom_opacity: u8) {
      for pixel in image.pixels_mut() {
          if pixel[3] == 0 {
              pixel[3] = custom_opacity;
          }
      }
      let mut found_fully_transparent = false;
      for pixel in image.pixels() {
          if pixel[3] == 0 {
              found_fully_transparent = true;
              break;
          }
      }
      if !found_fully_transparent {
          for pixel in image.pixels_mut() {
              pixel[3] = custom_opacity;
          }
      }
  }


pub fn create_img(ciphertext: &str, style: &str, watermark: &str, r: Option<u8>, g: Option<u8>, b: Option<u8>, a: Option<u8>, w: Option<u32>, h: Option<u32>) -> Option<String> {
    let custom_engine: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
    let r = r.unwrap_or(100);
    let g = g.unwrap_or(134);
    let b = b.unwrap_or(131);
    let width = ciphertext.len() as u32;
    let height = width;
    let mut img: RgbaImage = image::ImageBuffer::new(width, height);
    let last_column = ciphertext.chars().last();
    let shifted_ciphertext = if let Some(last) = last_column {
        last.to_string() + &ciphertext[..width as usize - 1]
    } else {
        ciphertext.to_string()
    };

    for x in 0..width {
        let char = shifted_ciphertext.chars().nth(x as usize).unwrap_or('a');
        let color = get_color(char).unwrap_or((0, 0, 0));
        for y in 0..height {
            let red = if y == 0 {
                color.0
            } else {
                (color.0 as i32 - (y as i32 + r as i32)).abs().min(255) as u8
            };
            let green = if y == 0 {
                color.1
            } else {
                (color.1 as i32 - (y as i32 + g as i32)).abs().min(255) as u8
            };
            let blue = if y == 0 {
                color.2
            } else {
                (color.2 as i32 - (y as i32 + b as i32)).abs().min(255) as u8
            };
            let rgba_color = Rgba([red, green, blue, 255]);
            img.put_pixel(x as u32, y, rgba_color);
        }
    }

    let mut row_count = 0;
    for y in 0..height {
        let contains_white = (0..width).any(|x| {
            let pixel = img.get_pixel(x, y);
            pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255
        });
        if contains_white && y != 0 {
              row_count += 1;
        }
    }

    let offset = if row_count > 10 { height / row_count } else { 1 };
    let mut new_img = image::ImageBuffer::new(width, height);
    let mut row_shift = 0;

    for y in 0..height {
      if (y as u32) % offset == 0 {
        row_shift += 1;
      }
      for x in 0..width {
        if (y as u32) >= row_shift && y != 0 {
            let pixel = *img.get_pixel(x, y - row_shift);
            new_img.put_pixel(x, y, pixel);
        } else {
            let pixel = *img.get_pixel(x, y);
            new_img.put_pixel(x, y, pixel);
        }
      }
    }
    match style {
        "v" => {
            new_img = imageops::rotate90(&new_img);
        },
        "v2" => {
            new_img = imageops::rotate270(&new_img);
            new_img = imageops::flip_vertical(&new_img);
        },
        "h" => {
        },
        "h2" => {
            new_img = imageops::flip_vertical(&new_img);
        },
        _ => { /* Default, no change */ }
    }

    let (alpha, center_w, center_h) = if custom_engine.decode(watermark).ok().is_some() {
        if let (Some(a), Some(w), Some(h)) = (a, w, h) {
            (Some(a), Some(w), Some(h))
        } else {
            return None;
        }
    } else {
        (Some(0), Some(32), Some(32))
    };

    let watermark_img = load_watermark(watermark, alpha, center_w, center_h);
    if let Some(watermark_img) = watermark_img {
        let nw = (width / 2) - (center_w.unwrap() / 2);
        let nh = (height / 2) - (center_h.unwrap() / 2);
        let mut watermark_img = watermark_img.to_rgba8();
        adjust_alpha(&mut watermark_img, alpha.unwrap_or(0));
        image::imageops::overlay(&mut new_img, &watermark_img, nw, nh);
    }

    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    let dyn_img: DynamicImage = DynamicImage::ImageRgba8(new_img.clone());
    if let Err(err) = encoder.encode(&dyn_img.to_rgba8(), width, height, ColorType::Rgba8) {
      eprintln!("Error encoding image: {:?}", err);
      return None;
    }

    let encoded_image = custom_engine.encode(&buf);
    Some(encoded_image)
}

/*
  pub fn create_img(ciphertext: &str, style: &str, watermark: &str, r: Option<u8>, g: Option<u8>, b: Option<u8>, a: Option<u8>, w: Option<u32>, h: Option<u32>) -> Option<String> {
    let custom_engine: engine::GeneralPurpose = engine::GeneralPurpose::new(&alphabet::STANDARD, general_purpose::PAD);
    let r = r.unwrap_or(100);
    let g = g.unwrap_or(134);
    let b = b.unwrap_or(131);
    let width = ciphertext.len() as u32;
    let height = width;
    let mut img: RgbaImage = image::ImageBuffer::new(width, height);
    let last_column = ciphertext.chars().last();
    let shifted_ciphertext = if let Some(last) = last_column {
      last.to_string() + &ciphertext[..width as usize - 1]
    } else {
      ciphertext.to_string()
    };
    for x in 0..width {
      let char = shifted_ciphertext.chars().nth(x as usize).unwrap_or('a');
      let color = get_color(char).unwrap_or((0, 0, 0));
      for y in 0..height {
        let red = if y == 0 {
          color.0
        } else {
          (color.0 as i32 - (y as i32 + r as i32)).abs().min(255) as u8
        };
        let green = if y == 0 {
          color.1
        } else {
          (color.1 as i32 - (y as i32 + g as i32)).abs().min(255) as u8
        };
        let blue = if y == 0 {
          color.2
        } else {
          (color.2 as i32 - (y as i32 + b as i32)).abs().min(255) as u8
        };
        let rgba_color = Rgba([red, green, blue, 255]);
        img.put_pixel(x as u32, y, rgba_color);
      }
    }
    let mut buf = Vec::new();
    let encoder = PngEncoder::new(&mut buf);
    let dyn_img: DynamicImage = DynamicImage::ImageRgba8(img.clone());
    encoder
        .encode(&dyn_img.to_rgba8(), width, height, ColorType::Rgba8)
        .ok()?;

    let mut row_count = 0;
    for y in 0..height {
        let contains_white = (0..width).any(|x| {
            let pixel = img.get_pixel(x, y);
            pixel[0] == 255 && pixel[1] == 255 && pixel[2] == 255
        });
        if contains_white && y != 0 {
              row_count += 1;
        }
    }

    let offset = if row_count > 10 { height / row_count } else { 1 };
    let mut new_img = image::ImageBuffer::new(width, height);
    let mut row_shift = 0;

    for y in 0..height {
      if (y as u32) % offset == 0 {
        row_shift += 1;
      }
      for x in 0..width {
        if (y as u32) >= row_shift && y != 0 {
            let pixel = *img.get_pixel(x, y - row_shift);
            new_img.put_pixel(x, y, pixel);
        } else {
            let pixel = *img.get_pixel(x, y);
            new_img.put_pixel(x, y, pixel);
        }
      }
    }


    let (alpha, center_w, center_h) = if custom_engine.decode(watermark).ok().is_some() {
        if let (Some(a), Some(w), Some(h)) = (a, w, h) {
            (Some(a), Some(w), Some(h))
        } else {
            return None;
        }
    } else {
        (Some(0), Some(32), Some(32))
    };

    let watermark_img = load_watermark(watermark, alpha, center_w, center_h);
    if let Some(watermark_img) = watermark_img {
        let nw = (width / 2) - (center_w.unwrap() / 2);
        let nh = (height / 2) - (center_h.unwrap() / 2);
        let mut watermark_img = watermark_img.to_rgba8();
        adjust_alpha(&mut watermark_img, alpha.unwrap_or(0));
        image::imageops::overlay(&mut new_img, &watermark_img, nw, nh);
    }

    let mut buf = Vec::new();  // Define buf here.
                               
    let encoder = PngEncoder::new(&mut buf);
        match style {
        "v" => {
            new_img = imageops::rotate90(&new_img);
        },
        "v2" => {
            new_img = imageops::rotate270(&new_img);
            new_img = imageops::flip_vertical(&new_img);
        },
        "h" => {
        },
        "h2" => {
            new_img = imageops::flip_vertical(&new_img);
        },
        _ => { /* Default, no change */ }
    }
    let dyn_img: DynamicImage = DynamicImage::ImageRgba8(new_img);  // Use new_img.
    if let Err(err) = encoder.encode(&dyn_img.to_rgba8(), width, height, ColorType::Rgba8) {
      eprintln!("Error encoding image: {:?}", err);
      return None; // Handle the error more explicitly.
    }
    let encoded_image =  custom_engine.encode(&buf);
    Some(encoded_image)
  }
*/
