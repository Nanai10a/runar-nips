use crate::base::astr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u256(u128, u128);

impl core::fmt::Debug for u256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!("it's so difficult")
    }
}

impl From<(u128, u128)> for u256 {
    fn from((lhs, rhs): (u128, u128)) -> Self { u256(lhs, rhs) }
}

impl From<u256> for (u128, u128) {
    fn from(u256(lhs, rhs): u256) -> Self { (lhs, rhs) }
}

impl serde::Serialize for u256 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use core::fmt::Write;

        let Self(lhs, rhs) = self;

        // TODO: replace fixed-length string on stack
        let mut s = astr::<64>::new();

        write!(&mut s, "{:032x}", lhs).unwrap();
        write!(&mut s, "{:032x}", rhs).unwrap();

        serializer.serialize_str(s.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for u256 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <&str>::deserialize(deserializer)?.as_bytes();

        if bytes.len() != 64 {
            return Err(serde::de::Error::invalid_length(bytes.len(), &"64"));
        }

        let (lhs, rhs) = bytes
            .array_chunks::<64>()
            .collect::<crate::utils::Exact<&[_; 64]>>()
            .into_inner()
            .split_at(32);

        let lhs = unsafe { core::str::from_utf8_unchecked(lhs) };
        let rhs = unsafe { core::str::from_utf8_unchecked(rhs) };

        let lhs = u128::from_str_radix(lhs, 16).map_err(serde::de::Error::custom)?;
        let rhs = u128::from_str_radix(rhs, 16).map_err(serde::de::Error::custom)?;

        Ok(Self(lhs, rhs))
    }
}

// FIXME: generalize integer implements
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct u512(u256, u256);

impl core::fmt::Debug for u512 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        todo!("it's so difficult")
    }
}

impl From<(u256, u256)> for u512 {
    fn from((lhs, rhs): (u256, u256)) -> Self { u512(lhs, rhs) }
}

impl From<u512> for (u256, u256) {
    fn from(u512(lhs, rhs): u512) -> Self { (lhs, rhs) }
}

impl serde::Serialize for u512 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use core::fmt::Write;

        let Self(u256(llhs, lrhs), u256(rlhs, rrhs)) = self;

        // TODO: replace fixed-length string on stack
        let mut s = astr::<128>::new();

        write!(&mut s, "{:032x}", llhs).unwrap();
        write!(&mut s, "{:032x}", lrhs).unwrap();
        write!(&mut s, "{:032x}", rlhs).unwrap();
        write!(&mut s, "{:032x}", rrhs).unwrap();

        serializer.serialize_str(s.as_str())
    }
}

impl<'de> serde::Deserialize<'de> for u512 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let bytes = <&str>::deserialize(deserializer)?.as_bytes();

        if bytes.len() != 128 {
            return Err(serde::de::Error::invalid_length(bytes.len(), &"64"));
        }

        let (lhs, rhs) = bytes
            .array_chunks::<128>()
            .collect::<crate::utils::Exact<&[_; 128]>>()
            .into_inner()
            .split_at(64);

        let (llhs, lrhs) = lhs.split_at(32);
        let (rlhs, rrhs) = rhs.split_at(32);

        let llhs = unsafe { core::str::from_utf8_unchecked(llhs) };
        let lrhs = unsafe { core::str::from_utf8_unchecked(lrhs) };
        let rlhs = unsafe { core::str::from_utf8_unchecked(rlhs) };
        let rrhs = unsafe { core::str::from_utf8_unchecked(rrhs) };

        let llhs = u128::from_str_radix(llhs, 16).map_err(serde::de::Error::custom)?;
        let lrhs = u128::from_str_radix(lrhs, 16).map_err(serde::de::Error::custom)?;
        let rlhs = u128::from_str_radix(rlhs, 16).map_err(serde::de::Error::custom)?;
        let rrhs = u128::from_str_radix(rrhs, 16).map_err(serde::de::Error::custom)?;

        Ok(Self(u256(llhs, lrhs), u256(rlhs, rrhs)))
    }
}
