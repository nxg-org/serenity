use std::borrow::Cow;

use reqwest::{
    header::{
        HeaderMap as Headers,
        HeaderValue,
        AUTHORIZATION,
        CONTENT_LENGTH,
        CONTENT_TYPE,
        USER_AGENT,
    },
    Url,
};
use reqwest::{Client, RequestBuilder as ReqwestRequestBuilder};
use tracing::instrument;

use super::{routing::RouteInfo, HttpError};
use crate::constants;

pub struct RequestBuilder<'a> {
    body: Option<&'a [u8]>,
    headers: Option<Headers>,
    route: RouteInfo<'a>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(route_info: RouteInfo<'a>) -> Self {
        Self {
            body: None,
            headers: None,
            route: route_info,
        }
    }

    pub fn build(self) -> Request<'a> {
        Request::new(self)
    }

    pub fn body(&mut self, body: Option<&'a [u8]>) -> &mut Self {
        self.body = body;

        self
    }

    pub fn headers(&mut self, headers: Option<Headers>) -> &mut Self {
        self.headers = headers;

        self
    }

    pub fn route(&mut self, route_info: RouteInfo<'a>) -> &mut Self {
        self.route = route_info;

        self
    }
}

#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub(super) body: Option<&'a [u8]>,
    pub(super) headers: Option<Headers>,
    pub(super) route: RouteInfo<'a>,
}

impl<'a> Request<'a> {
    pub fn new(builder: RequestBuilder<'a>) -> Self {
        let RequestBuilder {
            body,
            headers,
            route,
        } = builder;

        Self {
            body,
            headers,
            route,
        }
    }

    #[instrument(skip(token))]
    pub fn build(
        &'a self,
        client: &Client,
        token: &str,
        proxy: Option<&Url>,
    ) -> Result<ReqwestRequestBuilder, HttpError> {
        let Request {
            body,
            headers: ref request_headers,
            route: ref route_info,
        } = *self;

        let (method, _, mut path) = route_info.deconstruct();

        if let Some(proxy) = proxy {
            path = Cow::Owned(path.to_mut().replace("https://discord.com/", proxy.as_str()));
        }

        let mut builder = client.request(method.reqwest_method(), Url::parse(&path)?);

        if let Some(bytes) = body {
            builder = builder.body(Vec::from(bytes));
        }

        let mut headers = if let Some(request_headers) = request_headers {
            let mut h = request_headers.clone();
            h.reserve(3);
            h
        } else {
            let mut h = Headers::with_capacity(4);
            h.insert(USER_AGENT, HeaderValue::from_static(constants::USER_AGENT));
            h
        };
       
        headers
            .insert(AUTHORIZATION, HeaderValue::from_str(token).map_err(HttpError::InvalidHeader)?);

        // Discord will return a 400: Bad Request response if we set the content type header,
        // but don't give a body.
        if self.body.is_some() && !headers.contains_key(CONTENT_TYPE) {
            headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }

        headers.insert(
            CONTENT_LENGTH,
            HeaderValue::from_str(&body.unwrap_or(&Vec::new()).len().to_string())
                .map_err(HttpError::InvalidHeader)?,
        );
        
        pub(super) mod reqwest_private {
            // Hey guys, I don't use all of this stuff. :(
            #![allow(unused)]
            pub struct Request {
                method: reqwest::Method,
                url: reqwest::Url,
                pub headers: reqwest::header::HeaderMap,
                body: Option<reqwest::Body>,
                timeout: Option<std::time::Duration>,
                version: http_crate::Version,
            }
            pub struct RequestBuilder {
                client: reqwest::Client,
                pub request: reqwest::Result<Request>,
            }
        }

        // only replace headers that don't already exist
        if let Ok(req) = &mut unsafe{ core::mem::transmute::<_, &mut reqwest_private::RequestBuilder>(&mut builder)}.request {
            for (k, v) in headers {
                if let Some(k) = k {
                    if let None = req.headers.get(&k) {
                        req.headers.insert(k, v);
                    }
                }
            }
        }

        Ok(builder)
    }

    pub fn body_ref(&self) -> &Option<&'a [u8]> {
        &self.body
    }

    pub fn body_mut(&mut self) -> &mut Option<&'a [u8]> {
        &mut self.body
    }

    pub fn headers_ref(&self) -> &Option<Headers> {
        &self.headers
    }

    pub fn headers_mut(&mut self) -> &mut Option<Headers> {
        &mut self.headers
    }

    pub fn route_ref(&self) -> &RouteInfo<'_> {
        &self.route
    }

    pub fn route_mut(&mut self) -> &mut RouteInfo<'a> {
        &mut self.route
    }
}
