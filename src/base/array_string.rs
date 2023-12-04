#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct astr<const LEN: usize>([u8; LEN]);

impl<const LEN: usize> astr<LEN> {
    pub fn new() -> Self { Self([0; _]) }

    pub fn as_str(&self) -> &str { unsafe { core::str::from_utf8_unchecked(&self.0) } }

    pub fn cap(&self) -> usize { LEN - 1 }

    pub fn len(&self) -> usize { self.as_str().len() }
}

impl<const LEN: usize> core::fmt::Write for astr<LEN> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        if !s.is_ascii() {
            return Err(core::fmt::Error);
        }

        let len = self.as_str().len();
        let adv = s.as_bytes().len();

        if len + adv > self.cap() {
            return Err(core::fmt::Error);
        }

        self.0[len..(len + adv)].copy_from_slice(s.as_bytes());

        Ok(())
    }
}
