// FIXME: why cannot available `::from_be_bytes` on using `decl_macro`?
macro_rules! uint {
    ($name:ident,as $len:expr) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name([u8; $len]);

        impl $name {
            pub fn from_be_bytes(bytes: [u8; $len]) -> Self { Self(bytes) }

            pub fn to_be_bytes(&self) -> [u8; $len] { self.0 }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                <Self as core::fmt::Display>::fmt(self, f)
            }
        }

        impl core::fmt::Display for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                todo!("it's so difficult")
            }
        }

        impl core::fmt::LowerHex for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                self.0
                    .iter()
                    .map(|byte: &u8| [(byte & 0b1111_0000) >> 4, (byte & 0b0000_1111) >> 0])
                    .flatten()
                    .skip_while(|u4| *u4 == 0)
                    .map(|u4: u8| (u4 + u4 / 10 * (b'a' - (b'9' + 1)) + b'0') as char)
                    .try_for_each(|c| write!(f, "{c}"))
            }
        }

        impl serde::Serialize for $name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                use core::fmt::Write;

                let mut s = crate::base::astr::<{ $len * 2 }>::new();
                write!(&mut s, "{self:0w$x}", w = $len * 2).unwrap();

                serializer.serialize_str(s.as_str())
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let bytes = <&str>::deserialize(deserializer)?.as_bytes();

                Ok(todo!())
            }
        }
    };
}

uint! { u256, as 32 }
uint! { u512, as 64 }
