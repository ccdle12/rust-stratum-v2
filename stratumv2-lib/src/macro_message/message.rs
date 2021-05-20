pub mod macro_prelude {
    pub use crate::error::Result;
    pub use crate::frame::Frameable;
    pub use crate::parse::{ByteParser, Deserializable, Serializable};
    pub use crate::types::MessageType;
    pub use std::io;
}

/// Internal macro to build all the common requirements for a Stratum-v2 message.
#[doc(hidden)]
#[macro_export]
macro_rules! impl_message {
    (
      $(#[$doc_comment:meta])*
      $struct_name:ident,
      $message_type:path,
      $($(#[$field_comment:meta])*
      $field: ident $field_type:ident),*
    ) => {
    use crate::macro_message::message::macro_prelude::*;

    $(#[$doc_comment])*
    #[derive(Debug, Clone, PartialEq)]
    pub struct $struct_name {
      $(
        $(#[$field_comment])*
          pub $field: $field_type
      ),*
    }

    impl Serializable for $struct_name {
      fn serialize<W: io::Write>(&self, writer: &mut W) -> Result<usize> {
        Ok([
          $(self.$field.serialize(writer)?,)*
        ]
        .iter()
        .sum())
      }
    }

    impl Deserializable for $struct_name {
      fn deserialize(parser: &mut ByteParser) -> Result<Self> {
        $struct_name::new(
          $($field_type::deserialize(parser)?,)*
        )
      }
    }

    impl Frameable for $struct_name {
      fn message_type() -> MessageType {
        $message_type
      }
    }
  };
}
