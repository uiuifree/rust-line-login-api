mod error;
mod api;

pub use error::*;
use reqwest::{Body, Method, RequestBuilder, Url};
use serde::Serialize;
use serde_json::{json, Value};
pub use api::*;

pub struct LineLoginClient {
    pub(crate) context: LineContext,
}

impl LineLoginClient {
    pub fn new<T:ToString,S:ToString>(client_id: T, client_secret: S) -> Self {
        Self {
            context: LineContext {
                client_id: Some(client_id.to_string()),
                client_secret: Some(client_secret.to_string()),
            },
        }
    }
    pub(crate) fn client_id(&self) -> String {
        self.context.client_id.clone().unwrap_or_default()
    }
    pub(crate) fn client_secret(&self) -> String {
        self.context.client_secret.clone().unwrap_or_default()
    }
}

pub(crate) struct LineContext {
    pub(crate) client_id: Option<String>,
    pub(crate) client_secret: Option<String>,
}

pub type LineApiResponse<T> = Result<T, LineLoginError>;

impl LineLoginClient {
    async fn http_response_reqwest<R>(
        response: Result<reqwest::Response, reqwest::Error>,
    ) -> LineApiResponse<R>
        where
            R: for<'de> serde::Deserialize<'de>,
    {

        let response = match response {
            Ok(v) => v,
            Err(e) => return Err(LineSystemError::new(e.to_string()).into()),
        };


        let status = response.status();
        let body = match response.bytes().await {
            Ok(v) => v.to_vec(),
            Err(e) => {
                return Err(LineHttpError::new(status.as_u16(), e.to_string()).into());
            }
        };

        let http_response_body = match String::from_utf8(body) {
            Ok(v) => v,
            Err(e) => return Err(LineSystemError::new(e.to_string()).into()),
        };

        let value = match serde_json::from_str::<Value>(http_response_body.as_str()) {
            Ok(v) => v,
            Err(e) => return Err(LineSystemError::new(e.to_string()).into()),
        };

        Self::http_response_text(status.as_u16(), http_response_body, value).await
    }
    async fn http_response_text<R>(
        status: u16,
        http_response_body: String,
        value: Value,
    ) -> LineApiResponse<R>
        where
            R: for<'de> serde::Deserialize<'de>,
    {
        if 400 <= status {
            if let Ok(res) = serde_json::from_value(value.clone()) {
                return Err(LineApiError {
                    status: status,
                    error: res,
                    warnings: None,
                    http_response_body: Some(http_response_body),
                }
                    .into());
            }
        }

        match serde_json::from_value(value.clone()) {
            Ok(v) => Ok(v),
            Err(e) => {
                if let Ok(res) = serde_json::from_value(value.clone()) {
                    return Err(LineApiError {
                        status: status,
                        error: res,
                        warnings: None,
                        http_response_body: Some(http_response_body),
                    }
                        .into());
                }
                Err(LineSystemError::new(e.to_string()).into())
            }
        }
    }
    pub(crate) async fn http_get<P, R>(&self, url: &str, value: &P, access_token: &str) -> LineApiResponse<R>
        where
            P: serde::Serialize,
            R: for<'de> serde::Deserialize<'de>,
    {
        let build = builder2(
            Url::parse(url).unwrap(),
            Method::GET,
            access_token,
        );
        let request = build.body(Body::from(json!(value).to_string()));
        LineLoginClient::http_response_reqwest(request.send().await).await
    }

    pub(crate) async fn http_post< P,R>(&self, url: &str, value: P, access_token: &str) -> LineApiResponse<R>
        where
            P:Serialize + std::fmt::Debug,
            R: for<'de> serde::Deserialize<'de>,
    {
        let build = builder2(
            Url::parse(url).unwrap(),
            Method::POST,
            access_token,
        );

        let request = build.form(&value);
        let res = request.send().await;
        LineLoginClient::http_response_reqwest(res).await
    }

    // pub(crate) async fn http_delete<P, R>(&self, url: &str, value: &P, access_token: &str) -> LineApiResponse<R>
    //     where
    //         P: serde::Serialize,
    //         R: for<'de> serde::Deserialize<'de>,
    // {
    //     let build = builder2(
    //         Url::parse(url).unwrap(),
    //         Method::DELETE,
    //         access_token,
    //     );
    //
    //     let request = build.json(&json!(value));
    //     LineLoginClient::http_response_reqwest(request.send().await).await
    // }
    // pub(crate) async fn http_post_data<R>(&self, url: &str, content: Vec<u8>,access_token:&str) -> LineApiResponse<R>
    // where
    //     R: for<'de> serde::Deserialize<'de>,
    // {
    //     let req = reqwest::Client::new()
    //         .post(url)
    //         .header(
    //             "Authorization",
    //             format!("Bearer {}", access_token),
    //         )
    //         .header("Content-Type", "image/jpeg")
    //         .body(content)
    //         .send()
    //         .await;
    //
    //     Self::http_response_reqwest(req).await
    // }
}

fn builder2(url: Url, method: Method, token: &str) -> RequestBuilder {
    let mut res = reqwest::Client::new()
        .request(method, url);
    res = res.header("Content-Type", "application/x-www-form-urlencoded");
    if !token.is_empty() {
        res = res.header("Authorization", format!("Bearer {}", token));
    }
    res
}
