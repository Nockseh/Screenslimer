// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by pure
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `canvaz-meta.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:spotify.canvaz.Type)
pub enum Type {
    // @@protoc_insertion_point(enum_value:spotify.canvaz.Type.IMAGE)
    IMAGE = 0,
    // @@protoc_insertion_point(enum_value:spotify.canvaz.Type.VIDEO)
    VIDEO = 1,
    // @@protoc_insertion_point(enum_value:spotify.canvaz.Type.VIDEO_LOOPING)
    VIDEO_LOOPING = 2,
    // @@protoc_insertion_point(enum_value:spotify.canvaz.Type.VIDEO_LOOPING_RANDOM)
    VIDEO_LOOPING_RANDOM = 3,
    // @@protoc_insertion_point(enum_value:spotify.canvaz.Type.GIF)
    GIF = 4,
}

impl ::protobuf::Enum for Type {
    const NAME: &'static str = "Type";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Type> {
        match value {
            0 => ::std::option::Option::Some(Type::IMAGE),
            1 => ::std::option::Option::Some(Type::VIDEO),
            2 => ::std::option::Option::Some(Type::VIDEO_LOOPING),
            3 => ::std::option::Option::Some(Type::VIDEO_LOOPING_RANDOM),
            4 => ::std::option::Option::Some(Type::GIF),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<Type> {
        match str {
            "IMAGE" => ::std::option::Option::Some(Type::IMAGE),
            "VIDEO" => ::std::option::Option::Some(Type::VIDEO),
            "VIDEO_LOOPING" => ::std::option::Option::Some(Type::VIDEO_LOOPING),
            "VIDEO_LOOPING_RANDOM" => ::std::option::Option::Some(Type::VIDEO_LOOPING_RANDOM),
            "GIF" => ::std::option::Option::Some(Type::GIF),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [Type] = &[
        Type::IMAGE,
        Type::VIDEO,
        Type::VIDEO_LOOPING,
        Type::VIDEO_LOOPING_RANDOM,
        Type::GIF,
    ];
}

impl ::protobuf::EnumFull for Type {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("Type").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for Type {
    fn default() -> Self {
        Type::IMAGE
    }
}

impl Type {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Type>("Type")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x11canvaz-meta.proto\x12\x0espotify.canvaz*R\n\x04Type\x12\t\n\x05IMA\
    GE\x10\0\x12\t\n\x05VIDEO\x10\x01\x12\x11\n\rVIDEO_LOOPING\x10\x02\x12\
    \x18\n\x14VIDEO_LOOPING_RANDOM\x10\x03\x12\x07\n\x03GIF\x10\x04B\x1e\n\
    \x18com.spotify.canvaz.protoP\x01H\x02b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(0);
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(Type::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
