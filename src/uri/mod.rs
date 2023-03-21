mod authority_parser;
mod schema_parser;

// http://user:pw@127.0.0.1:8080
// scheme: HTTP || HTTPS
// authority: username password
// host: url or ipaddress
// path: paths
// query: query params
// fragment: #root

#[derive(PartialEq, Eq, Debug)]
pub struct URI<'a> {
    scheme: Scheme,
    authority: Option<Authority<'a>>,
    host: Host,
    port: Option<u16>,
    path: Option<Vec<&'a str>>,
    query: Option<QueryParams<'a>>,
    fragment: Option<&'a str>,
}

#[derive(PartialEq, Eq, Debug)]
enum Scheme {
    HTTP,
    HTTPS,
}

impl From<&str> for Scheme {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "http://" => Scheme::HTTP,
            "https://" => Scheme::HTTPS,
            _ => unimplemented!("No Other Schemes are currently supported"),
        }
    }
}

type Authority<'a> = (&'a str, Option<&'a str>);

#[derive(PartialEq, Eq, Debug)]
enum Host {
    Host(String),
    IP([u8; 4]),
}

type QueryParam<'a> = (&'a str, &'a str);
type QueryParams<'a> = Vec<QueryParam<'a>>;
