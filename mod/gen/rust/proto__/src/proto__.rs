// @generated, do not edit
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(u32)]
pub enum Func {
  None = 0,
  CaptchaVerify = 1,
  Captcha = 2,
  UserName = 3,
  AuthSigninMail = 4,
  AuthSignupMail = 5,
  AuthTest = 6,
  DemoCaptcha = 7,
  DemoManualCaptcha = 8,
}
impl Func {
  pub const KNOWN_VARIANTS: [Func; 9] = [Func::None, Func::CaptchaVerify, Func::Captcha, Func::UserName, Func::AuthSigninMail, Func::AuthSignupMail, Func::AuthTest, Func::DemoCaptcha, Func::DemoManualCaptcha];
}
impl ::std::default::Default for Func {
  fn default() -> Self {
    Func::None
  }
}
impl From<Func> for u32 {
  fn from(v: Func) -> u32 {
    match v {
      Func::None => 0,
      Func::CaptchaVerify => 1,
      Func::Captcha => 2,
      Func::UserName => 3,
      Func::AuthSigninMail => 4,
      Func::AuthSignupMail => 5,
      Func::AuthTest => 6,
      Func::DemoCaptcha => 7,
      Func::DemoManualCaptcha => 8,
    }
  }
}
impl ::std::convert::TryFrom<u32> for Func {
  type Error = u32;
  fn try_from(v: u32) -> ::std::result::Result<Self, u32> {
    match v {
      0 => Ok(Func::None),
      1 => Ok(Func::CaptchaVerify),
      2 => Ok(Func::Captcha),
      3 => Ok(Func::UserName),
      4 => Ok(Func::AuthSigninMail),
      5 => Ok(Func::AuthSignupMail),
      6 => Ok(Func::AuthTest),
      7 => Ok(Func::DemoCaptcha),
      8 => Ok(Func::DemoManualCaptcha),
      _ => Err(v),
    }
  }
}
impl ::pb_jelly::ProtoEnum for Func {
}
impl ::pb_jelly::ClosedProtoEnum for Func {
  fn name(self) -> &'static str {
    match self {
      Func::None => "None",
      Func::CaptchaVerify => "CaptchaVerify",
      Func::Captcha => "Captcha",
      Func::UserName => "UserName",
      Func::AuthSigninMail => "AuthSigninMail",
      Func::AuthSignupMail => "AuthSignupMail",
      Func::AuthTest => "AuthTest",
      Func::DemoCaptcha => "DemoCaptcha",
      Func::DemoManualCaptcha => "DemoManualCaptcha",
    }
  }
}

