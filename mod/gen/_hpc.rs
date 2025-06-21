//! AUTO GEN BY rust2proto , DO NOT EDIT

use dstr::dvec;
use hpc::{args_decode, call_err};
use hpc_captcha::{Captcha, GenCaptcha};
use ih::{CodeBody, State};
use r#mod::*;
use pb::Func;
use pb_jelly::Message;

pub struct Hpc;

impl hpc::Hpc for Hpc {
  type Func = Func;

  async fn run<G: GenCaptcha>(
    ctx: &ctx_::Ctx,
    func: Func,
    args: &[u8],
    captcha: &Captcha<G>,
  ) -> hpc::Result<CodeBody> {
    Ok(match func {
      Func::None => (State::OK, vec![]),

      Func::CaptchaVerify => {
        let args: pb::captcha::VerifyArgs = args_decode(args, "captcha::Verify")?;
        match captcha::verify(&args.id, &args.click_pos_li).await {
          Err(err) => {
            call_err("captcha::verify", err, captcha, || {
              dvec![&args.id, &args.click_pos_li].join(",")
            })
            .await?
          }
          Ok(r) => (State::OK, pb::captcha::String { v: r }.serialize_to_vec()),
        }
      }

      Func::Captcha => match captcha::captcha().await {
        Err(err) => call_err("captcha::captcha", err, captcha, || s_::EMPTY).await?,
        Ok(r) => (
          State::OK,
          pb::captcha::Captcha {
            id: r.id,
            img: r.img,
            tip: r.tip,
          }
          .serialize_to_vec(),
        ),
      },

      Func::UserName => {
        let user = ctx_::get(ctx).await?;
        match user::name(user).await {
          Err(err) => call_err("user::name", err, captcha, || dvec![&user].join(",")).await?,
          Ok(r) => (State::OK, pb::user::String { v: r }.serialize_to_vec()),
        }
      }

      Func::AuthSigninMail => {
        ctx_::captcha(ctx, captcha).await?;
        let args: pb::auth::SigninMailArgs = args_decode(args, "auth::SigninMail")?;
        let headers = &ctx.req.headers;
        let set_header = ctx_::sync::get(ctx);
        let browser = ctx.req.extensions.get().unwrap();
        match auth::signin::mail(&args.address, &args.password, headers, set_header, browser).await
        {
          Err(err) => {
            call_err("auth::signin::mail", err, captcha, || {
              dvec![
                &args.address,
                &args.password,
                &headers,
                &set_header,
                &browser
              ]
              .join(",")
            })
            .await?
          }
          Ok(_) => (State::OK, vec![]),
        }
      }

      Func::AuthSignupMail => {
        ctx_::captcha(ctx, captcha).await?;
        let args: pb::auth::SignupMailArgs = args_decode(args, "auth::SignupMail")?;
        let headers = &ctx.req.headers;
        match auth::signup::mail(&args.address, &args.password, headers).await {
          Err(err) => {
            call_err("auth::signup::mail", err, captcha, || {
              dvec![&args.address, &args.password, &headers].join(",")
            })
            .await?
          }
          Ok(_) => (State::OK, vec![]),
        }
      }

      Func::DemoCaptcha => {
        ctx_::captcha(ctx, captcha).await?;
        let headers = &ctx.req.headers;
        match demo::captcha(headers).await {
          Err(err) => call_err("demo::captcha", err, captcha, || dvec![&headers].join(",")).await?,
          Ok(_) => (State::OK, vec![]),
        }
      }

      Func::DemoManualCaptcha => {
        let headers = &ctx.req.headers;
        match demo::manual_captcha(headers).await {
          Err(err) => {
            call_err("demo::manual_captcha", err, captcha, || {
              dvec![&headers].join(",")
            })
            .await?
          }
          Ok(_) => (State::OK, vec![]),
        }
      }
    })
  }
}
