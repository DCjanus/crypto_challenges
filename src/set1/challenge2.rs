use bytes::{BufMut, Bytes, BytesMut};

use crate::utils::{hex_decode_iter, hex_encode_iter, xor_iter};

/// https://cryptopals.com/sets/1/challenges/2
/// Fixed XOR
/// Write a function that takes two equal-length buffers and produces their XOR combination.
/// If your function works properly, then when you feed it the string:
///     1c0111001f010100061a024b53535009181c
/// ... after hex decoding, and when XOR'd against:
///     686974207468652062756c6c277320657965
/// ... should produce:
///     746865206b696420646f6e277420706c6179
#[test]
fn test() {
    struct TestCase {
        a: &'static str,
        b: &'static str,
        expected: &'static str,
    }

    let testcases = vec![TestCase {
        a: "1c0111001f010100061a024b53535009181c",
        b: "686974207468652062756c6c277320657965",
        expected: "746865206b696420646f6e277420706c6179",
    }];

    for t in testcases {
        let a = Bytes::from(t.a);
        let b = Bytes::from(t.b);
        let expected = Bytes::from(t.expected);
        let output = solution(a.clone(), b.clone());
        assert_eq!(output, expected, "a: {}, b: {}", t.a, t.b);
    }
}

#[allow(dead_code)]
fn solution(a: Bytes, b: Bytes) -> Bytes {
    assert_eq!(a.len(), b.len(), "a and b must have the same length");
    assert_eq!(a.len() % 2, 0, "a and b length must be even");

    let mut output = BytesMut::with_capacity(a.len());
    let a_iter = hex_decode_iter(a.into_iter());
    let b_iter = hex_decode_iter(b.into_iter());
    let x_iter = xor_iter(a_iter, b_iter);
    let o_iter = hex_encode_iter(x_iter);
    o_iter.for_each(|b| output.put_u8(b));

    output.freeze()
}
