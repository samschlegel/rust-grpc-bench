// This file is generated by rust-protobuf 2.14.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `helloworld/helloworld.proto`

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_14_0;

#[derive(PartialEq,Clone,Default)]
pub struct HelloRequest {
    // message fields
    pub name: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HelloRequest {
    fn default() -> &'a HelloRequest {
        <HelloRequest as ::protobuf::Message>::default_instance()
    }
}

impl HelloRequest {
    pub fn new() -> HelloRequest {
        ::std::default::Default::default()
    }

    // uint64 name = 1;


    pub fn get_name(&self) -> u64 {
        self.name
    }
    pub fn clear_name(&mut self) {
        self.name = 0;
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: u64) {
        self.name = v;
    }
}

impl ::protobuf::Message for HelloRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.name = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.name != 0 {
            my_size += ::protobuf::rt::value_size(1, self.name, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.name != 0 {
            os.write_uint64(1, self.name)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> HelloRequest {
        HelloRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "name",
                    |m: &HelloRequest| { &m.name },
                    |m: &mut HelloRequest| { &mut m.name },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<HelloRequest>(
                    "HelloRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static HelloRequest {
        static mut instance: ::protobuf::lazy::Lazy<HelloRequest> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(HelloRequest::new)
        }
    }
}

impl ::protobuf::Clear for HelloRequest {
    fn clear(&mut self) {
        self.name = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HelloRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HelloRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct HelloReply {
    // message fields
    pub message: u64,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a HelloReply {
    fn default() -> &'a HelloReply {
        <HelloReply as ::protobuf::Message>::default_instance()
    }
}

impl HelloReply {
    pub fn new() -> HelloReply {
        ::std::default::Default::default()
    }

    // uint64 message = 1;


    pub fn get_message(&self) -> u64 {
        self.message
    }
    pub fn clear_message(&mut self) {
        self.message = 0;
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: u64) {
        self.message = v;
    }
}

impl ::protobuf::Message for HelloReply {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.message = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.message != 0 {
            my_size += ::protobuf::rt::value_size(1, self.message, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.message != 0 {
            os.write_uint64(1, self.message)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> HelloReply {
        HelloReply::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "message",
                    |m: &HelloReply| { &m.message },
                    |m: &mut HelloReply| { &mut m.message },
                ));
                ::protobuf::reflect::MessageDescriptor::new_pb_name::<HelloReply>(
                    "HelloReply",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }

    fn default_instance() -> &'static HelloReply {
        static mut instance: ::protobuf::lazy::Lazy<HelloReply> = ::protobuf::lazy::Lazy::INIT;
        unsafe {
            instance.get(HelloReply::new)
        }
    }
}

impl ::protobuf::Clear for HelloReply {
    fn clear(&mut self) {
        self.message = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for HelloReply {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for HelloReply {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x1bhelloworld/helloworld.proto\x12\nhelloworld\"\"\n\x0cHelloRequest\
    \x12\x12\n\x04name\x18\x01\x20\x01(\x04R\x04name\"&\n\nHelloReply\x12\
    \x18\n\x07message\x18\x01\x20\x01(\x04R\x07message2\x93\x01\n\x07Greeter\
    \x12>\n\x08SayHello\x12\x18.helloworld.HelloRequest\x1a\x16.helloworld.H\
    elloReply\"\0\x12H\n\x0eSayHelloStream\x12\x18.helloworld.HelloRequest\
    \x1a\x16.helloworld.HelloReply\"\0(\x010\x01B0\n\x1bio.grpc.examples.hel\
    loworldB\x0fHelloWorldProtoP\x01b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
