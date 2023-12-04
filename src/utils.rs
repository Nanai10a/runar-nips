mod sha2;
pub use sha2::sha2_256;

pub struct Exact<T>(T);

impl<T> Exact<T> {
    pub fn into_inner(self) -> T { self.0 }
}

impl<T> FromIterator<T> for Exact<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        let Some(item) = iter.next() else {
            panic!("unexpected end of iterator")
        };

        let None = iter.next() else {
            panic!("unexpected one more item of iterator")
        };

        Exact(item)
    }
}
