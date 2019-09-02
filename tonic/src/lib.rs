#![recursion_limit = "512"]

//! gRPC implementation

pub mod body;
pub mod client;
pub mod codec;
#[doc(hidden)]
pub mod error;
pub mod metadata;
pub mod server;

#[cfg(feature = "transport")]
pub mod transport;

mod request;
mod response;
mod status;

pub use body::BoxBody;
pub use request::Request;
pub use response::Response;
pub use status::{Code, Status};
pub use tonic_macros::{client, server};

pub(crate) use error::Error;

use crate::body::Body;
use http_body::Body as HttpBody;
use std::future::Future;
use std::task::{Context, Poll};
use tower_service::Service;

pub trait GrpcService<ReqBody> {
    type ResponseBody: Body + HttpBody;
    type Error: Into<crate::Error>;

    type Future: Future<Output = Result<http::Response<Self::ResponseBody>, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>>;

    fn call(&mut self, request: http::Request<ReqBody>) -> Self::Future;
}

impl<T, ReqBody, ResBody> GrpcService<ReqBody> for T
where
    T: Service<http::Request<ReqBody>, Response = http::Response<ResBody>>,
    T::Error: Into<crate::Error>,
    ResBody: Body + HttpBody,
    <ResBody as HttpBody>::Error: Into<crate::Error>,
{
    type ResponseBody = ResBody;
    type Error = T::Error;
    type Future = T::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Service::poll_ready(self, cx)
    }

    fn call(&mut self, request: http::Request<ReqBody>) -> Self::Future {
        Service::call(self, request)
    }
}

#[doc(hidden)]

pub mod _codegen {
    pub use futures_core::Stream;
    pub use futures_util::future::{ok, poll_fn, Ready};
    pub use http_body::Body as HttpBody;
    pub use std::future::Future;
    pub use std::pin::Pin;
    pub use std::task::{Context, Poll};
    pub use tower_service::Service;

    pub type BoxFuture<T, E> =
        self::Pin<Box<dyn self::Future<Output = Result<T, E>> + Send + 'static>>;
    pub type BoxStream<T> =
        self::Pin<Box<dyn futures_core::Stream<Item = Result<T, crate::Status>> + Send + 'static>>;

    pub mod http {
        pub use http::*;
    }
}
