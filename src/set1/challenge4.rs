use std::iter::repeat;

use bytes::{Bytes, BytesMut};

use crate::utils::{char_freq_score, hex_decode_iter, xor_iter};

/// https://cryptopals.com/sets/1/challenges/4
/// Detect single-character XOR
/// One of the 60-character strings in this file has been encrypted by single-character XOR.
/// Find it.
/// (Your code from #3 should help.)
#[test]
fn test() {
    let input = include_str!("./challenge4.txt");
    let expected = Bytes::from("Now that the party is jumping\n");
    let output = Solution::default().run(input);
    assert_eq!(output, expected);
}

#[derive(Default)]
struct Solution {
    line_decode_buf: BytesMut,
    xor_buf: BytesMut,

    best_score: f64,
    best_line: BytesMut,
}

impl Solution {
    #[allow(dead_code)]
    fn run(&mut self, input: &str) -> Bytes {
        self.best_score = -1.0;

        let lines = input.lines().map(|l| l.trim()).filter(|l| l.len() == 60);
        for line in lines {
            self.line_decode_buf.clear();
            self.line_decode_buf.extend(hex_decode_iter(line.bytes()));

            for key in 0..=255 {
                self.xor_buf.clear();
                let decode_iter = xor_iter(self.line_decode_buf.clone().into_iter(), repeat(key));
                self.xor_buf.extend(decode_iter);
                let score = char_freq_score(self.xor_buf.clone().into_iter());
                if score > self.best_score {
                    self.best_score = score;
                    self.best_line.clear();
                    self.best_line.extend_from_slice(&self.xor_buf);
                }
            }
        }
        self.best_line.clone().freeze()
    }
}
