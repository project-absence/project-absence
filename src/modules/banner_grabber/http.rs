use std::collections::HashMap;

use crate::helpers;

use super::{Banner, BannerGrabber};
use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{SERVER, USER_AGENT},
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

    fn grab_server_header(&self, http_client: &Client) -> Option<String> {
        let uri = format!("{}://{}:{}", self.protocol, self.hostname, self.port);
        if let Ok(response) = http_client
            .get(uri)
            .header(USER_AGENT, helpers::ua::get_random())
            .send()
        {
            if let Some(server) = response.headers().get(SERVER) {
                return Some(server.to_str().unwrap().to_string());
            } else {
                return None;
            }
        }
        None
    }

    fn grab_title_tag(&self, http_client: &Client) -> Option<String> {
        let uri = format!("{}://{}:{}", self.protocol, self.hostname, self.port);
        if let Ok(response) = http_client
            .get(uri)
            .header(USER_AGENT, helpers::ua::get_random())
            .send()
        {
            if let Ok(text) = response.text() {
                let re = Regex::new(r#"<title>(?P<title>.*?)</title>"#).unwrap();
                if let Some(captures) = re.captures(&text) {
                    if let Some(title) = captures.name("title") {
                        return Some(title.as_str().to_string());
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
        None
    }
}

impl BannerGrabber for HttpBannerGrabber {
    fn grab_banner(&self, http_client: &Client) -> Banner {
        let mut http_banner = HashMap::new();
        if let Some(server_header) = self.grab_server_header(http_client) {
            http_banner.insert(String::from("server"), server_header);
        }
        if let Some(title_tag) = self.grab_title_tag(http_client) {
            http_banner.insert(String::from("title"), title_tag);
        }
        Banner(http_banner)
    }
}
