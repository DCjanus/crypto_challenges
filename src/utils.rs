use itertools::Itertools;

pub fn hex_decode_byte(hex: u8) -> u8 {
    match hex {
        b'0'..=b'9' => hex - b'0',
        b'a'..=b'f' => hex - b'a' + 10,
        b'A'..=b'F' => hex - b'A' + 10,
        _ => panic!("invalid hex digit: {}", hex),
    }
}

pub fn hex_encode_byte(dig: u8) -> u8 {
    match dig {
        0..=9 => dig + b'0',
        10..=15 => dig - 10 + b'a',
        _ => panic!("invalid digit: {}", dig),
    }
}

pub fn hex_decode_iter(input: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    input
        .tuples()
        .map(|(a, b)| hex_decode_byte(a) * 16 + hex_decode_byte(b))
}

pub fn hex_encode_iter(input: impl Iterator<Item = u8>) -> impl Iterator<Item = u8> {
    input.flat_map(|b| {
        let a = b / 16;
        let b = b % 16;
        [hex_encode_byte(a), hex_encode_byte(b)].into_iter()
    })
}

pub fn xor_iter(
    a: impl Iterator<Item = u8>,
    b: impl Iterator<Item = u8>,
) -> impl Iterator<Item = u8> {
    a.zip(b).map(|(a, b)| a ^ b)
}
