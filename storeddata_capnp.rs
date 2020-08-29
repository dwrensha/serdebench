// @generated by the capnpc-rust plugin to the Cap'n Proto schema compiler.
// DO NOT EDIT.
// source: storeddata.capnp


pub mod stored_data {
  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_variant(self) -> crate::storeddata_capnp::stored_data::variant::Reader<'a> {
      ::capnp::traits::FromStructReader::new(self.reader)
    }
    #[inline]
    pub fn get_opt_bool(self) -> ::capnp::Result<crate::storeddata_capnp::option_bool::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(1), ::core::option::Option::None)
    }
    pub fn has_opt_bool(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_vec_strs(self) -> ::capnp::Result<::capnp::text_list::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(2), ::core::option::Option::None)
    }
    pub fn has_vec_strs(&self) -> bool {
      !self.reader.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_range(self) -> ::capnp::Result<crate::storeddata_capnp::range::Reader<'a>> {
      ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(3), ::core::option::Option::None)
    }
    pub fn has_range(&self) -> bool {
      !self.reader.get_pointer_field(3).is_null()
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn reborrow(&mut self) -> Builder<> {
      Builder { .. *self }
    }
    pub fn reborrow_as_reader(&self) -> Reader<> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn get_variant(self) -> crate::storeddata_capnp::stored_data::variant::Builder<'a> {
      ::capnp::traits::FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn init_variant(self, ) -> crate::storeddata_capnp::stored_data::variant::Builder<'a> {
      self.builder.set_data_field::<u16>(1, 0);
      self.builder.set_bool_field(0, false);
      self.builder.set_data_field::<u8>(0, 0u8);
      self.builder.set_data_field::<i64>(1, 0i64);
      self.builder.get_pointer_field(0).clear();
      ::capnp::traits::FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn get_opt_bool(self) -> ::capnp::Result<crate::storeddata_capnp::option_bool::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(1), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_opt_bool<'b>(&mut self, value: crate::storeddata_capnp::option_bool::Reader<'b>) -> ::capnp::Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(1), value, false)
    }
    #[inline]
    pub fn init_opt_bool(self, ) -> crate::storeddata_capnp::option_bool::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(1), 0)
    }
    pub fn has_opt_bool(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
    #[inline]
    pub fn get_vec_strs(self) -> ::capnp::Result<::capnp::text_list::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(2), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_vec_strs(&mut self, value: ::capnp::text_list::Reader<'a>) -> ::capnp::Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(2), value, false)
    }
    #[inline]
    pub fn init_vec_strs(self, size: u32) -> ::capnp::text_list::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(2), size)
    }
    pub fn has_vec_strs(&self) -> bool {
      !self.builder.get_pointer_field(2).is_null()
    }
    #[inline]
    pub fn get_range(self) -> ::capnp::Result<crate::storeddata_capnp::range::Builder<'a>> {
      ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(3), ::core::option::Option::None)
    }
    #[inline]
    pub fn set_range<'b>(&mut self, value: crate::storeddata_capnp::range::Reader<'b>) -> ::capnp::Result<()> {
      ::capnp::traits::SetPointerBuilder::set_pointer_builder(self.builder.get_pointer_field(3), value, false)
    }
    #[inline]
    pub fn init_range(self, ) -> crate::storeddata_capnp::range::Builder<'a> {
      ::capnp::traits::FromPointerBuilder::init_pointer(self.builder.get_pointer_field(3), 0)
    }
    pub fn has_range(&self) -> bool {
      !self.builder.get_pointer_field(3).is_null()
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
    pub fn get_variant(&self) -> crate::storeddata_capnp::stored_data::variant::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.noop())
    }
    pub fn get_opt_bool(&self) -> crate::storeddata_capnp::option_bool::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(1))
    }
    pub fn get_range(&self) -> crate::storeddata_capnp::range::Pipeline {
      ::capnp::capability::FromTypelessPipeline::new(self._typeless.get_pointer_field(3))
    }
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 2, pointers: 4 };
    pub const TYPE_ID: u64 = 0x897d_8080_b2e1_4a2a;
  }

  pub mod variant {
    pub use self::Which::{YesNo,Small,Signy,Stringy};

    #[derive(Copy, Clone)]
    pub struct Owned;
    impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
    impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

    #[derive(Clone, Copy)]
    pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

    impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
      fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
        Reader { reader,  }
      }
    }

    impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
      fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
      }
    }

    impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
      fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
        self.reader
      }
    }

    impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
      fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
        self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
      }
    }

    impl <'a,> Reader<'a,>  {
      pub fn reborrow(&self) -> Reader<> {
        Reader { .. *self }
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.reader.total_size()
      }
      pub fn has_stringy(&self) -> bool {
        if self.reader.get_data_field::<u16>(1) != 3 { return false; }
        !self.reader.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn which(self) -> ::core::result::Result<WhichReader<'a,>, ::capnp::NotInSchema> {
        match self.reader.get_data_field::<u16>(1) {
          0 => {
            ::core::result::Result::Ok(YesNo(
              self.reader.get_bool_field(0)
            ))
          }
          1 => {
            ::core::result::Result::Ok(Small(
              self.reader.get_data_field::<u8>(0)
            ))
          }
          2 => {
            ::core::result::Result::Ok(Signy(
              self.reader.get_data_field::<i64>(1)
            ))
          }
          3 => {
            ::core::result::Result::Ok(Stringy(
              ::capnp::traits::FromPointerReader::get_from_pointer(&self.reader.get_pointer_field(0), ::core::option::Option::None)
            ))
          }
          x => ::core::result::Result::Err(::capnp::NotInSchema(x))
        }
      }
    }

    pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
    impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
      #[inline]
      fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
    }
    impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
      #[inline]
      fn type_id() -> u64 { _private::TYPE_ID }
    }
    impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
      fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
        Builder { builder,  }
      }
    }

    impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
      fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
        self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
      }
    }

    impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
      fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
        ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
      }
      fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
        ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
      }
    }

    impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
      fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
    }

    impl <'a,> Builder<'a,>  {
      pub fn into_reader(self) -> Reader<'a,> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }
      pub fn reborrow(&mut self) -> Builder<> {
        Builder { .. *self }
      }
      pub fn reborrow_as_reader(&self) -> Reader<> {
        ::capnp::traits::FromStructReader::new(self.builder.into_reader())
      }

      pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
        self.builder.into_reader().total_size()
      }
      #[inline]
      pub fn set_yes_no(&mut self, value: bool)  {
        self.builder.set_data_field::<u16>(1, 0);
        self.builder.set_bool_field(0, value);
      }
      #[inline]
      pub fn set_small(&mut self, value: u8)  {
        self.builder.set_data_field::<u16>(1, 1);
        self.builder.set_data_field::<u8>(0, value);
      }
      #[inline]
      pub fn set_signy(&mut self, value: i64)  {
        self.builder.set_data_field::<u16>(1, 2);
        self.builder.set_data_field::<i64>(1, value);
      }
      #[inline]
      pub fn set_stringy(&mut self, value: ::capnp::text::Reader)  {
        self.builder.set_data_field::<u16>(1, 3);
        self.builder.get_pointer_field(0).set_text(value);
      }
      #[inline]
      pub fn init_stringy(self, size: u32) -> ::capnp::text::Builder<'a> {
        self.builder.set_data_field::<u16>(1, 3);
        self.builder.get_pointer_field(0).init_text(size)
      }
      pub fn has_stringy(&self) -> bool {
        if self.builder.get_data_field::<u16>(1) != 3 { return false; }
        !self.builder.get_pointer_field(0).is_null()
      }
      #[inline]
      pub fn which(self) -> ::core::result::Result<WhichBuilder<'a,>, ::capnp::NotInSchema> {
        match self.builder.get_data_field::<u16>(1) {
          0 => {
            ::core::result::Result::Ok(YesNo(
              self.builder.get_bool_field(0)
            ))
          }
          1 => {
            ::core::result::Result::Ok(Small(
              self.builder.get_data_field::<u8>(0)
            ))
          }
          2 => {
            ::core::result::Result::Ok(Signy(
              self.builder.get_data_field::<i64>(1)
            ))
          }
          3 => {
            ::core::result::Result::Ok(Stringy(
              ::capnp::traits::FromPointerBuilder::get_from_pointer(self.builder.get_pointer_field(0), ::core::option::Option::None)
            ))
          }
          x => ::core::result::Result::Err(::capnp::NotInSchema(x))
        }
      }
    }

    pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
    impl ::capnp::capability::FromTypelessPipeline for Pipeline {
      fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
        Pipeline { _typeless: typeless,  }
      }
    }
    impl Pipeline  {
    }
    mod _private {
      use capnp::private::layout;
      pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 2, pointers: 4 };
      pub const TYPE_ID: u64 = 0x9bf5_04d5_d076_4b23;
    }
    pub enum Which<A0> {
      YesNo(bool),
      Small(u8),
      Signy(i64),
      Stringy(A0),
    }
    pub type WhichReader<'a,> = Which<::capnp::Result<::capnp::text::Reader<'a>>>;
    pub type WhichBuilder<'a,> = Which<::capnp::Result<::capnp::text::Builder<'a>>>;
  }
}

