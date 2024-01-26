//! Implements query-string decoding

use ehttpd::{bytes::Data, error, error::Error};
use std::{borrow::Cow, collections::BTreeMap, ops::Deref};

/// A query string
///
/// ## Important
/// The query parser is pretty simple and basically parses any `key` or `key=` or `key=value` component without further
/// validation.
///
/// The following rules apply:
///  - the query string _MUST NOT_ begin with a `?` – it's not a bug, it's a feature: this allows the parser to parse raw
///    query strings in the body (e.g. from HTML forms)
///  - keys don't need a value (i.e. `key0&key1` is valid)
///  - keys can have an empty value (i.e. `key0=&key1=` is valid)
///  - keys can have a non-empty value (i.e. `key0=value0&key1=value1` is valid)
///  - empty keys/key-value pairs are ignored (i.e. `&` evaluates to `[]`, `key0&&key1` evaluates to
///    `["key0": "", "key1": ""]` and `=value0&key1=value1&` evaluates to `["key1": "value1"]`)
#[derive(Debug, Clone)]
pub struct QueryString<'a> {
    /// The request base URL
    url: &'a [u8],
    /// The querystring key-value pairs
    fields: BTreeMap<Cow<'a, [u8]>, Cow<'a, [u8]>>,
}
impl<'a> QueryString<'a> {
    /// Splits a request target into its base URL and the query string
    #[allow(clippy::missing_panics_doc)]
    pub fn decode(target: &'a Data) -> Result<Self, Error> {
        // Split the URL
        let mut target = target.splitn(2, |b| *b == b'?');
        let url = target.next().expect("first element of split iterator is empty?!");
        let querystring = target.next().unwrap_or_default();

        // Parse the query components
        let mut fields = BTreeMap::new();
        for pair in querystring.split(|b| *b == b'&') {
            // Read the next pair
            let mut pair = pair.splitn(2, |b| *b == b'=');
            let key = pair.next().map(Cow::Borrowed).expect("first element of split iterator is empty?!");
            let value = pair.next().map(Cow::Borrowed).unwrap_or_default();

            // Insert the key if it is not empty
            if !key.is_empty() {
                // Decode key and value and insert it
                let key = Self::percent_decode(key)?;
                let value = Self::percent_decode(value)?;
                fields.insert(key, value);
            }
        }
        Ok(Self { url, fields })
    }

    /// The request base URL
    pub fn url(&self) -> &[u8] {
        self.url
    }

    /// Percent-decodes the encoded data
    pub fn percent_decode(encoded: Cow<[u8]>) -> Result<Cow<[u8]>, Error> {
        // Check if we need some decoding
        let needs_decode = encoded.iter().any(|b| *b == b'%');
        if !needs_decode {
            return Ok(encoded);
        }

        // Perform decoding
        let mut source = encoded.iter().copied();
        let mut decoded = Vec::new();
        while let Some(mut byte) = source.next() {
            // Decode percent literal if necessary
            if byte == b'%' {
                // Get the encoded bytes
                let high = source.next().ok_or(error!("Truncated hex literal"))?;
                let low = source.next().ok_or(error!("Truncated hex literal"))?;
                byte = Self::percent_decode_byte(high, low)?;
            }

            // Write byte
            decoded.push(byte);
        }
        Ok(Cow::Owned(decoded))
    }

    /// Encodes a nibble into a hex char
    fn percent_decode_nibble(nibble: u8) -> Result<u8, Error> {
        // Note: All operations are safe since they are implicitly validated by the range comparisons
        #[allow(clippy::arithmetic_side_effects)]
        match nibble {
            b'0'..=b'9' => Ok(nibble - b'0'),
            b'a'..=b'f' => Ok((nibble - b'a') + 0xA),
            b'A'..=b'F' => Ok((nibble - b'A') + 0xA),
            nibble => Err(error!("Invalid nibble 0x{nibble:01x}")),
        }
    }

    /// Encodes a byte
    fn percent_decode_byte(high: u8, low: u8) -> Result<u8, Error> {
        Ok(Self::percent_decode_nibble(high)? << 4 | Self::percent_decode_nibble(low)?)
    }
}
impl<'a> Deref for QueryString<'a> {
    type Target = BTreeMap<Cow<'a, [u8]>, Cow<'a, [u8]>>;

    fn deref(&self) -> &Self::Target {
        &self.fields
    }
}
