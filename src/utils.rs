use itertools::Itertools;

pub fn decode_hex_byte(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => panic!("invalid hex digit: {}", hex),
    }
}

pub fn encode_hex_byte(dig: u8) -> u8 {
    match dig {
        0..=9 => dig + b'0',
        10..=15 => dig - 10 + b'a',
        _ => panic!("invalid digit: {}", dig),
    }
}

pub fn hex_decode_iter(input: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    input
        .tuples()
        .map(|(a, b)| decode_hex_byte(a) * 16 + decode_hex_byte(b))
}
