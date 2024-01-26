//! An extension trait for HTTP requests to work with query strings

use crate::querystring::QueryString;
use ehttpd::{error::Error, http::Request};

/// An extension trait for HTTP requests to work with query strings
pub trait RequestQuerystringExt {
    /// Gets the request query string
    fn querystring(&self) -> Result<QueryString, Error>;
}
impl<'a, const HEADER_SIZE_MAX: usize> RequestQuerystringExt for Request<'a, HEADER_SIZE_MAX> {
    fn querystring(&self) -> Result<QueryString, Error> {
        QueryString::decode(&self.target)
    }
}
