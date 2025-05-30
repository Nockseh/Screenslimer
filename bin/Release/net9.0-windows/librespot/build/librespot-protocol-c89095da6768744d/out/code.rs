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

//! Generated file from `spotify/login5/v3/challenges/code.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:spotify.login5.v3.challenges.CodeChallenge)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CodeChallenge {
    // message fields
    // @@protoc_insertion_point(field:spotify.login5.v3.challenges.CodeChallenge.method)
    pub method: ::protobuf::EnumOrUnknown<code_challenge::Method>,
    // @@protoc_insertion_point(field:spotify.login5.v3.challenges.CodeChallenge.code_length)
    pub code_length: i32,
    // @@protoc_insertion_point(field:spotify.login5.v3.challenges.CodeChallenge.expires_in)
    pub expires_in: i32,
    // @@protoc_insertion_point(field:spotify.login5.v3.challenges.CodeChallenge.canonical_phone_number)
    pub canonical_phone_number: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:spotify.login5.v3.challenges.CodeChallenge.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CodeChallenge {
    fn default() -> &'a CodeChallenge {
        <CodeChallenge as ::protobuf::Message>::default_instance()
    }
}

impl CodeChallenge {
    pub fn new() -> CodeChallenge {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "method",
            |m: &CodeChallenge| { &m.method },
            |m: &mut CodeChallenge| { &mut m.method },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "code_length",
            |m: &CodeChallenge| { &m.code_length },
            |m: &mut CodeChallenge| { &mut m.code_length },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "expires_in",
            |m: &CodeChallenge| { &m.expires_in },
            |m: &mut CodeChallenge| { &mut m.expires_in },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "canonical_phone_number",
            |m: &CodeChallenge| { &m.canonical_phone_number },
            |m: &mut CodeChallenge| { &mut m.canonical_phone_number },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CodeChallenge>(
            "CodeChallenge",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CodeChallenge {
    const NAME: &'static str = "CodeChallenge";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.method = is.read_enum_or_unknown()?;
                },
                16 => {
                    self.code_length = is.read_int32()?;
                },
                24 => {
                    self.expires_in = is.read_int32()?;
                },
                34 => {
                    self.canonical_phone_number = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.method != ::protobuf::EnumOrUnknown::new(code_challenge::Method::UNKNOWN) {
            my_size += ::protobuf::rt::int32_size(1, self.method.value());
        }
        if self.code_length != 0 {
            my_size += ::protobuf::rt::int32_size(2, self.code_length);
        }
        if self.expires_in != 0 {
            my_size += ::protobuf::rt::int32_size(3, self.expires_in);
        }
        if !self.canonical_phone_number.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.canonical_phone_number);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.method != ::protobuf::EnumOrUnknown::new(code_challenge::Method::UNKNOWN) {
            os.write_enum(1, ::protobuf::EnumOrUnknown::value(&self.method))?;
        }
        if self.code_length != 0 {
            os.write_int32(2, self.code_length)?;
        }
        if self.expires_in != 0 {
            os.write_int32(3, self.expires_in)?;
        }
        if !self.canonical_phone_number.is_empty() {
            os.write_string(4, &self.canonical_phone_number)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CodeChallenge {
        CodeChallenge::new()
    }

    fn clear(&mut self) {
        self.method = ::protobuf::EnumOrUnknown::new(code_challenge::Method::UNKNOWN);
        self.code_length = 0;
        self.expires_in = 0;
        self.canonical_phone_number.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CodeChallenge {
        static instance: CodeChallenge = CodeChallenge {
            method: ::protobuf::EnumOrUnknown::from_i32(0),
            code_length: 0,
            expires_in: 0,
            canonical_phone_number: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CodeChallenge {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CodeChallenge").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CodeChallenge {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CodeChallenge {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

/// Nested message and enums of message `CodeChallenge`
pub mod code_challenge {
    #[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
    // @@protoc_insertion_point(enum:spotify.login5.v3.challenges.CodeChallenge.Method)
    pub enum Method {
        // @@protoc_insertion_point(enum_value:spotify.login5.v3.challenges.CodeChallenge.Method.UNKNOWN)
        UNKNOWN = 0,
        // @@protoc_insertion_point(enum_value:spotify.login5.v3.challenges.CodeChallenge.Method.SMS)
        SMS = 1,
    }

    impl ::protobuf::Enum for Method {
        const NAME: &'static str = "Method";

        fn value(&self) -> i32 {
            *self as i32
        }

        fn from_i32(value: i32) -> ::std::option::Option<Method> {
            match value {
                0 => ::std::option::Option::Some(Method::UNKNOWN),
                1 => ::std::option::Option::Some(Method::SMS),
                _ => ::std::option::Option::None
            }
        }

        fn from_str(str: &str) -> ::std::option::Option<Method> {
            match str {
                "UNKNOWN" => ::std::option::Option::Some(Method::UNKNOWN),
                "SMS" => ::std::option::Option::Some(Method::SMS),
                _ => ::std::option::Option::None
            }
        }

        const VALUES: &'static [Method] = &[
            Method::UNKNOWN,
            Method::SMS,
        ];
    }

    impl ::protobuf::EnumFull for Method {
        fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
            static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
            descriptor.get(|| super::file_descriptor().enum_by_package_relative_name("CodeChallenge.Method").unwrap()).clone()
        }

        fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
            let index = *self as usize;
            Self::enum_descriptor().value_by_index(index)
        }
    }

    impl ::std::default::Default for Method {
        fn default() -> Self {
            Method::UNKNOWN
        }
    }

    impl Method {
        pub(in super) fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
            ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Method>("CodeChallenge.Method")
        }
    }
}

// @@protoc_insertion_point(message:spotify.login5.v3.challenges.CodeSolution)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct CodeSolution {
    // message fields
    // @@protoc_insertion_point(field:spotify.login5.v3.challenges.CodeSolution.code)
    pub code: ::std::string::String,
    // special fields
    // @@protoc_insertion_point(special_field:spotify.login5.v3.challenges.CodeSolution.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a CodeSolution {
    fn default() -> &'a CodeSolution {
        <CodeSolution as ::protobuf::Message>::default_instance()
    }
}

impl CodeSolution {
    pub fn new() -> CodeSolution {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "code",
            |m: &CodeSolution| { &m.code },
            |m: &mut CodeSolution| { &mut m.code },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<CodeSolution>(
            "CodeSolution",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for CodeSolution {
    const NAME: &'static str = "CodeSolution";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.code = is.read_string()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.code.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.code);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.code.is_empty() {
            os.write_string(1, &self.code)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> CodeSolution {
        CodeSolution::new()
    }

    fn clear(&mut self) {
        self.code.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static CodeSolution {
        static instance: CodeSolution = CodeSolution {
            code: ::std::string::String::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for CodeSolution {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("CodeSolution").unwrap()).clone()
    }
}

impl ::std::fmt::Display for CodeSolution {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CodeSolution {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n'spotify/login5/v3/challenges/code.proto\x12\x1cspotify.login5.v3.chal\
    lenges\"\xf1\x01\n\rCodeChallenge\x12J\n\x06method\x18\x01\x20\x01(\x0e2\
    2.spotify.login5.v3.challenges.CodeChallenge.MethodR\x06method\x12\x1f\n\
    \x0bcode_length\x18\x02\x20\x01(\x05R\ncodeLength\x12\x1d\n\nexpires_in\
    \x18\x03\x20\x01(\x05R\texpiresIn\x124\n\x16canonical_phone_number\x18\
    \x04\x20\x01(\tR\x14canonicalPhoneNumber\"\x1e\n\x06Method\x12\x0b\n\x07\
    UNKNOWN\x10\0\x12\x07\n\x03SMS\x10\x01\"\"\n\x0cCodeSolution\x12\x12\n\
    \x04code\x18\x01\x20\x01(\tR\x04codeB8\n&com.spotify.login5.v3.challenge\
    s.protoP\x01H\x02\xa2\x02\tSPTLogin5b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(2);
            messages.push(CodeChallenge::generated_message_descriptor_data());
            messages.push(CodeSolution::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(code_challenge::Method::generated_enum_descriptor_data());
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
