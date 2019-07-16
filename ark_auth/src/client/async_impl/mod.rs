mod auth;
mod key;
mod service;
mod user;

use crate::client::{Client, ClientOptions, Error, RequestError};
use crate::server::api::route;
use actix_web::client::ClientResponse;
use actix_web::http::{header, StatusCode};
use futures::stream::Stream;
use futures::{future, Future};
use serde::ser::Serialize;
use serde_json::Value;

impl From<actix_http::client::SendRequestError> for Error {
    fn from(err: actix_http::client::SendRequestError) -> Error {
        let description = format!("{}", err);
        Error::Client(description)
    }
}

impl From<awc::error::JsonPayloadError> for Error {
    fn from(err: awc::error::JsonPayloadError) -> Error {
        let description = format!("{}", err);
        Error::Client(description)
    }
}

/// Asynchronous client handle.
pub struct AsyncClient {
    pub options: ClientOptions,
    pub client: actix_web::client::Client,
}

impl AsyncClient {
    pub fn ping(&self) -> impl Future<Item = Value, Error = Error> {
        self.get(route::PING)
            .send()
            .map_err(Into::into)
            .and_then(AsyncClient::match_status_code)
            .and_then(|mut res| res.json::<Value>().map_err(Into::into))
    }

    fn build_client(options: &ClientOptions) -> actix_web::client::Client {
        actix_web::client::Client::build()
            .header(header::CONTENT_TYPE, header::ContentType::json())
            .header(header::USER_AGENT, options.user_agent.to_owned())
            .finish()
    }

    fn get(&self, path: &str) -> actix_web::client::ClientRequest {
        let url = self.options.url_path(path).unwrap();
        self.client
            .get(url.to_string())
            .header(header::AUTHORIZATION, self.options.authorisation.to_owned())
    }

    fn get_query<T: Serialize>(&self, path: &str, query: T) -> actix_web::client::ClientRequest {
        let url = self.options.url_path_query(path, query).unwrap();
        self.client
            .get(url.to_string())
            .header(header::AUTHORIZATION, self.options.authorisation.to_owned())
    }

    fn post(&self, path: &str) -> actix_web::client::ClientRequest {
        let url = self.options.url_path(path).unwrap();
        self.client
            .post(url.to_string())
            .header(header::AUTHORIZATION, self.options.authorisation.to_owned())
    }

    fn match_status_code<T: Stream>(
        response: ClientResponse<T>,
    ) -> impl Future<Item = ClientResponse<T>, Error = Error> {
        match response.status() {
            StatusCode::OK => future::ok(response),
            StatusCode::BAD_REQUEST => future::err(Error::Request(RequestError::BadRequest)),
            StatusCode::FORBIDDEN => future::err(Error::Request(RequestError::Forbidden)),
            _ => future::err(Error::Response),
        }
    }
}

impl Client for AsyncClient {
    fn new(options: ClientOptions) -> Self {
        let client = AsyncClient::build_client(&options);
        AsyncClient { options, client }
    }
}
