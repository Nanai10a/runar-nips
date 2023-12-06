#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct astr<const LEN: usize> {
    raw: [u8; LEN],
    len: usize,
}

impl<const LEN: usize> astr<LEN> {
    pub fn new() -> Self {
        Self {
            raw: [b'0'; _],
            len: 0,
        }
    }

    pub fn as_str(&self) -> &str { core::str::from_utf8(&self.raw[..self.len]).unwrap() }

    pub fn cap(&self) -> usize { LEN }

    pub fn len(&self) -> usize { self.len }
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

        self.raw[len..(len + adv)].copy_from_slice(s.as_bytes());
        self.len += adv;

        Ok(())
    }
}
