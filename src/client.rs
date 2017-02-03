use hyper::header::Headers;
use hyper::Result;
use hyper::client::{Client, Response};
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
header!{ (Authorization, "Authorization") => [String] }
header!{ (UserAgent, "User-Agent") => [String] }

#[derive(Debug)]
pub struct Github {
    client: Client,
    headers: Headers
}

fn get_start_headers() -> Headers {
    let user_agent = UserAgent("GitHub-rs".to_owned());
    let mut headers = Headers::new();
    headers.set(user_agent);
    headers
}

impl Github {
     pub fn new() -> Github {
        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = Client::with_connector(connector);
        Github{client: client, headers: get_start_headers()}
     }
     pub fn add_auth(mut self, token: String) -> Github {
        let token_string = format!("token {}", token);
        let auth = Authorization(token_string);
        self.headers.set(auth);
        self
     }
     pub fn make_request(&self, url: &str) -> Result<Response> {
         self.client.get(url).headers(self.headers.clone()).send()
     }
}