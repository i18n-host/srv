// @generated, do not edit
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct VerifyArgs {
  pub id: ::std::vec::Vec<u8>,
  pub click_pos_li: ::std::vec::Vec<u32>,
}
impl ::std::default::Default for VerifyArgs {
  fn default() -> Self {
    VerifyArgs {
      id: ::std::default::Default::default(),
      click_pos_li: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref VerifyArgs_default: VerifyArgs = VerifyArgs::default();
}
impl ::pb_jelly::Message for VerifyArgs {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "VerifyArgs",
      full_name: "captcha.VerifyArgs",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "id",
          full_name: "captcha.VerifyArgs.id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "click_pos_li",
          full_name: "captcha.VerifyArgs.click_pos_li",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::Varint,
          label: ::pb_jelly::Label::Repeated,
          oneof_index: None,
        },
      ],
      oneofs: &[
      ],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.id, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    if !self.click_pos_li.is_empty() {
      let mut click_pos_li_size = 0usize;
      for val in &self.click_pos_li {
        click_pos_li_size += ::pb_jelly::Message::compute_size(val);
      }
      size += ::pb_jelly::wire_format::serialized_length(2);
      size += ::pb_jelly::varint::serialized_length(click_pos_li_size as u64);
      size += click_pos_li_size;
    }
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.id, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    if !self.click_pos_li.is_empty() {
      let mut size = 0usize;
      for val in &self.click_pos_li {
        size += ::pb_jelly::Message::compute_size(val);
      }
      ::pb_jelly::wire_format::write(2, ::pb_jelly::wire_format::Type::LengthDelimited, w)?;
      ::pb_jelly::varint::write(size as u64, w)?;
      for val in &self.click_pos_li {
        ::pb_jelly::Message::serialize(val, w)?;
      }
    }
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "VerifyArgs", 1)?;
          self.id = val;
        }
        2 => {
          ::pb_jelly::helpers::deserialize_packed::<B, u32>(buf, typ, ::pb_jelly::wire_format::Type::Varint, "VerifyArgs", 2, &mut self.click_pos_li)?;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for VerifyArgs {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "id" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.id)
      }
      "click_pos_li" => {
        unimplemented!("Repeated fields are not currently supported.")
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct String {
  pub v: ::std::string::String,
}
impl ::std::default::Default for String {
  fn default() -> Self {
    String {
      v: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref String_default: String = String::default();
}
impl ::pb_jelly::Message for String {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "String",
      full_name: "captcha.String",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "v",
          full_name: "captcha.String.v",
          index: 0,
          number: 1,
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
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(&self.v, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(w, &self.v, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(buf, typ, "String", 1)?;
          self.v = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for String {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "v" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.v)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Captcha {
  pub id: ::std::vec::Vec<u8>,
  pub img: ::std::vec::Vec<u8>,
  pub tip: ::std::vec::Vec<u8>,
}
impl ::std::default::Default for Captcha {
  fn default() -> Self {
    Captcha {
      id: ::std::default::Default::default(),
      img: ::std::default::Default::default(),
      tip: ::std::default::Default::default(),
    }
  }
}
::lazy_static::lazy_static! {
  pub static ref Captcha_default: Captcha = Captcha::default();
}
impl ::pb_jelly::Message for Captcha {
  fn descriptor(&self) -> ::std::option::Option<::pb_jelly::MessageDescriptor> {
    Some(::pb_jelly::MessageDescriptor {
      name: "Captcha",
      full_name: "captcha.Captcha",
      fields: &[
        ::pb_jelly::FieldDescriptor {
          name: "id",
          full_name: "captcha.Captcha.id",
          index: 0,
          number: 1,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "img",
          full_name: "captcha.Captcha.img",
          index: 1,
          number: 2,
          typ: ::pb_jelly::wire_format::Type::LengthDelimited,
          label: ::pb_jelly::Label::Optional,
          oneof_index: None,
        },
        ::pb_jelly::FieldDescriptor {
          name: "tip",
          full_name: "captcha.Captcha.tip",
          index: 2,
          number: 3,
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
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.id, 1, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.img, 2, ::pb_jelly::wire_format::Type::LengthDelimited);
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::vec::Vec<u8>>(&self.tip, 3, ::pb_jelly::wire_format::Type::LengthDelimited);
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.id, 1, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.img, 2, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::vec::Vec<u8>>(w, &self.tip, 3, ::pb_jelly::wire_format::Type::LengthDelimited)?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(&mut self, mut buf: &mut B) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "Captcha", 1)?;
          self.id = val;
        }
        2 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "Captcha", 2)?;
          self.img = val;
        }
        3 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::vec::Vec<u8>>(buf, typ, "Captcha", 3)?;
          self.tip = val;
        }
        _ => {
          ::pb_jelly::skip(typ, &mut buf)?;
        }
      }
    }
    Ok(())
  }
}
impl ::pb_jelly::Reflection for Captcha {
  fn which_one_of(&self, oneof_name: &str) -> ::std::option::Option<&'static str> {
    match oneof_name {
      _ => {
        panic!("unknown oneof name given");
      }
    }
  }
  fn get_field_mut(&mut self, field_name: &str) -> ::pb_jelly::reflection::FieldMut<'_> {
    match field_name {
      "id" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.id)
      }
      "img" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.img)
      }
      "tip" => {
        ::pb_jelly::reflection::FieldMut::Value(&mut self.tip)
      }
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}

