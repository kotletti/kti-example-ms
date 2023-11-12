pub mod health_http_controller {

  use std::convert::Infallible;

  use http_body_util::Full;
  use hyper::{body::Bytes, Method, Request, Response, StatusCode, Uri};

  type ReturnType = Result<Response<Full<Bytes>>, Infallible>;

  const SUCCESS_MESSAGE: &'static str = "SUCCESS\n";
  const NOT_FOUND_MESSAGE: &'static str = "NOT_FOUND\n";

  pub async fn router(
    request: Request<hyper::body::Incoming>,
  ) -> Result<Response<Full<Bytes>>, Infallible> {
    let method = request.method();
    let endpoint = request.uri();

    handle_method(method, endpoint).await
  }

  async fn handle_method(method: &Method, endpoint: &Uri) -> ReturnType {
    match method {
      &Method::GET => handle_get_endpoint(endpoint).await,
      _ => not_found().await,
    }
  }

  async fn handle_get_endpoint(endpoint: &Uri) -> ReturnType {
    match endpoint.path() {
      "/" => not_found().await,
      "/startup" => get_startup().await,
      "/readiness" => get_readiness().await,
      "/liveness" => get_liveness().await,
      _ => not_found().await,
    }
  }

  async fn get_startup() -> ReturnType {
    Ok(Response::new(Full::new(Bytes::from(SUCCESS_MESSAGE))))
  }

  async fn get_readiness() -> ReturnType {
    Ok(Response::new(Full::new(Bytes::from(SUCCESS_MESSAGE))))
  }

  async fn get_liveness() -> ReturnType {
    Ok(Response::new(Full::new(Bytes::from(SUCCESS_MESSAGE))))
  }

  async fn not_found() -> ReturnType {
    let mut response = Response::new(Full::new(Bytes::from(NOT_FOUND_MESSAGE)));

    *response.status_mut() = StatusCode::NOT_FOUND;

    Ok(response)
  }
}
