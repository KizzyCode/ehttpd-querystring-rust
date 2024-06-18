//! Extension traits for `querystring::QueryString`

use crate::querystring::QueryString;
use ehttpd::error::Error;
use std::{
    borrow::{Borrow, Cow},
    str::{self, FromStr},
};

/// Some query string extensions
pub trait QueryStringExt<'a> {
    /// Gets a value and converts it to the requested type
    fn get_as<T, K>(&self, key: &K) -> Result<Option<T>, Error>
    where
        T: FromStr,
        K: Ord + ?Sized,
        Error: From<T::Err>,
        Cow<'a, [u8]>: Borrow<K> + Ord;
}
impl<'a> QueryStringExt<'a> for QueryString<'a> {
    fn get_as<T, K>(&self, key: &K) -> Result<Option<T>, Error>
    where
        T: FromStr,
        K: Ord + ?Sized,
        Error: From<T::Err>,
        Cow<'a, [u8]>: Borrow<K> + Ord,
    {
        // Get the value
        let Some(value) = self.get(key) else {
            // No value for this key
            return Ok(None);
        };

        // Parse the value to a string
        let value = str::from_utf8(value)?;
        let parsed = value.parse()?;
        Ok(Some(parsed))
    }
}