pub mod range {
  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_start(self) -> u64 {
      self.reader.get_data_field::<u64>(0)
    }
    #[inline]
    pub fn get_end(self) -> u64 {
      self.reader.get_data_field::<u64>(1)
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn reborrow(&mut self) -> Builder<> {
      Builder { .. *self }
    }
    pub fn reborrow_as_reader(&self) -> Reader<> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn get_start(self) -> u64 {
      self.builder.get_data_field::<u64>(0)
    }
    #[inline]
    pub fn set_start(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(0, value);
    }
    #[inline]
    pub fn get_end(self) -> u64 {
      self.builder.get_data_field::<u64>(1)
    }
    #[inline]
    pub fn set_end(&mut self, value: u64)  {
      self.builder.set_data_field::<u64>(1, value);
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 2, pointers: 0 };
    pub const TYPE_ID: u64 = 0xa9cc_76af_db16_cad9;
  }
}

pub mod option_bool {
  #[derive(Copy, Clone)]
  pub struct Owned;
  impl <'a> ::capnp::traits::Owned<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl <'a> ::capnp::traits::OwnedStruct<'a> for Owned { type Reader = Reader<'a>; type Builder = Builder<'a>; }
  impl ::capnp::traits::Pipelined for Owned { type Pipeline = Pipeline; }

