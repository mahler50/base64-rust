use std::{io, vec};

use crate::code_table::{Classic, CodeTable};

pub fn decode(data: &String) -> Result<Vec<u8>, io::Error> {
    let code_table = Classic;
    decode_using_code_table(code_table, data)
}

pub fn decode_using_code_table<T: CodeTable>(code_table: T, data: &String) -> Result<Vec<u8>, io::Error> {
    if data.chars().count() % 4 != 0 {
        return Err(io::Error::from(io::ErrorKind::InvalidInput));
    }

    let result = data
    .chars()
    .collect::<Vec<char>>()
    .chunks(4)
    .map(|chunk| original(&code_table, chunk))
    .flat_map(recombination)
    .collect();

    Ok(result)
}

fn original<T: CodeTable>(code_table: &T, chunk: &[char]) -> Vec<u8> {
    chunk
    .iter()
    .filter(|c| **c != code_table.get_padding_char())
    .map(|c| {
        code_table.get_idx_for_char(*c).expect("unable to find char in code table")
    }).collect()
}

fn recombination(bytes: Vec<u8>) -> Vec<u8> {
    let output = match bytes.len() {
        2 => vec![
            (&bytes[0] & 0b111111) << 2 | &bytes[1] >> 4,
            (&bytes[1] & 0b1111) << 4
        ],
        3 => vec![
            (&bytes[0] & 0b111111) << 2 | &bytes[1] >> 4,
            (&bytes[1] & 0b1111) << 4 | &bytes[2] >> 2,
            (&bytes[2] & 0b11) << 6
        ],
        4 => vec![
            (&bytes[0] & 0b111111) << 2 | &bytes[1] >> 4,
            (&bytes[1] & 0b1111) << 4 | &bytes[2] >> 2,
            (&bytes[2] & 0b11) << 6 | &bytes[3] & 0b111111
        ],
        _ => unreachable!()
    };

    output.into_iter().filter(|&x| x > 0).collect()
}

#[cfg(test)]
mod test {
    use crate::encoder::encode;

    use super::*;

    #[test]
    fn test_encode_and_decode() {
        let s = String::from("Hello base64!");
        let encoded = encode(s.as_bytes());
        assert_eq!(s.as_bytes(), decode(&encoded).unwrap());
    }

}