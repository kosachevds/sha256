use std::u64;
use std::u32;
use std::convert::TryInto;

const RESULT_LENGTH: usize = 32;
const CHUNK_BYTES_COUNT: usize = 512 / 8;
const CHUNK_WORDS_COUNT: usize = 16;

fn calculate(input: &[u8]) -> Vec<u8> {
    let mut h0: u32 = 0x6A09E667;
    let mut h1: u32 = 0xBB67AE85;
    let mut h2: u32 = 0x3C6EF372;
    let mut h3: u32 = 0xA54FF53A;
    let mut h4: u32 = 0x510E527F;
    let mut h5: u32 = 0x9B05688C;
    let mut h6: u32 = 0x1F83D9AB;
    let mut h7: u32 = 0x5BE0CD19;
    
    let k: [u32; 64] = [
        0x428A2F98, 0x71374491, 0xB5C0FBCF, 0xE9B5DBA5, 0x3956C25B, 0x59F111F1, 0x923F82A4, 0xAB1C5ED5,
        0xD807AA98, 0x12835B01, 0x243185BE, 0x550C7DC3, 0x72BE5D74, 0x80DEB1FE, 0x9BDC06A7, 0xC19BF174,
        0xE49B69C1, 0xEFBE4786, 0x0FC19DC6, 0x240CA1CC, 0x2DE92C6F, 0x4A7484AA, 0x5CB0A9DC, 0x76F988DA,
        0x983E5152, 0xA831C66D, 0xB00327C8, 0xBF597FC7, 0xC6E00BF3, 0xD5A79147, 0x06CA6351, 0x14292967,
        0x27B70A85, 0x2E1B2138, 0x4D2C6DFC, 0x53380D13, 0x650A7354, 0x766A0ABB, 0x81C2C92E, 0x92722C85,
        0xA2BFE8A1, 0xA81A664B, 0xC24B8B70, 0xC76C51A3, 0xD192E819, 0xD6990624, 0xF40E3585, 0x106AA070,
        0x19A4C116, 0x1E376C08, 0x2748774C, 0x34B0BCB5, 0x391C0CB3, 0x4ED8AA4A, 0x5B9CCA4F, 0x682E6FF3,
        0x748F82EE, 0x78A5636F, 0x84C87814, 0x8CC70208, 0x90BEFFFA, 0xA4506CEB, 0xBEF9A3F7, 0xC67178F2
    ];

    let input = preprocess(input);
    let chunk_size = 512 / 8;
    let chunk_begin = 0;
    // TODO: remade with chunks();
    while chunk_begin < input.len() {
        let chunk = &input[chunk_begin..(chunk_begin + chunk_size)];
        let mut words = chunk_to_be_words(chunk);
        let words = extend_words(&mut words);

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;

        for i in 0..63 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = overflowing_sum(&[h, s1, ch, k[i], words[i]]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b ^ c);
            let (temp2, _) = s0.overflowing_add(maj);

            h = g;
            g = f;
            f = e;
            e = d.overflowing_add(temp1).0;
            d = c;
            c = b;
            b = a;
            a = temp1.overflowing_add(temp2).0;
        }

        h0 = h0.overflowing_add(a).0;
        h1 = h1.overflowing_add(b).0;
        h2 = h2.overflowing_add(c).0;
        h3 = h3.overflowing_add(d).0;
        h4 = h4.overflowing_add(e).0;
        h5 = h5.overflowing_add(f).0;
        h6 = h6.overflowing_add(g).0;
        h7 = h7.overflowing_add(h).0;

    }

    let mut result: Vec<u8> = Vec::new();
    result.extend_from_slice(&h0.to_be_bytes());
    result.extend_from_slice(&h1.to_be_bytes());
    result.extend_from_slice(&h2.to_be_bytes());
    result.extend_from_slice(&h3.to_be_bytes());
    result.extend_from_slice(&h4.to_be_bytes());
    result.extend_from_slice(&h5.to_be_bytes());
    result.extend_from_slice(&h6.to_be_bytes());
    result.extend_from_slice(&h7.to_be_bytes());

    // TODO: try with [u8; 32] (via try_from)
    result
}

fn overflowing_sum(items: &[u32]) -> u32 {
    let mut sum = 0u32;
    for item in items {
        sum = sum.overflowing_add(*item).0;
    }
    sum
}

fn extend_words(words: &mut [u32; CHUNK_WORDS_COUNT]) -> Vec<u32> {
    let mut result: Vec<u32> = Vec::new();
    result.reserve(64);
    result.extend_from_slice(words);
    for i in CHUNK_WORDS_COUNT..64 {
        let w1 = result[i - 15];
        let s0 = w1.rotate_right(7) ^ w1.rotate_right(18) ^ (w1 >> 3);
        let w2 = result[i - 2];
        let s1 = w2.rotate_right(17) ^ w2.rotate_right(19) ^ (w2 >> 10);

        result[i] = overflowing_sum(&[result[i - 16], s0, result[i - 7], s1]);
    }
    result
}

fn chunk_to_be_words(chunk: &[u8]) -> [u32; CHUNK_WORDS_COUNT] {
    let item_size = std::mem::size_of::<u32>();
    let mut result = [0u32; CHUNK_WORDS_COUNT];

    for i in 0..CHUNK_WORDS_COUNT {
        let first_byte = i * item_size;
        let int_bytes = &chunk[first_byte..(first_byte + item_size)];
        result[i] = u32::from_be_bytes(int_bytes.try_into().unwrap());
    }
    result
}

fn preprocess(input: &[u8]) -> Vec<u8> {
    let message_bits_count = (input.len() * 8) as u64;
    let extra_zero_bits = get_extra_zero_bits_count(message_bits_count);

    let extra_bytes_count = (1 + extra_zero_bits) / 8;
    let mut tail = vec![0u8; extra_bytes_count as usize];
    tail[0] = 1u8 << 7;
    tail.extend_from_slice(&message_bits_count.to_be_bytes());
    let mut result = input.to_vec();
    result.append(&mut tail);
    result
}

fn get_extra_zero_bits_count(message_bits_count: u64) -> u64 {
    let reminder = (message_bits_count + 1) % 512;
    if reminder < 448 {
        448 - reminder
    } else if reminder > 448 {
        512 + 448 - reminder
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn calculate() {
        let expected: [u8; 32] = [
            0xD7, 0xA8, 0xFB, 0xB3, 0x07, 0xD7, 0x80, 0x94, 
            0x69, 0xCA, 0x9A, 0xBC, 0xB0, 0x08, 0x2E, 0x4F, 
            0x8D, 0x56, 0x51, 0xE4, 0x6D, 0x3C, 0xDB, 0x76, 
            0x2D, 0x02, 0xD0, 0xBF, 0x37, 0xC9, 0xE5, 0x92
        ];
        let input = b"The quick brown fox jumps over the lazy dog";

        let result = super::calculate(input);

        for i in 0..super::RESULT_LENGTH {
            assert_eq!(result[i], expected[i]);
        }
    }
}
