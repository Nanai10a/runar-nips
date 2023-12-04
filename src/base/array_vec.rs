#[derive(Debug, Clone)]
pub struct FixedVec<T, const LEN: usize>([Option<T>; LEN]);

impl<const LEN: usize, T> FixedVec<T, LEN> {
    pub fn new() -> Self { Self([(); _].map(|()| None)) }

    pub fn len(&self) -> usize {
        self.0
            .iter()
            .position(Option::is_none)
            .unwrap_or(self.cap())
    }

    pub fn cap(&self) -> usize { LEN }

    pub fn is_empty(&self) -> bool { self.0[0].is_none() }

    pub fn iter(&self) -> impl Iterator<Item = &T> { self.0.iter().filter_map(Option::as_ref) }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.0.iter_mut().filter_map(Option::as_mut)
    }

    pub fn try_push(&mut self, value: T) -> Option<T> {
        match self.0.iter_mut().find(|o| o.is_none()) {
            None => Some(value),
            Some(tail) => tail.replace(value),
        }
    }
}

impl<const LEN: usize, T: serde::Serialize> serde::Serialize for FixedVec<T, LEN> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0[..self.len()].serialize(serializer)
    }
}

impl<'de, const LEN: usize, T: serde::Deserialize<'de>> serde::Deserialize<'de>
    for FixedVec<T, LEN>
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Visitor<T>(core::marker::PhantomData<T>);

        impl<'de, const LEN: usize, T: serde::Deserialize<'de>> serde::de::Visitor<'de>
            for Visitor<FixedVec<T, LEN>>
        {
            type Value = FixedVec<T, LEN>;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "an array of length {} or less", LEN)
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut seq: A,
            ) -> Result<Self::Value, A::Error> {
                let mut vec = Self::Value::new();

                while let Some(item) = seq.next_element()? {
                    if vec.try_push(item).is_some() {
                        return Err(serde::de::Error::invalid_length(vec.len() + 1, &self));
                    }
                }

                Ok(vec)
            }
        }

        deserializer.deserialize_seq(Visitor(core::marker::PhantomData))
    }
}
