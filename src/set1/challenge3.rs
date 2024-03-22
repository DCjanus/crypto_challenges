use std::iter::repeat;

use bytes::{Bytes, BytesMut};

use crate::utils::{char_freq_score, hex_decode_iter, xor_iter};

/// https://cryptopals.com/sets/1/challenges/3
/// Single-byte XOR cipher
/// The hex encoded string:
///     1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736
/// ... has been XOR'd against a single character. Find the key, decrypt the message.
/// You can do this by hand. But don't: write code to do it for you.
#[test]
fn test() {
    struct TestCase {
        input: &'static str,
        expected: &'static str,
    }

    let testcases = vec![TestCase {
        input: "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736",
        expected: "Cooking MC's like a pound of bacon",
    }];

    for t in testcases {
        let input = Bytes::from(t.input);
        let expected = Bytes::from(t.expected);
        let output = solution(input.clone());
        assert_eq!(output, expected, "input: {}", t.input);
    }
}

#[allow(dead_code)]
fn solution(input: Bytes) -> Bytes {
    assert_eq!(input.len() % 2, 0, "input length must be even");
    let mut output = BytesMut::with_capacity(input.len() / 2);

    let mut max_score = -1f64;
    let mut max_key = 0;

    let hex_decoded = BytesMut::from_iter(hex_decode_iter(input.into_iter())).freeze();

    for key in 0..=255 {
        let x_iter = xor_iter(hex_decoded.clone().into_iter(), repeat(key));
        let score = char_freq_score(x_iter);
        if score > max_score {
            max_score = score;
            max_key = key;
        }
    }

    let o_iter = xor_iter(hex_decoded.into_iter(), repeat(max_key));
    output.extend(o_iter);
    output.freeze()
}
