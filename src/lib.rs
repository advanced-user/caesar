#[derive(Clone, Copy)]
pub enum Language {
    English,
    Ukrainian,
}

impl Language {
    pub fn get_alph_uppercase(&self) -> Vec<char> {
        match self {
            Language::English => "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect(),
            Language::Ukrainian => "АБВГҐДЕЄЖЗИІЇЙКЛМНОПРСТУФХЦЧШЩЬЮЯ".chars().collect(),
        }
    }

    pub fn get_alph_lowercase(&self) -> Vec<char> {
        match self {
            Language::English => "abcdefghijklmnopqrstuvwxyz".chars().collect(),
            Language::Ukrainian => "абвгґдеєжзиіїйклмнопрстуфхцчшщьюя".chars().collect(),
        }
    }
}

pub fn caesar_encrypt(text: &str, key: i32, lang: Language) -> String {
    let uppercase = lang.get_alph_uppercase();
    let lowercase = lang.get_alph_lowercase();

    text.chars()
    .map(|c| {
        let alphabet = if lowercase.contains(&c) {
            &lowercase
        } else if uppercase.contains(&c) {
            &uppercase
        } else {
            return c;
        };

        if let Some(pos) = alphabet.iter().position(|&letter| letter == c) {
            let len = alphabet.len() as i32;
            let mut shifted_pos = pos as i32 + key;

            if shifted_pos < 0 {
                shifted_pos += len;
            } else if shifted_pos >= len {
                shifted_pos -= len;
            }

            return alphabet[shifted_pos as usize];
        }

        return c;
    })
    .collect()
}

pub fn caesar_decrypt(text: &str, key: i32, lang: Language) -> String {
    caesar_encrypt(text, -key, lang)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_eng_text() {
        let plain_text = "aBcdEFgHiJkLmNoPqRSTuVWxYz";
        let key = 4;
        let encrypted_text = caesar_encrypt(&plain_text, key, Language::English);
        assert_eq!(encrypted_text, "eFghIJkLmNoPqRsTuVWXyZAbCd");
    }

    #[test]
    fn decrypt_eng_text() {
        let encrypted_text = "eFghIJkLmNoPqRsTuVWXyZAbCd";
        let key = 4;
        let dencrypted_text = caesar_decrypt(&encrypted_text, key, Language::English);
        assert_eq!(dencrypted_text, "aBcdEFgHiJkLmNoPqRSTuVWxYz");
    }

    #[test]
    fn encrypt_ukr_text() {
        let plain_text = "АбВгҐДеЄЖЗИіЇЙКлМНоПРСтУфХЦчШЩЬЮя";
        let key = 4;
        let encrypted_text = caesar_encrypt(&plain_text, key, Language::Ukrainian);
        assert_eq!(encrypted_text, "ҐдЕєЖЗиІЇЙКлМНОпРСтУФХцЧшЩЬюЯАБВг");
    }

    #[test]
    fn decrypt_ukr_text() {
        let encrypted_text = "ҐдЕєЖЗиІЇЙКлМНОпРСтУФХцЧшЩЬюЯАБВг";
        let key = 4;
        let dencrypted_text = caesar_decrypt(&encrypted_text, key, Language::Ukrainian);
        assert_eq!(dencrypted_text, "АбВгҐДеЄЖЗИіЇЙКлМНоПРСтУфХЦчШЩЬЮя");
    }
}
