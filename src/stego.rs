use crate::error::{Result, StegoError};

// Zero-width characters for encoding
const ZW_SPACE: char = '\u{200B}'; // Zero Width Space (represents 0)
const ZW_NON_JOINER: char = '\u{200C}'; // Zero Width Non-Joiner (represents 1)
const ZW_JOINER: char = '\u{200D}'; // Zero Width Joiner (byte separator)

/// Encode binary data as zero-width Unicode characters
fn bytes_to_zero_width(data: &[u8]) -> String {
    let mut result = String::new();

    for byte in data {
        // Encode each bit of the byte
        for i in (0..8).rev() {
            if (byte >> i) & 1 == 1 {
                result.push(ZW_NON_JOINER);
            } else {
                result.push(ZW_SPACE);
            }
        }
        result.push(ZW_JOINER); // Byte separator
    }

    result
}

/// Decode zero-width characters back to binary data
fn zero_width_to_bytes(encoded: &str) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    let mut current_byte: u8 = 0;
    let mut bit_count = 0;

    for ch in encoded.chars() {
        match ch {
            ZW_SPACE => {
                current_byte = (current_byte << 1) | 0;
                bit_count += 1;
            }
            ZW_NON_JOINER => {
                current_byte = (current_byte << 1) | 1;
                bit_count += 1;
            }
            ZW_JOINER => {
                if bit_count == 8 {
                    result.push(current_byte);
                    current_byte = 0;
                    bit_count = 0;
                }
            }
            _ => {} // Ignore visible characters
        }
    }

    if result.is_empty() {
        return Err(StegoError::NoDataFound);
    }

    Ok(result)
}

/// Calculate CRC32 checksum
fn crc32(data: &[u8]) -> u32 {
    crc32fast::hash(data)
}

/// Embed encrypted data into cover text using zero-width characters
pub fn embed(cover_text: &str, payload: &[u8]) -> Result<String> {
    // Prepend CRC32 checksum (4 bytes) to payload
    let checksum = crc32(payload);
    let mut data_with_checksum = checksum.to_le_bytes().to_vec();
    data_with_checksum.extend_from_slice(payload);

    let encoded = bytes_to_zero_width(&data_with_checksum);

    // Calculate capacity: we inject between each visible character
    let visible_chars: Vec<char> = cover_text.chars().collect();
    let injection_points = visible_chars.len().saturating_sub(1);
    let chars_per_byte = 9; // 8 bits + 1 separator
    let capacity = injection_points / chars_per_byte;

    if capacity < data_with_checksum.len() {
        return Err(StegoError::InsufficientCover {
            needed: data_with_checksum.len(),
            available: capacity,
        });
    }

    // Distribute zero-width chars evenly across the cover text
    let zw_chars: Vec<char> = encoded.chars().collect();
    let chunk_size = if injection_points > 0 {
        zw_chars.len() / injection_points
    } else {
        zw_chars.len()
    };

    let mut result = String::new();
    let mut zw_index = 0;

    for (i, ch) in visible_chars.iter().enumerate() {
        result.push(*ch);

        // Inject zero-width characters after each visible char (except last)
        if i < visible_chars.len() - 1 && zw_index < zw_chars.len() {
            let end = (zw_index + chunk_size + 1).min(zw_chars.len());
            for zw in &zw_chars[zw_index..end] {
                result.push(*zw);
            }
            zw_index = end;
        }
    }

    // Append any remaining zero-width chars
    for zw in &zw_chars[zw_index..] {
        result.push(*zw);
    }

    Ok(result)
}

/// Extract hidden data from text containing zero-width characters
pub fn extract(artifact: &str) -> Result<Vec<u8>> {
    let data_with_checksum = zero_width_to_bytes(artifact)?;

    if data_with_checksum.len() < 4 {
        return Err(StegoError::NoDataFound);
    }

    // Split checksum and payload
    let stored_checksum = u32::from_le_bytes([
        data_with_checksum[0],
        data_with_checksum[1],
        data_with_checksum[2],
        data_with_checksum[3],
    ]);
    let payload = &data_with_checksum[4..];

    // Verify integrity
    let computed_checksum = crc32(payload);
    if stored_checksum != computed_checksum {
        return Err(StegoError::IntegrityFailure);
    }

    Ok(payload.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_embedding() {
        // Cover text needs ~9 chars per payload byte (8 bits + separator)
        // For "SECRET" (6 bytes) + 4 bytes CRC32 = 10 bytes, we need ~90+ chars
        let cover = "This is a perfectly normal looking sentence that will contain some hidden \
                     data embedded within it using zero-width Unicode characters that are invisible.";
        let secret = b"SECRET";

        let artifact = embed(cover, secret).unwrap();
        let extracted = extract(&artifact).unwrap();

        assert_eq!(secret.as_slice(), extracted.as_slice());

        // Verify the visible text is preserved
        let visible: String = artifact
            .chars()
            .filter(|c| !matches!(*c, ZW_SPACE | ZW_NON_JOINER | ZW_JOINER))
            .collect();
        assert_eq!(visible, cover);
    }

    #[test]
    fn detects_corruption() {
        let cover = "This is a test sentence for corruption detection purposes here and we need \
                     enough text to actually embed the data properly so the test works correctly.";
        let secret = b"DATA";

        let mut artifact = embed(cover, secret).unwrap();

        // Corrupt by removing some zero-width characters
        artifact = artifact.replace(ZW_NON_JOINER, "");

        let result = extract(&artifact);
        assert!(result.is_err());
    }
}

