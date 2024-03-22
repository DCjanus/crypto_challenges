use bytes::{Bytes, BytesMut};
use bytes::BufMut;
use itertools::Itertools;

use crate::utils::hex_decode_iter;

/// https://cryptopals.com/sets/1/challenges/1
/// Convert hex to base64
/// The string:
///     49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
/// Should produce:
///     SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t
/// So go ahead and make that happen. You'll need to use this code for the rest of the exercises.
#[test]
fn test() {
    struct TestCase {
        input: &'static str,
        expected: &'static str,
    }

    let testcases = vec![
        TestCase { // test case with no padding
            input: "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d",
            expected: "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
        },
        TestCase { // test case with one padding
            input: "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f",
            expected: "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb28=",
        },
        TestCase { // test case with two padding
            input: "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f",
            expected: "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ubw==",
        },
    ];

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
    let byte_len = input.len() / 2;
    let byte_iter = hex_decode_iter(input.into_iter());
    let base64_len = (byte_len * 4 + 2) / 3;

    let mut output = BytesMut::with_capacity(base64_len);
    let mut buf = Vec::with_capacity(3);
    for chunk in &byte_iter.chunks(3) {
        buf.clear();
        buf.extend(chunk);
        digit_to_base64_part(&mut output, &buf);
    }

    output.freeze()
}

pub static BASE64_TABLE: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn digit_to_base64_part(out: &mut BytesMut, input: &[u8]) {
    match input {
        [a, b, c] => {
            out.put_u8(BASE64_TABLE[(a >> 2) as usize]);
            out.put_u8(BASE64_TABLE[((a & 0b11) << 4 | (b >> 4)) as usize]);
            out.put_u8(BASE64_TABLE[((b & 0b1111) << 2 | (c >> 6)) as usize]);
            out.put_u8(BASE64_TABLE[(c & 0b111111) as usize]);
        }
        [a, b] => {
            out.put_u8(BASE64_TABLE[(a >> 2) as usize]);
            out.put_u8(BASE64_TABLE[((a & 0b11) << 4 | (b >> 4)) as usize]);
            out.put_u8(BASE64_TABLE[((b & 0b1111) << 2) as usize]);
            out.put_u8(b'=');
        }
        [a] => {
            out.put_u8(BASE64_TABLE[(a >> 2) as usize]);
            out.put_u8(BASE64_TABLE[((a & 0b11) << 4) as usize]);
            out.put_u8(b'=');
            out.put_u8(b'=');
        }
        _ => panic!("invalid input: {:?}", input),
    }
}
