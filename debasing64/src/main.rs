use std::io;

trait Decoder {
    fn decode(&self, text: String) -> String;
}

struct Base64Decoder {}

impl Base64Decoder {
    fn new() -> Self {
        Base64Decoder {}
    }
}

impl Decoder for Base64Decoder {
    fn decode(&self, text: String) -> String {
        let to_base64 = [
            'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q',
            'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
            'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y',
            'z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '+', '/',
        ];

        // removing padding from the input text
        let text_paddingless: Vec<char> = text.replace("=", "").chars().collect();

        // Convert each Base64 char to its corresponding number
        // very bad brute force way of doing it, is as follows!
        let mut six_group_bits = String::new();

        for k in 0..text_paddingless.len() {
            for i in 0..64 {
                let to_base64_char = to_base64.get(i).unwrap();
                let text_char = text_paddingless.get(k).unwrap();
                if to_base64_char == text_char {
                    let current = format!("{:06b}", i);
                    six_group_bits.push_str(&current);
                    break;
                }
            }
        }

        let mut decoded: String = String::new();
        let six_group_bits = six_group_bits.chars().collect::<Vec<char>>();
        let mut i = 0;

        loop {
            if i + 8 > six_group_bits.len() {
                break;
            }
            let mut k = 0;
            let mut current_window = String::new();
            loop {
                if k >= 8 || i + k >= six_group_bits.len() {
                    break;
                }
                current_window.push(*six_group_bits.get(i + k).unwrap());
                k += 1;
            }
            i += k;

            let num = isize::from_str_radix(&current_window, 2).unwrap() as u32;
            let current_char = char::from_u32(num).unwrap();

            decoded.push(current_char);
        }

        decoded
    }
}

fn main() {
    let decoder = Base64Decoder::new();
    for _ in 0..16 {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("failed to read input");
        println!("{}", decoder.decode(inp));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let decoder = Base64Decoder::new();
        assert_eq!("Logto", decoder.decode("TG9ndG8=".to_string()));
    }
}
