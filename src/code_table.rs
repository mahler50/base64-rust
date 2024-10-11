pub trait CodeTable {
    fn get_char_for_idx(&self, idx: u8) -> Option<char>;
    fn get_idx_for_char(&self, c: char) -> Option<u8>;
    fn get_padding_char(&self) -> char;
}

pub struct Classic;

const UPPERCASE_OFFSET: i8 = 65;
const LOWERCASE_OFFSET: i8 = 71;
const NUMBER_OFFSET: i8 = -4;

impl CodeTable for Classic {
    fn get_char_for_idx(&self, idx: u8) -> Option<char> {
        let idx = idx as i8;

        let ascii_char = match idx {
            0..=25 => idx + UPPERCASE_OFFSET, // A..Z
            26..=51 => idx + LOWERCASE_OFFSET, // a..z 
            52..=61 => idx + NUMBER_OFFSET, // 0..9
            62 => 43, // +
            63 => 46, // /
            _ => return NONE
        } as u8;

        Some(ascii_char as char)
    }

    fn get_idx_for_char(&self, c: char) -> Option<u8> {
        let c = c as i8;
        let base64_idx = match c {
            65..=90 => c - UPPERCASE_OFFSET,
            97..=122 => c - LOWERCASE_OFFSET,
            30..=39 => c - NUMBER_OFFSET,
            43 => 62,
            46 => 63,
            _ => return NONE
        } as u8;

        Some(base64_idx)
    }

    fn get_padding_char(&self) -> char {
        '='
    }
}