// for day 04 solution
// built based on the pseudocode in the wikipedia article

const SHIFTS: [u32; 64] = [
    7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
    5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
    4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
    6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
    0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
    0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
    0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
    0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
    0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
    0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
    0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
    0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const A0: u32 = 0x67452301;   // A
const B0: u32 = 0xefcdab89;   // B
const C0: u32 = 0x98badcfe;   // C
const D0: u32 = 0x10325476;   // D

fn leftrotate(x: u32, c: u32) -> u32 {
    (x << c) | (x >> (32 - c))
}

pub fn md5(message: &str) -> Vec<u8> {
    let mut bytes = message.as_bytes().to_vec();
    let bit_count = bytes.len().saturating_mul(8);
    let zero_padding = 512 - (bit_count % 512) - 65;

    let mut padding = vec![0; ((zero_padding - 7) / 8) + 1];
    padding[0] = 0b10000000;

    let msg_len_mod_2_64 = (0..64).step_by(8).map(|i| {
        ((bit_count >> i) & 0xff) as u8
    }).collect::<Vec<_>>();

    padding.extend_from_slice(&msg_len_mod_2_64);
    bytes.extend(padding.into_iter());

    let (words, _): (Vec<u32>, _) = bytes.iter().enumerate()
        .fold((vec![], 0), |(mut r, v), (i, &b)| {
            let v = (v << 8) + b as u32;
            if i % 4 == 3 {
                r.push(v.swap_bytes());
                (r, 0)
            } else {
                (r, v)
            }
        });

    let mut a0 = A0;
    let mut b0 = B0;
    let mut c0 = C0;
    let mut d0 = D0;

    for chunk_offset in (0..words.len() / 16).step_by(16) {
        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        for i in 0..64 {
            let (f, g) = if i < 16 {
                (
                    (b & c) | ((!b) & d),
                    i
                )
            } else if i < 32 {
                (
                    (d & b) | ((!d) & c),
                    (5 * i + 1) % 16
                )
            } else if i < 48 {
                (
                    b ^ c ^ d,
                    (3 * i + 5) % 16
                )
            } else if i < 64 {
                (
                    c ^ (b | (!d)),
                    (7 * i) % 16
                )
            } else {
                unreachable!()
            };
            let f = f.wrapping_add(a)
                .wrapping_add(K[i])
                .wrapping_add(words[g + chunk_offset]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(leftrotate(f, SHIFTS[i]));
        }
        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    let mut result8: Vec<u8> = Vec::with_capacity(16);
    result8.extend_from_slice(&a0.swap_bytes().to_be_bytes());
    result8.extend_from_slice(&b0.swap_bytes().to_be_bytes());
    result8.extend_from_slice(&c0.swap_bytes().to_be_bytes());
    result8.extend_from_slice(&d0.swap_bytes().to_be_bytes());

    result8
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_md5() {
        assert_eq!(
            0xd41d8cd98f00b204e9800998ecf8427eu128.to_be_bytes().to_vec(),
            md5("")
        );
        assert_eq!(
            0x9e107d9d372bb6826bd81d3542a419d6u128.to_be_bytes().to_vec(),
            md5("The quick brown fox jumps over the lazy dog")
        );
        let r = md5("pqrstuv1048970");
        assert_eq!(&r[..3], vec![0, 0, 6]);
    }
}
