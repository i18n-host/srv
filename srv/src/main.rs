use aok::Result;
use hpc_srv::srv;

#[static_init::constructor(0)]
extern "C" fn init() {
  loginit::init()
}

#[tokio::main]
async fn main() -> Result<()> {
  static_::init().await?;
  srv().await
}
