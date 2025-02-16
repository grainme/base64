use std::io;

trait Decoder {
    fn decode(&self, text: &str) -> Result<String, String>;
}

struct Base64Decoder;

impl Base64Decoder {
    fn new() -> Self {
        Base64Decoder
    }
}

impl Decoder for Base64Decoder {
    fn decode(&self, text: &str) -> Result<String, String> {
        let base64_chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

        let text = text.trim().replace("=", "");

        let mut six_group_bits = String::new();

        for char in text.chars() {
            base64_chars
                .find(char)
                .ok_or_else(|| format!("Invalid character found in base64 input: {}", char))?
                .apply(|index| {
                    six_group_bits.push_str(&format!("{:06b}", index));
                });
        }

        six_group_bits
            .as_bytes()
            .chunks(8)
            .map(|chunk| {
                let byte_str = String::from_utf8_lossy(chunk);
                u8::from_str_radix(&byte_str, 2)
                    .map_err(|_| "Failed to parse byte from binary string".to_string())
                    .map(|num| num as char)
            })
            .collect::<Result<String, _>>()
    }
}

fn main() {
    let decoder = Base64Decoder::new();

    for _ in 0..16 {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        match decoder.decode(input) {
            Ok(decoded) => println!("{}", decoded),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let decoder = Base64Decoder::new();
        assert_eq!("Logto", decoder.decode("TG9ndG8=").unwrap());
    }

    #[test]
    fn test_invalid_character() {
        let decoder = Base64Decoder::new();
        assert_eq!(
            Err("Invalid character found in base64 input: #".to_string()),
            decoder.decode("TG9ndG8#")
        );
    }
}
