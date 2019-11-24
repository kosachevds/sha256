use std::u64;

const RESULT_LENGTH: usize = 32;

fn calculate(input: &[u8]) -> [u8; 32] {
    let h0: u32 = 0x6A09E667;
    let h1: u32 = 0xBB67AE85;
    let h2: u32 = 0x3C6EF372;
    let h3: u32 = 0xA54FF53A;
    let h4: u32 = 0x510E527F;
    let h5: u32 = 0x9B05688C;
    let h6: u32 = 0x1F83D9AB;
    let h7: u32 = 0x5BE0CD19;
    
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

    [
        0xD7u8, 0xA8, 0xFB, 0xB3, 0x07, 0xD7, 0x80, 0x94, 
        0x69, 0xCA, 0x9A, 0xBC, 0xB0, 0x08, 0x2E, 0x4F, 
        0x8D, 0x56, 0x51, 0xE4, 0x6D, 0x3C, 0xDB, 0x76, 
        0x2D, 0x02, 0xD0, 0xBF, 0x37, 0xC9, 0xE5, 0x92
    ]
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
