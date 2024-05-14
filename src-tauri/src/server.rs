use async_trait::async_trait;
use multilink::http::hyper::{Body, Method, StatusCode, Uri};
use multilink::http::server::{HttpServer, HttpServerConfig};
use multilink::http::util::{
    notification_sse_response, parse_request, parse_response, serialize_to_http_request,
    serialize_to_http_response, validate_method,
};
use multilink::http::{HttpRequest, ModalHttpResponse, RequestHttpConvert, ResponseHttpConvert};
use multilink::tower::Service;
use multilink::{ProtocolError, ServiceError, ServiceFuture, ServiceResponse};
use serde::{Deserialize, Serialize};
use std::task::{Context, Poll};
use tauri::{AppHandle, Manager};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HelloRequest {
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HelloResponse {
    pub result: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestRequest {
    Hello(HelloRequest),
}

#[async_trait]
impl RequestHttpConvert<TestRequest> for TestRequest {
    async fn from_http_request(request: HttpRequest<Body>) -> Result<Option<Self>, ProtocolError> {
        let path = request.uri().path();
        let request = match path {
            "/hello" => {
                validate_method(&request, Method::POST)?;
                Self::Hello(parse_request(request).await?)
            }
            _ => return Ok(None),
        };
        Ok(Some(request))
    }

    fn to_http_request(&self, base_url: &Uri) -> Result<Option<HttpRequest<Body>>, ProtocolError> {
        let request = match self {
            Self::Hello(request) => {
                serialize_to_http_request(base_url, "/hello", Method::POST, &request)?
            }
        };
        Ok(Some(request))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestResponse {
    Hello(HelloResponse),
}

#[async_trait]
impl ResponseHttpConvert<TestRequest, TestResponse> for TestResponse {
    async fn from_http_response(
        response: ModalHttpResponse,
        original_request: &TestRequest,
    ) -> Result<Option<ServiceResponse<Self>>, ProtocolError> {
        Ok(Some(match response {
            ModalHttpResponse::Single(response) => match original_request {
                TestRequest::Hello(_) => {
                    ServiceResponse::Single(Self::Hello(parse_response(response).await?))
                }
            },
            ModalHttpResponse::Event(_) => return Ok(None),
        }))
    }

    fn to_http_response(
        response: ServiceResponse<Self>,
    ) -> Result<Option<ModalHttpResponse>, ProtocolError> {
        let response = match response {
            ServiceResponse::Single(response) => match response {
                Self::Hello(response) => ModalHttpResponse::Single(serialize_to_http_response(
                    &response,
                    StatusCode::OK,
                )?),
            },
            ServiceResponse::Multiple(stream) => {
                // Output a single server-side event HTTP response
                ModalHttpResponse::Single(notification_sse_response(stream))
            }
        };
        Ok(Some(response))
    }
}

#[derive(Clone)]
pub struct TestService {
    app_handle: AppHandle,
}

impl Service<TestRequest> for TestService {
    type Response = ServiceResponse<TestResponse>;
    type Error = ServiceError;
    type Future = ServiceFuture<ServiceResponse<TestResponse>>;

    fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: TestRequest) -> Self::Future {
        dbg!("REQUEST", &req);

        let handle = self.app_handle.clone();

        Box::pin(async move {
            Ok(match req {
                TestRequest::Hello(request) => {
                    let res = TestResponse::Hello(HelloResponse {
                        result: format!("Hello, {}!", request.name),
                    });

                    handle.emit_all("hello", res.clone()).unwrap();

                    ServiceResponse::Single(res)
                }
            })
        })
    }
}

pub async fn create_server(app_handle: AppHandle) {
    let service = TestService { app_handle };

    let server = HttpServer::new(
        service,
        HttpServerConfig {
            port: 6660,
            ..Default::default()
        },
    );

    server.run().await.unwrap()
}
