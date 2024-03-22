use bytes::{Bytes, BytesMut};

use crate::utils::{hex_encode_iter, xor_iter};

/// https://cryptopals.com/sets/1/challenges/5
/// Implement repeating-key XOR
/// Here is the opening stanza of an important work of the English language:
///     Burning 'em, if you ain't quick and nimble
///     I go crazy when I hear a cymbal
/// Encrypt it, under the key "ICE", using repeating-key XOR.
/// In repeating-key XOR, you'll sequentially apply each byte of the key; the first byte of plaintext will be XOR'd against I, the next C, the next E, then I again for the 4th byte, and so on.
/// It should come out to:
///     0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f
#[test]
fn test() {
    let input =
        Bytes::from("Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal");
    let key = Bytes::from("ICE");
    let expected = Bytes::from("0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f");
    let output = solution(input, key);
    assert_eq!(output, expected);
}

#[allow(dead_code)]
fn solution(input: Bytes, key: Bytes) -> Bytes {
    let mut output = BytesMut::with_capacity(input.len() * 2);
    let k_iter = key.iter().cycle();
    let o_iter = hex_encode_iter(xor_iter(input.into_iter(), k_iter.cloned()));
    output.extend(o_iter);
    output.freeze()
}
