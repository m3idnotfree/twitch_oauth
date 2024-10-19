use crate::{traits::OauthRequest, types::HttpResponse, Error, Result};

use http::{
    header::{ACCEPT, CONTENT_TYPE},
    HeaderMap, HeaderValue,
};

const CONTENT_TYPE_JSON: &str = "application/json";
const CONTENT_TYPE_FORMENCODED: &str = "application/x-www-form-urlencoded";

pub async fn oauth_request<T>(request: T) -> Result<HttpResponse>
where
    T: OauthRequest,
{
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let request = match request.method() {
        http::Method::GET => {
            let mut headers = GET_headers();

            if let Some(header) = request.headers() {
                headers.extend(header);
            }

            Ok(client
                .request(request.method(), request.url())
                .headers(headers))
        }
        http::Method::POST => {
            let mut headers = POST_headers();

            if let Some(header) = request.headers() {
                headers.extend(header);
            }

            let request_builder = if let Some(body) = request.body() {
                client
                    .request(request.method(), request.url())
                    .headers(headers)
                    .body(body)
            } else {
                client
                    .request(request.method(), request.url())
                    .headers(headers)
            };

            Ok(request_builder)
        }
        _ => Err(Error::MethodError("only GET, POST implement".to_string())),
    };

    let response = request?.send().await?;

    let status = response.status();
    let headers = response.headers().clone();
    let body = response.text().await.unwrap();

    Ok(HttpResponse {
        status,
        headers,
        body,
    })
}

#[allow(non_snake_case)]
pub fn POST_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(ACCEPT, HeaderValue::from_static(CONTENT_TYPE_JSON));
    headers.append(
        CONTENT_TYPE,
        HeaderValue::from_static(CONTENT_TYPE_FORMENCODED),
    );

    headers
}

#[allow(non_snake_case)]
pub fn GET_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.append(ACCEPT, HeaderValue::from_static(CONTENT_TYPE_JSON));

    headers
}
