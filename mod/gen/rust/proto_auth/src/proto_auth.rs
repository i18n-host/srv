// @generated, do not edit
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SigninMailArgs {
  pub address: ::std::string::String,
  pub password: ::std::string::String,
}
impl ::std::default::Default for SigninMailArgs {
  fn default() -> Self {
    SigninMailArgs {
      address: ::std::default::Default::default(),
      password: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref SigninMailArgs_default: SigninMailArgs = SigninMailArgs::default();
}
impl ::pb_jelly::Message for SigninMailArgs {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SigninMailArgs",
      full_name: "auth.SigninMailArgs",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "address",
          full_name: "auth.SigninMailArgs.address",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "password",
          full_name: "auth.SigninMailArgs.password",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SigninMailArgs", 1)?;
          self.address = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SigninMailArgs", 2)?;
          self.password = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for SigninMailArgs {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "address" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.address)
      }
      "password" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.password)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignupMailArgs {
  pub address: ::std::string::String,
  pub password: ::std::string::String,
}
impl ::std::default::Default for SignupMailArgs {
  fn default() -> Self {
    SignupMailArgs {
      address: ::std::default::Default::default(),
      password: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref SignupMailArgs_default: SignupMailArgs = SignupMailArgs::default();
}
impl ::pb_jelly::Message for SignupMailArgs {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "SignupMailArgs",
      full_name: "auth.SignupMailArgs",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "address",
          full_name: "auth.SignupMailArgs.address",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "password",
          full_name: "auth.SignupMailArgs.password",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.address, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.password, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SignupMailArgs", 1)?;
          self.address = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "SignupMailArgs", 2)?;
          self.password = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for SignupMailArgs {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "address" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.address)
      }
      "password" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.password)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TestArgs {
  pub timezone: ::pb_jelly::Signed32,
  pub dpi: u32,
  pub w: u32,
  pub h: u32,
  pub arch: ::std::string::String,
  pub model: ::std::string::String,
  pub cpu_num: u32,
  pub gpu: ::std::string::String,
  pub os_v1: u32,
  pub os_v2: u32,
}
impl ::std::default::Default for TestArgs {
  fn default() -> Self {
    TestArgs {
      timezone: ::std::default::Default::default(),
      dpi: ::std::default::Default::default(),
      w: ::std::default::Default::default(),
      h: ::std::default::Default::default(),
      arch: ::std::default::Default::default(),
      model: ::std::default::Default::default(),
      cpu_num: ::std::default::Default::default(),
      gpu: ::std::default::Default::default(),
      os_v1: ::std::default::Default::default(),
      os_v2: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref TestArgs_default: TestArgs = TestArgs::default();
}
impl ::pb_jelly::Message for TestArgs {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "TestArgs",
      full_name: "auth.TestArgs",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "timezone",
          full_name: "auth.TestArgs.timezone",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "dpi",
          full_name: "auth.TestArgs.dpi",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "w",
          full_name: "auth.TestArgs.w",
          index: 2,
          number: 3,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "h",
          full_name: "auth.TestArgs.h",
          index: 3,
          number: 4,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "arch",
          full_name: "auth.TestArgs.arch",
          index: 4,
          number: 5,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "model",
          full_name: "auth.TestArgs.model",
          index: 5,
          number: 6,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "cpu_num",
          full_name: "auth.TestArgs.cpu_num",
          index: 6,
          number: 7,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "gpu",
          full_name: "auth.TestArgs.gpu",
          index: 7,
          number: 8,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "os_v1",
          full_name: "auth.TestArgs.os_v1",
          index: 8,
          number: 9,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "os_v2",
          full_name: "auth.TestArgs.os_v2",
          index: 9,
          number: 10,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::pb_jelly::Signed32>(&self.timezone, 1, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.dpi, 2, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.w, 3, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.h, 4, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.arch, 5, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.model, 6, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.cpu_num, 7, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.gpu, 8, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.os_v1, 9, ::pb_jelly::wire_format::Type::Varint);
    size += ::pb_jelly::helpers::compute_size_scalar::<u32>(&self.os_v2, 10, ::pb_jelly::wire_format::Type::Varint);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::pb_jelly::Signed32>(w, &self.timezone, 1, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.dpi, 2, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.w, 3, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.h, 4, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.arch, 5, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.model, 6, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.cpu_num, 7, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.gpu, 8, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.os_v1, 9, ::pb_jelly::wire_format::Type::Varint)?;
    ::pb_jelly::helpers::serialize_scalar::<W, u32>(w, &self.os_v2, 10, ::pb_jelly::wire_format::Type::Varint)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, ::pb_jelly::Signed32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TestArgs", 1)?;
          self.timezone = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TestArgs", 2)?;
          self.dpi = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TestArgs", 3)?;
          self.w = val;
        }
        4 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TestArgs", 4)?;
          self.h = val;
        }
        5 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "TestArgs", 5)?;
          self.arch = val;
        }
        6 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "TestArgs", 6)?;
          self.model = val;
        }
        7 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TestArgs", 7)?;
          self.cpu_num = val;
        }
        8 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "TestArgs", 8)?;
          self.gpu = val;
        }
        9 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TestArgs", 9)?;
          self.os_v1 = val;
        }
        10 => {
          let val = ::pb_jelly::helpers::deserialize_known_length::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "TestArgs", 10)?;
          self.os_v2 = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for TestArgs {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "timezone" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.timezone)
      }
      "dpi" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.dpi)
      }
      "w" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.w)
      }
      "h" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.h)
      }
      "arch" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.arch)
      }
      "model" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.model)
      }
      "cpu_num" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.cpu_num)
      }
      "gpu" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.gpu)
      }
      "os_v1" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.os_v1)
      }
      "os_v2" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.os_v2)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

