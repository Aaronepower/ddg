use std::borrow::Cow;

#[cfg(feature = "reqwest")]
use reqwest::{self, IntoUrl, Url, UrlError, Error as HttpError};

#[cfg(feature = "hyper")] use hyper::Url;
#[cfg(feature = "hyper")] use hyper::client::IntoUrl;
#[cfg(feature = "hyper")]
use hyper::error::{ParseError as UrlError, Error as HttpError};

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
    /// ```no_run
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
    /// ```no_run
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
    #[cfg(feature = "reqwest")]
    pub fn execute(self) -> Result<DdgResponse, Error> {
        Ok(serde_json::from_reader(reqwest::get(self)?)?)
    }
}

/// Error from parsing or convertingi into a URL.
#[derive(Debug)]
pub enum Error {
    /// An error in making the HTTP request, or parsing the query string into a
    /// url.
    Http(HttpError),
    /// An error in parsing the JSON.
    SerdeJson(serde_json::Error),
}

impl From<HttpError> for Error {
    fn from(error: HttpError) -> Self {
        Error::Http(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::SerdeJson(error)
    }
}

impl<'a> IntoUrl for Query<'a> {
    fn into_url(self) -> Result<Url, UrlError> {
        let mut query = format!("https://api.duckduckgo.com/?q={}&t={}&format=json&no_redirect=1",
                                self.query, self.name);

        if self.no_html {
            query.push_str("&no_html=1");
        }

        if self.skip_disambig {
            query.push_str("&skip_disambig=1");
        }

        Url::parse(&query)
    }
}

#[cfg(all(test, feature = "reqwest"))]
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
