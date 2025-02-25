use std::collections::HashMap;

use crate::helpers;

use super::{Banner, BannerGrabber};
use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, ALLOW, CONTENT_TYPE, SERVER, USER_AGENT},
};

pub struct HttpBannerGrabber {
    hostname: String,
    port: u16,
    protocol: String,
}

impl HttpBannerGrabber {
    pub fn new(hostname: String, port: u16) -> Self {
        let protocol = match port {
            80 => String::from("http"),
            443 => String::from("https"),
            _ => todo!(),
        };
        HttpBannerGrabber {
            hostname,
            port,
            protocol,
        }
    }

    fn grab_supported_methods_header(&self, headers: &HeaderMap) -> Option<String> {
        if let Some(allow) = headers.get(ALLOW) {
            return Some(allow.to_str().unwrap().to_string());
        }
        None
    }

    fn grab_content_type_header(&self, headers: &HeaderMap) -> Option<String> {
        if let Some(content_type) = headers.get(CONTENT_TYPE) {
            return Some(content_type.to_str().unwrap().to_string());
        }
        None
    }

    fn grab_server_header(&self, headers: &HeaderMap) -> Option<String> {
        if let Some(server) = headers.get(SERVER) {
            return Some(server.to_str().unwrap().to_string());
        }
        None
    }

    fn grab_title_tag(&self, response: String) -> Option<String> {
        let re = Regex::new(r#"<title>(?P<title>.*?)</title>"#).unwrap();
        if let Some(captures) = re.captures(&response) {
            return captures
                .name("title")
                .map(|title| title.as_str().to_string());
        }
        None
    }
}

impl BannerGrabber for HttpBannerGrabber {
    fn grab_banner(&self, http_client: &Client) -> Banner {
        let mut http_banner = HashMap::new();
        let uri = format!("{}://{}:{}", self.protocol, self.hostname, self.port);
        if let Ok(response) = http_client
            .get(uri)
            .header(USER_AGENT, helpers::ua::get_random())
            .send()
        {
            let headers = response.headers().clone();
            let text = response.text();
            if let Some(content_type_header) = self.grab_content_type_header(&headers) {
                http_banner.insert(String::from("content_type"), content_type_header);
            }
            if let Some(allow_header) = self.grab_supported_methods_header(&headers) {
                http_banner.insert(String::from("allow"), allow_header);
            }
            if let Some(server_header) = self.grab_server_header(&headers) {
                http_banner.insert(String::from("server"), server_header);
            }
            if let Some(title_tag) = self.grab_title_tag(text.unwrap_or_default()) {
                http_banner.insert(String::from("title"), title_tag);
            }
        }
        Banner(http_banner)
    }
}
