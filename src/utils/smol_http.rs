use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

use hyper::{Client, Uri, Response};
use serde::Deserialize;
use smol::net::unix::UnixStream;

use crate::{Error, Result};

use super::{IntoGetQuery, Wrapper};

pub async fn request<R: for<'de> Deserialize<'de>>(
    stream: UnixStream,
    req: impl IntoGetQuery,
) -> Result<()> {
    let s = req.into_get_query();

    let connector = SmolConnector {
        stream,
    };

    let resp: Response<String> = Client::builder()
        .executor(SmolExecutor)
        .build(connector).request(s).await?;

    Ok(())
}

#[derive(Clone)]
struct SmolConnector {
    pub(crate) stream: UnixStream,
}

impl hyper::service::Service<Uri> for SmolConnector {
    type Response = UnixStream;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response>> + Send>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _uri: Uri) -> Self::Future {
        let stream = self.stream.clone();

        Box::pin(async move { Ok(stream) })
    }
}

#[derive(Clone)]
struct SmolExecutor;

impl<F: Future + Send + 'static> hyper::rt::Executor<F> for SmolExecutor {
    fn execute(&self, fut: F) {
        smol::spawn(async { drop(fut.await) }).detach();
    }
}
