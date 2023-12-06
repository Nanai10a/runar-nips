use crate::base::{astr, u256, u512};

pub trait ToAstr {
    const LEN: usize;

    fn to_astr(&self) -> astr<{ Self::LEN }>;
}

impl ToAstr for crate::base::Uri<'_> {
    // FIXME: temporary decided
    const LEN: usize = 256;

    fn to_astr(&self) -> astr<{ Self::LEN }> {
        use core::fmt::Write;

        let mut s = astr::new();
        write!(&mut s, "{self}").unwrap();

        s
    }
}

uint_impl! {   u8, as   2 }
uint_impl! {  u16, as   4 }
uint_impl! {  u32, as   8 }
uint_impl! {  u64, as  16 }
uint_impl! { u128, as  32 }
uint_impl! { u256, as  64 }
uint_impl! { u512, as 128 }

macro uint_impl($name:ident,as $len:expr) {
    impl ToAstr for $name {
        const LEN: usize = $len;

        fn to_astr(&self) -> astr<{ Self::LEN }> {
            use core::fmt::Write;

            let mut s = astr::new();
            write!(&mut s, "{self:0w$x}", w = $len).unwrap();

            s
        }
    }
}
