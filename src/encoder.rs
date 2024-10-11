use crate::code_table::{CodeTable, Classic};

pub fn encode(data: &[u8]) -> String {
    let code_table = Classic;
    encode_using_code_table(&code_table, data)
}

pub fn encode_using_code_table<T: CodeTable>(code_table: &T, data: &[u8]) -> String {
    let encoded = data
    .chunks(3)
    .map(split)
    .flat_map(|chunk| encode_chunk(code_table, chunk));

    String::from_iter(encoded)
}

fn split(chunk: &[u8]) -> Vec<u8>{
    match chunk.len() {
        1 => vec![&chunk[0] >> 2, (&chunk[0] & 0b11) << 4],
        2 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b11) << 4 | (&chunk[1] >> 4),
            (&chunk[1] & 0b1111) << 2
        ],
        3 => vec![
            &chunk[0] >> 2,
            (&chunk[0] & 0b11) << 4 | (&chunk[1] >> 4),
            (&chunk[1] & 0b1111) << 2 | (&chunk[2] >> 6),
            &chunk[2] & 0b111111
        ],
        _ => unreachable!()
    }
}

fn encode_chunk<T: CodeTable>(code_table: &T, chunk: Vec<u8>) -> Vec<char> {
    let mut output = vec![code_table.get_padding_char(); 4];

    for i in 0..chunk.len() {
        if let Some(c) = code_table.get_char_for_idx(chunk[i]) {
            output[i] = c;
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one_chunk() {
        let bytes: [u8; 1] = [65]; // A
        assert_eq!("QQ==", encode(&bytes));
    }

    #[test]
    fn test_two_chunks() {
        let bytes: [u8; 2] = [65, 66]; // AB
        assert_eq!("QUI=", encode(&bytes));
    }

    #[test]
    fn test_three_chunks() {
        let bytes: [u8; 3] = [65, 66, 67]; // ABC
        assert_eq!("QUJD", encode(&bytes));
    }

}