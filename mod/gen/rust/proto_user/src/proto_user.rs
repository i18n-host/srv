// @generated, do not edit
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
      full_name: "user.String",
      fields: &[::pb_jelly::FieldDescriptor {
        name: "v",
        full_name: "user.String.v",
        index: 0,
        number: 1,
        typ: ::pb_jelly::wire_format::Type::LengthDelimited,
        label: ::pb_jelly::Label::Optional,
        oneof_index: None,
      }],
      oneofs: &[],
    })
  }
  fn compute_size(&self) -> usize {
    let mut size = 0usize;
    size += ::pb_jelly::helpers::compute_size_scalar::<::std::string::String>(
      &self.v,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    );
    size
  }
  fn serialize<W: ::pb_jelly::PbBufferWriter>(&self, w: &mut W) -> ::std::io::Result<()> {
    ::pb_jelly::helpers::serialize_scalar::<W, ::std::string::String>(
      w,
      &self.v,
      1,
      ::pb_jelly::wire_format::Type::LengthDelimited,
    )?;
    Ok(())
  }
  fn deserialize<B: ::pb_jelly::PbBufferReader>(
    &mut self,
    mut buf: &mut B,
  ) -> ::std::io::Result<()> {
    while let Some((field_number, typ)) = ::pb_jelly::wire_format::read(&mut buf)? {
      match field_number {
        1 => {
          let val = ::pb_jelly::helpers::deserialize_length_delimited::<B, ::std::string::String>(
            buf, typ, "String", 1,
          )?;
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
      "v" => ::pb_jelly::reflection::FieldMut::Value(&mut self.v),
      _ => {
        panic!("unknown field name given")
      }
    }
  }
}
