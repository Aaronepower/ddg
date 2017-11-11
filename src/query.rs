use std::borrow::Cow;
use std::{fmt, error};

use reqwest::{self, Url, UrlError, Error as HttpError};

use serde_json;

use super::*;

/// A Builder struct for making the query.
#[derive(Clone, Debug, Default)]
pub struct Query<'a> {
    name: Cow<'a, str>,
    no_html: bool,
    query: Cow<'a, str>,
    skip_disambig: bool,
}

impl<'a> Query<'a> {
    /// Constructs a new query object, requiring the **query**, and the **name**
    /// of your app. It is recommended to use a constant for the name of your
    /// app.
    ///
    /// ```
    /// use ddg::Query;
    /// const APP_NAME: &'static str = "ddg_example_app";
    /// let query = Query::new("Rust", APP_NAME);
    ///
    /// let response = query.execute().unwrap();
    /// ```
    pub fn new<I: Into<Cow<'a, str>>>(query: I, name: I) -> Self {
        Query { query: query.into(), name: name.into(), ..Self::default() }
    }

    /// Will strip out any HTML content from the text in the Response
    /// eg.(_italics_, **bolds**, etc)
    ///
    /// ```
    /// use ddg::Query;
    /// const APP_NAME: &'static str = "ddg_example_app";
    ///
    /// let query = Query::new("Rust", APP_NAME).no_html();
    ///
    /// let response = query.execute().unwrap();
    /// ```
    pub fn no_html(mut self) -> Self {
        self.no_html = true;
        self
    }

    /// Skip the D(Disambiguation) type of Instant Answer.
    pub fn skip_disambig(mut self) -> Self {
        self.skip_disambig = true;
        self
    }

    /// Execute the request and parses it into a `DdgResponse` struct.
    pub fn execute(self) -> Result<Response, Error> {
        Ok(reqwest::get(self.into_url()?)?.error_for_status()?.json()?)
    }

    fn into_url(self) -> Result<Url, UrlError> {
        const URL: &'static str = "https://api.duckduckgo.com?format=json&no_redirect=1";
        let mut query = String::from(URL);

        if self.no_html {
            query.push_str("&no_html=1");
        }

        if self.skip_disambig {
            query.push_str("&skip_disambig=1");
        }

        Url::parse_with_params(&query, &[
            ("q", &*self.query),
            ("t", &*self.name)
        ])
    }
}

/// Error from parsing or converting into a URL.
#[derive(Debug)]
pub enum Error {
    /// An error in making the HTTP request, or parsing the query string into a
    /// url.
    Http(HttpError),
    /// An error in parsing the JSON.
    Serde(serde_json::Error),
    /// An error in parsing the URL.
    Url(UrlError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        use self::Error::*;

        match *self {
            Http(ref err) => write!(f, "Http: {}", err.description()),
            Serde(ref err) => write!(f, "Serde: {}", err.description()),
            Url(ref err) => write!(f, "Url: {}", err.description()),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::Error::*;

        match *self {
            Http(ref err) => err.description(),
            Serde(ref err) => err.description(),
            Url(ref err) => err.description(),
        }
    }
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Self {
        Error::Http(error)
    }
}

impl From<UrlError> for Error {
    fn from(error: UrlError) -> Self {
        Error::Url(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::Serde(error)
    }
}

#[cfg(test)]
mod tests {
    use super::Query;

    const APP_NAME: &'static str = "ddg_rs_tests";

    #[test]
    fn it_works() {
        let rs = Query::new("Rust", APP_NAME).execute();

        println!("{:?}", rs);
        assert!(rs.is_ok());
    }

    #[test]
    fn never_directly_redirect() {
        let query = Query::new("!crates tokei", APP_NAME);

        let rs = query.execute();

        assert!(rs.is_ok());
    }
}
