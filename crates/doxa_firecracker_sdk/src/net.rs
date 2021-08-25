use hyper::{Body, Client, Method, Request, Response};
use hyperlocal::{UnixClientExt, Uri};
use std::path::Path;

use crate::error::ErrorStatusCode;

/// Uri is relative to the base socket address
pub(crate) async fn send_socket_request(
    socket: impl AsRef<Path>,
    method: Method,
    body: Body,
    uri: &str,
) -> Result<Response<Body>, crate::error::RequestError> {
    let url = Uri::new(socket, uri);

    let client = Client::unix();

    let request = Request::builder().uri(url).method(method).body(body)?;

    let response = client.request(request).await?;

    Ok(response)
}

pub(crate) fn expect_ok_response(response: &Response<Body>) -> Result<(), ErrorStatusCode> {
    if response.status().is_success() {
        Ok(())
    } else {
        Err(ErrorStatusCode)
    }
}