  #[derive(Clone, Copy)]
  pub struct Reader<'a> { reader: ::capnp::private::layout::StructReader<'a> }

  impl <'a,> ::capnp::traits::HasTypeId for Reader<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructReader<'a> for Reader<'a,>  {
    fn new(reader: ::capnp::private::layout::StructReader<'a>) -> Reader<'a,> {
      Reader { reader,  }
    }
  }

  impl <'a,> ::capnp::traits::FromPointerReader<'a> for Reader<'a,>  {
    fn get_from_pointer(reader: &::capnp::private::layout::PointerReader<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Reader<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructReader::new(reader.get_struct(default)?))
    }
  }

  impl <'a,> ::capnp::traits::IntoInternalStructReader<'a> for Reader<'a,>  {
    fn into_internal_struct_reader(self) -> ::capnp::private::layout::StructReader<'a> {
      self.reader
    }
  }

  impl <'a,> ::capnp::traits::Imbue<'a> for Reader<'a,>  {
    fn imbue(&mut self, cap_table: &'a ::capnp::private::layout::CapTable) {
      self.reader.imbue(::capnp::private::layout::CapTableReader::Plain(cap_table))
    }
  }

  impl <'a,> Reader<'a,>  {
    pub fn reborrow(&self) -> Reader<> {
      Reader { .. *self }
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.reader.total_size()
    }
    #[inline]
    pub fn get_value(self) -> bool {
      self.reader.get_bool_field(0)
    }
  }

  pub struct Builder<'a> { builder: ::capnp::private::layout::StructBuilder<'a> }
  impl <'a,> ::capnp::traits::HasStructSize for Builder<'a,>  {
    #[inline]
    fn struct_size() -> ::capnp::private::layout::StructSize { _private::STRUCT_SIZE }
  }
  impl <'a,> ::capnp::traits::HasTypeId for Builder<'a,>  {
    #[inline]
    fn type_id() -> u64 { _private::TYPE_ID }
  }
  impl <'a,> ::capnp::traits::FromStructBuilder<'a> for Builder<'a,>  {
    fn new(builder: ::capnp::private::layout::StructBuilder<'a>) -> Builder<'a, > {
      Builder { builder,  }
    }
  }

  impl <'a,> ::capnp::traits::ImbueMut<'a> for Builder<'a,>  {
    fn imbue_mut(&mut self, cap_table: &'a mut ::capnp::private::layout::CapTable) {
      self.builder.imbue(::capnp::private::layout::CapTableBuilder::Plain(cap_table))
    }
  }

  impl <'a,> ::capnp::traits::FromPointerBuilder<'a> for Builder<'a,>  {
    fn init_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, _size: u32) -> Builder<'a,> {
      ::capnp::traits::FromStructBuilder::new(builder.init_struct(_private::STRUCT_SIZE))
    }
    fn get_from_pointer(builder: ::capnp::private::layout::PointerBuilder<'a>, default: ::core::option::Option<&'a [capnp::Word]>) -> ::capnp::Result<Builder<'a,>> {
      ::core::result::Result::Ok(::capnp::traits::FromStructBuilder::new(builder.get_struct(_private::STRUCT_SIZE, default)?))
    }
  }

  impl <'a,> ::capnp::traits::SetPointerBuilder<Builder<'a,>> for Reader<'a,>  {
    fn set_pointer_builder<'b>(pointer: ::capnp::private::layout::PointerBuilder<'b>, value: Reader<'a,>, canonicalize: bool) -> ::capnp::Result<()> { pointer.set_struct(&value.reader, canonicalize) }
  }

  impl <'a,> Builder<'a,>  {
    pub fn into_reader(self) -> Reader<'a,> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }
    pub fn reborrow(&mut self) -> Builder<> {
      Builder { .. *self }
    }
    pub fn reborrow_as_reader(&self) -> Reader<> {
      ::capnp::traits::FromStructReader::new(self.builder.into_reader())
    }

    pub fn total_size(&self) -> ::capnp::Result<::capnp::MessageSize> {
      self.builder.into_reader().total_size()
    }
    #[inline]
    pub fn get_value(self) -> bool {
      self.builder.get_bool_field(0)
    }
    #[inline]
    pub fn set_value(&mut self, value: bool)  {
      self.builder.set_bool_field(0, value);
    }
  }

  pub struct Pipeline { _typeless: ::capnp::any_pointer::Pipeline }
  impl ::capnp::capability::FromTypelessPipeline for Pipeline {
    fn new(typeless: ::capnp::any_pointer::Pipeline) -> Pipeline {
      Pipeline { _typeless: typeless,  }
    }
  }
  impl Pipeline  {
  }
  mod _private {
    use capnp::private::layout;
    pub const STRUCT_SIZE: layout::StructSize = layout::StructSize { data: 1, pointers: 0 };
    pub const TYPE_ID: u64 = 0xf1b0_5bbf_e7dd_740a;
  }
}
