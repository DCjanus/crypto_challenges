use std::collections::HashMap;
use std::sync::LazyLock;

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

// https://en.wikipedia.org/wiki/Letter_frequency
const CHAR_FREQ_RAW: [(u8, f64); 26] = [
    (b'a', 0.08167),
    (b'b', 0.01492),
    (b'c', 0.02782),
    (b'd', 0.04253),
    (b'e', 0.12702),
    (b'f', 0.02228),
    (b'g', 0.02015),
    (b'h', 0.06094),
    (b'i', 0.06966),
    (b'j', 0.00153),
    (b'k', 0.00772),
    (b'l', 0.04025),
    (b'm', 0.02406),
    (b'n', 0.06749),
    (b'o', 0.07507),
    (b'p', 0.01929),
    (b'q', 0.00095),
    (b'r', 0.05987),
    (b's', 0.06327),
    (b't', 0.09056),
    (b'u', 0.02758),
    (b'v', 0.00978),
    (b'w', 0.02360),
    (b'x', 0.00150),
    (b'y', 0.01974),
    (b'z', 0.00074),
];
static CHAR_FREQ: LazyLock<HashMap<u8, f64>> = LazyLock::new(|| {
    let mut o = HashMap::new();
    for (k, v) in CHAR_FREQ_RAW.iter() {
        o.insert(*k, *v);
        o.insert(k.to_ascii_uppercase(), *v);
    }
    // code from https://github.com/anneouyang/cryptopals/blob/master/solutions/challenge3.py
    // I don't know why the space is 0.13000
    o.insert(b' ', 0.13000);
    o
});

pub fn char_freq_score(input: impl Iterator<Item = u8>) -> f64 {
    input
        .map(|byte| CHAR_FREQ.get(&byte).copied().unwrap_or(0.0))
        .sum()
}
