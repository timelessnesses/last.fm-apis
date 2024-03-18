use std::borrow::Cow;
use dxr;
use reqwest;
mod errors;
mod auth;
mod utils;
pub struct Authentication {
    pub login_url: String,
}

pub struct WebApplicationAuthenticationBuilder {
    pub login_url: String,
    pub auth_token: Option<String>
}

impl WebApplicationAuthenticationBuilder {
    pub fn new() -> Self {
        return WebApplicationAuthenticationBuilder {
            login_url: "http://www.last.fm/api/auth/".to_owned(),
            auth_token: None
        }
    }

    pub fn api_key(&self, api_key: impl AsRef<str>) -> Self {
        let mut url = reqwest::Url::from_file_path(self.login_url.to_owned()).unwrap();
        url.set_query(Some(format!("apikey={}", api_key.as_ref()).as_str()));
        return WebApplicationAuthenticationBuilder { login_url: url.to_string(), auth_token: self.auth_token.to_owned() }
    }

    pub fn callback(&self, u: impl reqwest::IntoUrl) -> Self {
        let mut url = reqwest::Url::from_file_path(self.login_url.to_owned()).unwrap();
        url.set_query(Some(format!("cb={}", u.as_str()).as_str()));
        return WebApplicationAuthenticationBuilder {
            login_url: url.to_string(),
            auth_token: self.auth_token.to_owned()
        }
    }

    pub fn parse_authentication_handler(&self, u: impl reqwest::IntoUrl) -> Result<Self, ()> {
        let url = u.into_url().unwrap();
        let pairs = url.query_pairs().filter(|q| {
            q.0 == "token"
        }).collect::<Vec<(Cow<str>, Cow<str>)>>();
        if pairs.len() == 0 {
            return Err(())
        }
        Ok(
            WebApplicationAuthenticationBuilder {
                login_url: self.login_url.to_owned(),
                auth_token: Some(pairs[0].clone().1.into_owned())
            }
        )
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn builder_thing() {
        WebApplicationAuthenticationBuilder::new().api_key("gay").callback("yeah");
    }
}