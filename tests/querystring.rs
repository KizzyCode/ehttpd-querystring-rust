use ehttpd::bytes::Data;
use ehttpd_querystring::querystring::QueryString;
use std::{borrow::Cow, collections::BTreeMap, ops::Deref};

/// Creates a new map
pub fn map<I>(pairs: I) -> BTreeMap<Cow<'static, [u8]>, Cow<'static, [u8]>>
where
    I: IntoIterator<Item = (&'static str, &'static str)>,
{
    (pairs.into_iter())
        .map(|(k, v)| (k.as_bytes(), v.as_bytes()))
        .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
        .collect()
}

#[derive(Debug)]
struct Test {
    raw: &'static [u8],
    expected: BTreeMap<Cow<'static, [u8]>, Cow<'static, [u8]>>,
}
impl Test {
    pub fn test(self) {
        // Parse the query string
        let data = Data::Static(self.raw);
        let query = QueryString::decode(&data).expect("failed to decode query string?!");
        assert_eq!(&self.expected, query.deref(), "invalid decoded query string?!");
    }
}
#[test]
fn test() {
    Test {
        raw: b"?code=M696be062-f150-bb19-9944-0c3a0ca60b48&state=99f4bd624dbe53d0ae330eabda904ac4",
        expected: map([
            ("code", "M696be062-f150-bb19-9944-0c3a0ca60b48"),
            ("state", "99f4bd624dbe53d0ae330eabda904ac4"),
        ]),
    }
    .test();
    Test {
        raw: b"?q=tree+-swing&l=commderiv&d=taken-20000101-20051231&ct=0&lol&mt=all&adv=1&&",
        expected: map([
            ("q", "tree+-swing"),
            ("l", "commderiv"),
            ("d", "taken-20000101-20051231"),
            ("ct", "0"),
            ("mt", "all"),
            ("adv", "1"),
            ("lol", ""),
        ]),
    }
    .test();
    Test { raw: b"?%2FVolume=%2FData%2F%0A", expected: map([("/Volume", "/Data/\x0A")]) }.test();
    Test { raw: b"?", expected: BTreeMap::new() }.test();
}
