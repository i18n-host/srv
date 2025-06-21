mod gen_captcha;

use aok::Void;
use axum::{Router, extract::Request, response::Response};
use gen_captcha::GenCaptcha;
mod _hpc;

const BATCH_LIMIT: usize = 64;

#[axum::debug_handler]
pub async fn hpc(request: Request) -> Response {
  hpc::run::<BATCH_LIMIT, _hpc::Hpc, GenCaptcha>(request).await
}

genv::def!(PORT:u16 | 2025);

pub async fn srv() -> Void {
  let port = PORT();
  let router = Router::new();
  hpc::srv(port, router, "/", hpc).await
}
