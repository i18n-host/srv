use aok::Result;
use pb_jelly::Message;

pub(crate) struct GenCaptcha;

impl hpc_captcha::GenCaptcha for GenCaptcha {
  async fn get() -> Result<Vec<u8>> {
    let c = r#mod::captcha::captcha().await?;
    Ok(
      pb::captcha::Captcha {
        id: c.id,
        img: c.img,
        tip: c.tip,
      }
      .serialize_to_vec(),
    )
  }
}
