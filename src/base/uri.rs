use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ref: https://developer.mozilla.org/en-US/docs/Learn/Common_questions/Web_mechanics/What_is_a_URL
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Uri<'a> {
    scheme: &'a str,
    authority: Option<&'a str>,
    path: &'a str,
    query: Option<&'a str>,
    fragment: Option<&'a str>,
}

impl<'a> Uri<'a> {
    pub fn new(s: &'a str) -> Result<Self, !> {
        use core::cell::LazyCell;

        use regex::Regex;

        // ref: https://datatracker.ietf.org/doc/html/rfc3986#appendix-B
        const PARSER_REGEXP: &str = r"^(([^:/?#]+):)?(//([^/?#]*))?([^?#]*)(\?([^#]*))?(#(.*))?$";
        const PARSER: LazyCell<Regex> = LazyCell::new(|| Regex::new(PARSER_REGEXP).unwrap());

        let Some(caps) = PARSER.captures(s) else {
            panic!();
        };

        #[cfg(test)]
        dbg!(&caps);

        #[rustfmt::skip]
        let uri = {
            let scheme    = caps.get(2).map(|m| m.as_str()).unwrap();
            let authority = caps.get(4).map(|m| m.as_str());
            let path      = caps.get(5).map(|m| m.as_str()).unwrap();
            let query     = caps.get(7).map(|m| m.as_str());
            let fragment  = caps.get(9).map(|m| m.as_str());

            Self {
                scheme,
                authority,
                path,
                query,
                fragment,
            }
        };

        Ok(uri)
    }
}

impl<'a> Serialize for Uri<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        todo!()
    }
}

impl<'de> Deserialize<'de> for Uri<'de> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <&str>::deserialize(deserializer)
            .and_then(|s| Self::new(s).map_err(serde::de::Error::custom))
    }
}

#[test]
fn test() {
    dbg!(Uri::new(
        "file:/home/nanai/Workspace/runar-nips/target/doc/nom/bytes/complete/fn.take_while.html?\
         search=tag"
    ));
}
