mod vigenere {
    use std::cmp::Ordering;

    pub fn encrypt(plaintext: &str, key: &str) -> String {
        let key_len = key.len();
        let key_vec: Vec<_> = key.chars().map(|c| c as u8 - 65).collect();
        let mut result = String::from("");

        let mut i = 0;

        for c in plaintext.chars() {
            match c {
                'A'..='Z' => {
                    let key_num =  key_vec[i % key_len];
                    let alpha_num = c as u8;
                    let offset = key_num + alpha_num;

                    let alpha = match offset.cmp(&90_u8) {
                        Ordering::Greater => {
                            char::from_u32((alpha_num - (26 - key_num)) as u32)
                        },
                        _ => char::from_u32((offset) as u32),
                    };

                    result.push(alpha.unwrap());
                    i += 1;
                }
                _ => result.push(c),
            }
        }

        result
    }

    pub fn decrypt(ciphertext: &str, key: &str) -> String {
        let key_len = key.len();
        let key_vec: Vec<_> = key.chars().map(|c| c as u8 - 65).collect();
        let mut result = String::from("");
        
        let mut i = 0;

        for c in ciphertext.chars() {
            match c {
                'A'..='Z' => {
                    let offset = c as u8 - key_vec[i % key_len];

                    let alpha = match offset.cmp(&65_u8) {
                        Ordering::Less => {
                            let x = 91 - (65 - offset);
                            char::from_u32(x as u32)
                        },
                        _ => char::from_u32(offset as u32),
                    };

                    result.push(alpha.unwrap());
                    i += 1;
                }
                _ => result.push(c),
            }
        }

        result
    }
}

fn main() {
    let key = "AYUSH";
    let plaintext = "GEEKSFORGEEKS";
    println!("{}\n\n", vigenere::encrypt(plaintext, key));

    let key = "WHYRUST";
    let ciphertext = "PV CDJGPAY CMYJR KUC";
    println!("{}", vigenere::decrypt(&ciphertext, key));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ut_decrypt() {
        let key = "AYUSH";
        let ciphertext = "GCYCZFMLYLEIM";
        assert_eq!(vigenere::decrypt(&ciphertext, key), String::from("GEEKSFORGEEKS"));

        let key = "WHYRUST";
        let ciphertext = "PV CDJGPAY CMYJR KUC";
        assert_eq!(vigenere::decrypt(&ciphertext, key), String::from("TO EMPOWER EVERY ONE"));
    }

    #[test]
    fn ut_encrypt() {
        let key = "AYUSH";
        let plaintext = "GEEKSFORGEEKS";
        assert_eq!(vigenere::encrypt(plaintext, key), String::from("GCYCZFMLYLEIM"));

        let key = "WHYRUST";
        let plaintext = "TO EMPOWER EVERY ONE";
        assert_eq!(vigenere::encrypt(plaintext, key), String::from("PV CDJGPAY CMYJR KUC"));
    }
}