#[allow(non_camel_case_types)]
type byte = u8;

#[allow(non_camel_case_types)]
type word = u32;

const WORD_SIZE: usize = core::mem::size_of::<word>();
const CHUNK_LEN: usize = 512 /* bit */ / 8;
const INIT_CHUNK_LEN: usize = 16;

const INIT: [word; 8] = [
    0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
];

const ROUND: [word; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
];

// ref: https://en.wikipedia.org/wiki/SHA-2#Pseudocode
pub fn sha2_256<T: IntoIterator<Item = byte>>(t: T) -> [byte; 32] {
    let data = t.into_iter();

    let len = core::cell::Cell::new(0);

    let pad_head = 0b10000000;

    let pad_zero = PadZero::uninit(&len, CHUNK_LEN);
    let pad_len = PadLen::uninit(&len);

    let padded = counting(&len, data)
        .chain([pad_head])
        .chain(pad_zero)
        .chain(pad_len);

    let chunks = padded
        .array_chunks::<WORD_SIZE>()
        .map(word::from_be_bytes)
        .array_chunks::<INIT_CHUNK_LEN>();

    let mut hash = INIT;

    chunks
        .map(|chunk| {
            let mut sch = [0; CHUNK_LEN];
            sch[..INIT_CHUNK_LEN].copy_from_slice(&chunk);

            for idx in INIT_CHUNK_LEN..CHUNK_LEN {
                #[rustfmt::skip]
                let t0 = (sch[idx - 15].rotate_right( 7))
                       ^ (sch[idx - 15].rotate_right(18))
                       ^ (sch[idx - 15] >>  3);

                let t1 = sch[idx - 7];

                #[rustfmt::skip]
                let t2 = (sch[idx -  2].rotate_right(17))
                       ^ (sch[idx -  2].rotate_right(19))
                       ^ (sch[idx -  2] >> 10);

                sch[idx] = sch[idx - 16]
                    .wrapping_add(t0)
                    .wrapping_add(t1)
                    .wrapping_add(t2);
            }

            sch
        })
        .for_each(|sch| {
            let mut work = hash;

            for (sch, rnd) in (0..CHUNK_LEN).map(|idx| (sch[idx], ROUND[idx])) {
                let [a, b, c, d, e, f, g, h] = work;

                let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
                let ch = (e & f) ^ (!e & g);
                let t0 = h
                    .wrapping_add(s1)
                    .wrapping_add(ch)
                    .wrapping_add(rnd)
                    .wrapping_add(sch);

                let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
                let mj = (a & b) ^ (a & c) ^ (b & c);
                let t1 = s0.wrapping_add(mj);

                work = [t0.wrapping_add(t1), a, b, c, d.wrapping_add(t0), e, f, g];
            }

            for (h, w) in hash.iter_mut().zip(work.iter()) {
                *h = h.wrapping_add(*w);
            }
        });

    let mut result = [0; _];

    #[rustfmt::skip]
    #[allow(clippy::identity_op)]
    for (r, h) in result
        .iter_mut()
        .array_chunks::<WORD_SIZE>()
        .zip(hash.iter())
    {
        *r[0] = (h >> 24) as byte;
        *r[1] = (h >> 16) as byte;
        *r[2] = (h >>  8) as byte;
        *r[3] = (h >>  0) as byte;
    }

    result
}

fn counting<'a, I: Iterator + 'a>(
    count: &'a core::cell::Cell<u64>,
    iter: I,
) -> impl Iterator<Item = I::Item> + 'a {
    iter.inspect(|_| {
        count.update(|c| c + 1);
    })
}

enum PadZero<'a> {
    Uninit {
        src_len: &'a core::cell::Cell<u64>,
        chk_len: usize,
    },
    Init {
        iter: core::iter::Take<core::iter::Repeat<byte>>,
    },
}

impl<'a> PadZero<'a> {
    fn uninit(src_len: &'a core::cell::Cell<u64>, chk_len: usize) -> Self {
        Self::Uninit { src_len, chk_len }
    }
}

impl<'a> Iterator for PadZero<'a> {
    type Item = byte;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Uninit { src_len, chk_len } => {
                let src_len = src_len.get() as isize;
                let chk_len = *chk_len as isize;

                let pad_zero_len = (-1 - src_len).rem_euclid(chk_len) - 8;
                let iter = core::iter::repeat(0b00000000).take(pad_zero_len as usize);

                *self = Self::Init { iter };
                self.next()
            },
            Self::Init { iter } => iter.next(),
        }
    }
}

enum PadLen<'a> {
    Uninit { src_len: &'a core::cell::Cell<u64> },
    Init { iter: core::array::IntoIter<u8, 8> },
}

impl<'a> PadLen<'a> {
    fn uninit(src_len: &'a core::cell::Cell<u64>) -> Self { Self::Uninit { src_len } }
}

impl<'a> Iterator for PadLen<'a> {
    type Item = byte;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Uninit { src_len } => {
                let src_len = src_len.get() * 8;
                let iter = src_len.to_be_bytes().into_iter();

                *self = Self::Init { iter };
                self.next()
            },
            Self::Init { iter } => iter.next(),
        }
    }
}

#[test]
fn test() {
    use crate::base::u256;
    use crate::utils::ToAstr;

    macro assert_eq_hash($bytes_ref:expr, $hash:expr,) {
        assert_eq! {
            u256::from_be_bytes(sha2_256($bytes_ref.iter().copied())).to_astr().as_str(),
            $hash
        };
    }

    assert_eq_hash! {
        b"",
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
    };

    assert_eq_hash! {
        b"abc",
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
    };

    assert_eq_hash! {
        b"k1ANJ/6TiGuRtWSqI1GwujUqwMajX4srw3HM1T3Th2WQ1d7Ad//RxqSSyhc6kQ6Aa8s23KGdZC46",
        "6ddc612f6179fd101e86ae815f58d95e95e49a4a05eec887a0a3b1ea83dccf99",
    };
}
