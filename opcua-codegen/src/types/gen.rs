use std::collections::{HashMap, HashSet};

use convert_case::{Case, Casing};
use proc_macro2::Span;
use syn::{
    parse_quote, punctuated::Punctuated, FieldsNamed, File, Generics, Ident, Item, ItemEnum,
    ItemImpl, ItemMacro, ItemStruct, Lit, LitByte, Path, Token, Type, Visibility,
};

use crate::{error::CodeGenError, GeneratedOutput, StructuredType};

use super::{loader::LoadedType, EnumType, ExternalType};
use quote::quote;

pub enum ItemDefinition {
    Struct(ItemStruct),
    Enum(ItemEnum),
    BitField(ItemMacro),
}

pub struct GeneratedItem {
    pub item: ItemDefinition,
    pub impls: Vec<ItemImpl>,
    pub module: String,
    pub name: String,
}

impl GeneratedOutput for GeneratedItem {
    fn to_file(self) -> File {
        let mut items = Vec::new();
        match self.item {
            ItemDefinition::Struct(v) => items.push(Item::Struct(v)),
            ItemDefinition::Enum(v) => items.push(Item::Enum(v)),
            ItemDefinition::BitField(v) => items.push(Item::Macro(v)),
        }
        for imp in self.impls {
            items.push(Item::Impl(imp));
        }

        File {
            shebang: None,
            attrs: Vec::new(),
            items,
        }
    }

    fn module(&self) -> &str {
        &self.module
    }

    fn name(&self) -> &str {
        &self.name
    }
}

pub struct CodeGenItemConfig {
    pub enums_single_file: bool,
    pub structs_single_file: bool,
}

pub struct CodeGenerator {
    import_map: HashMap<String, ExternalType>,
    input: HashMap<String, LoadedType>,
    default_excluded: HashSet<String>,
    config: CodeGenItemConfig,
}

impl CodeGenerator {
    pub fn new(
        external_import_map: HashMap<String, ExternalType>,
        input: Vec<LoadedType>,
        default_excluded: HashSet<String>,
        config: CodeGenItemConfig,
    ) -> Self {
        Self {
            import_map: external_import_map,
            input: input
                .into_iter()
                .map(|v| (v.name().to_owned(), v))
                .collect(),
            config,
            default_excluded,
        }
    }

    fn is_default_recursive(&self, name: &str) -> bool {
        if self.default_excluded.contains(name) {
            return false;
        }

        let Some(it) = self.import_map.get(name) else {
            // Not in the import map means it's a builtin, we assume these have defaults for now.
            return true;
        };

        if let Some(def) = it.has_default {
            return def;
        }

        let Some(input) = self.input.get(name) else {
            return false;
        };

        match input {
            LoadedType::Struct(s) => {
                for k in &s.fields {
                    let has_default = match &k.typ {
                        crate::StructureFieldType::Field(f) => self.is_default_recursive(f),
                        crate::StructureFieldType::Array(_) => true,
                    };
                    if !has_default {
                        return false;
                    }
                }
                true
            }
            LoadedType::Enum(e) => e.option || e.default_value.is_some(),
        }
    }

    pub fn generate_types(mut self) -> Result<Vec<GeneratedItem>, CodeGenError> {
        let mut generated = Vec::new();

        for item in self.input.values() {
            let name = match item {
                LoadedType::Struct(s) => {
                    if self.config.structs_single_file {
                        "structs".to_owned()
                    } else {
                        s.name.to_case(Case::Snake)
                    }
                }
                LoadedType::Enum(s) => {
                    if self.config.enums_single_file {
                        "enums".to_owned()
                    } else {
                        s.name.to_case(Case::Snake)
                    }
                }
            };

            self.import_map.insert(
                item.name().to_owned(),
                ExternalType {
                    path: format!("super::{}", name),
                    // Determined later
                    has_default: None,
                    base_type: match &item {
                        LoadedType::Struct(v) => v.base_type.clone(),
                        LoadedType::Enum(_) => None,
                    },
                },
            );
        }
        for key in self.import_map.keys().cloned().collect::<Vec<_>>() {
            let has_default = self.is_default_recursive(&key);
            if let Some(it) = self.import_map.get_mut(&key) {
                it.has_default = Some(has_default);
            }
        }

        let input = std::mem::take(&mut self.input);

        for item in input.into_values() {
            match item {
                LoadedType::Struct(v) => generated.push(self.generate_struct(v)?),
                LoadedType::Enum(v) => generated.push(self.generate_enum(v)?),
            }
        }

        Ok(generated)
    }

    fn get_type_path(&self, name: &str) -> String {
        // Type is known, use the external path.
        if let Some(ext) = self.import_map.get(name) {
            return format!("{}::{}", ext.path, name);
        }
        // Assume the type is a builtin.
        name.to_string()
    }

    fn has_default(&self, name: &str) -> bool {
        self.import_map
            .get(name)
            .is_some_and(|v| v.has_default.is_some_and(|v| v))
    }

    fn generate_bitfield(&self, item: EnumType) -> Result<GeneratedItem, CodeGenError> {
        let mut body = quote! {};
        let ty: Type = syn::parse_str(&item.typ.to_string())?;
        if let Some(doc) = item.documentation {
            body.extend(quote! {
                #[doc = #doc]
            });
        }
        let mut variants = quote! {};
        for field in &item.values {
            let name = Ident::new(&field.name, Span::call_site());
            let value = field.value;
            let value_token = match item.typ {
                super::enum_type::EnumReprType::u8 => {
                    let value: u8 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to u8, {} is out of range",
                            value
                        ))
                    })?;
                    Lit::Byte(LitByte::new(value, Span::call_site()))
                }
                super::enum_type::EnumReprType::i16 => {
                    let value: i16 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to i16, {} is out of range",
                            value
                        ))
                    })?;
                    parse_quote! { #value }
                }
                super::enum_type::EnumReprType::i32 => {
                    let value: i32 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to i32, {} is out of range",
                            value
                        ))
                    })?;
                    parse_quote! { #value }
                }
                super::enum_type::EnumReprType::i64 => {
                    parse_quote! { #value }
                }
            };
            variants.extend(quote! {
                const #name = #value_token;
            });
        }
        let enum_ident = Ident::new(&item.name, Span::call_site());

        body.extend(quote! {
            bitflags::bitflags! {
                #[derive(Debug, Copy, Clone, PartialEq)]
                pub struct #enum_ident: #ty {
                    #variants
                }
            }
        });

        let mut impls = Vec::new();
        let size: usize = item.size.try_into().map_err(|_| {
            CodeGenError::Other(format!("Value {} does not fit in a usize", item.size))
        })?;
        let write_method = Ident::new(&format!("write_{}", item.typ), Span::call_site());

        impls.push(parse_quote! {
            impl opcua::types::BinaryEncoder for #enum_ident {
                fn byte_len(&self) -> usize {
                    #size
                }

                fn encode<S: std::io::Write>(&self, stream: &mut S) -> opcua::types::EncodingResult<usize> {
                    opcua::types::#write_method(stream, self.bits())
                }

                fn decode<S: std::io::Read>(stream: &mut S, decoding_options: &opcua::types::DecodingOptions) -> opcua::types::EncodingResult<Self> {
                    Ok(Self::from_bits_truncate(#ty::decode(stream, decoding_options)?))
                }
            }
        });

        impls.push(parse_quote! {
            impl Default for #enum_ident {
                fn default() -> Self {
                    Self::empty()
                }
            }
        });

        impls.push(parse_quote! {
            impl From<#enum_ident> for opcua::types::Variant {
                fn from(v: #enum_ident) -> Self {
                    Self::from(v.bits())
                }
            }
        });

        impls.push(parse_quote! {
            impl opcua::types::AsVariantRef for #enum_ident {
                fn as_variant(&self) -> opcua::types::Variant {
                    self.bits().into()
                }
            }
        });

        let ser_method = Ident::new(&format!("serialize_{}", item.typ), Span::call_site());
        let deser_method = Ident::new(&format!("deserialize_{}", item.typ), Span::call_site());
        let typ_str = format!("an {}", item.typ);

        impls.push(parse_quote! {
            #[cfg(feature = "json")]
            impl<'de> serde::de::Deserialize<'de> for #enum_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    struct BitFieldVisitor;

                    impl<'de> serde::de::Visitor<'de> for BitFieldVisitor {
                        type Value = #ty;

                        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                            write!(formatter, #typ_str)
                        }
                    }

                    deserializer
                        .#deser_method(BitFieldVisitor)
                        .map(#enum_ident::from_bits_truncate)
                }
            }
        });

        impls.push(parse_quote! {
            #[cfg(feature = "json")]
            impl serde::ser::Serialize for #enum_ident {
                fn serialize<S>(&self, serializer: S) -> Result<
                    <S as serde::ser::Serializer>::Ok, <S as serde::ser::Serializer>::Error>
                where
                    S: serde::ser::Serializer
                {
                    serializer.#ser_method(self.bits())
                }
            }
        });

        Ok(GeneratedItem {
            item: ItemDefinition::BitField(parse_quote! {
                #body
            }),
            impls,
            module: if self.config.enums_single_file {
                "enums".to_owned()
            } else {
                item.name.to_case(Case::Snake)
            },
            name: item.name.clone(),
        })
    }

    fn generate_enum(&self, item: EnumType) -> Result<GeneratedItem, CodeGenError> {
        if item.option {
            return self.generate_bitfield(item);
        }

        let mut attrs = Vec::new();
        let mut variants = Punctuated::new();

        if let Some(doc) = item.documentation {
            attrs.push(parse_quote! {
                #[doc = #doc]
            });
        }
        attrs.push(parse_quote! {
            #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        });
        let ty: Type = syn::parse_str(&item.typ.to_string())?;
        attrs.push(parse_quote! {
            #[repr(#ty)]
        });

        let mut try_from_arms = quote! {};

        for field in &item.values {
            let name = Ident::new(&field.name, Span::call_site());
            let value = field.value;
            let value_token = match item.typ {
                super::enum_type::EnumReprType::u8 => {
                    let value: u8 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to u8, {} is out of range",
                            value
                        ))
                    })?;
                    Lit::Byte(LitByte::new(value, Span::call_site()))
                }
                super::enum_type::EnumReprType::i16 => {
                    let value: i16 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to i16, {} is out of range",
                            value
                        ))
                    })?;
                    parse_quote! { #value }
                }
                super::enum_type::EnumReprType::i32 => {
                    let value: i32 = value.try_into().map_err(|_| {
                        CodeGenError::Other(format!(
                            "Unexpected error converting to i32, {} is out of range",
                            value
                        ))
                    })?;
                    parse_quote! { #value }
                }
                super::enum_type::EnumReprType::i64 => {
                    parse_quote! { #value }
                }
            };

            try_from_arms = quote! {
                #try_from_arms
                #value_token => Self::#name,
            };

            variants.push(parse_quote! {
                #name = #value_token
            })
        }

        if item.values.iter().any(|f| f.name == "Invalid") {
            let invalid_msg = format!(
                "Got unexpected value for enum {}: {{}}. Falling back on Invalid",
                item.name
            );
            try_from_arms = quote! {
                #try_from_arms
                r => {
                    log::warn!(#invalid_msg, r);
                    Self::Invalid
                },
            };
        } else {
            let invalid_msg = format!("Got unexpected value for enum {}: {{}}", item.name);
            try_from_arms = quote! {
                #try_from_arms
                r => {
                    log::error!(#invalid_msg, r);
                    return Err(opcua::types::StatusCode::BadUnexpectedError)
                }
            };
        }

        let mut impls = Vec::new();
        let enum_ident = Ident::new(&item.name, Span::call_site());

        // TryFrom impl
        impls.push(parse_quote! {
            impl TryFrom<#ty> for #enum_ident {
                type Error = opcua::types::StatusCode;

                fn try_from(value: #ty) -> Result<Self, <Self as TryFrom<#ty>>::Error> {
                    Ok(match value {
                        #try_from_arms
                    })
                }
            }
        });

        impls.push(parse_quote! {
            impl From<#enum_ident> for #ty {
                fn from(value: #enum_ident) -> Self {
                    value as #ty
                }
            }
        });
        impls.push(parse_quote! {
            impl From<#enum_ident> for opcua::types::Variant {
                fn from(value: #enum_ident) -> Self {
                    Self::from(value as #ty)
                }
            }
        });

        impls.push(parse_quote! {
            impl opcua::types::AsVariantRef for #enum_ident {
                fn as_variant(&self) -> opcua::types::Variant {
                    (*self as #ty).into()
                }
            }
        });

        let ser_method = Ident::new(&format!("serialize_{}", item.typ), Span::call_site());
        let deser_method = Ident::new(&format!("deserialize_{}", item.typ), Span::call_site());
        let typ_str = format!("{}", item.typ);
        let typ_name_str = item.typ.to_string();

        impls.push(parse_quote! {
            #[cfg(feature = "json")]
            impl<'de> serde::de::Deserialize<'de> for #enum_ident {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::de::Deserializer<'de>,
                {
                    struct EnumVisitor;
                    use serde::de::Error;

                    impl<'de> serde::de::Visitor<'de> for EnumVisitor {
                        type Value = #ty;

                        fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                            write!(formatter, #typ_str)
                        }
                    }

                    let value = deserializer.#deser_method(EnumVisitor)?;
                    Self::try_from(value).map_err(|e| D::Error::custom(
                        &format!("Failed to deserialize {}: {:?}", #typ_name_str, e)
                    ))
                }
            }
        });

        impls.push(parse_quote! {
            #[cfg(feature = "json")]
            impl serde::ser::Serialize for #enum_ident {
                fn serialize<S>(&self, serializer: S) -> Result<
                    <S as serde::ser::Serializer>::Ok, <S as serde::ser::Serializer>::Error>
                where
                    S: serde::ser::Serializer
                {
                    serializer.#ser_method(*self as #ty)
                }
            }
        });

        // BinaryEncoder impl
        let size: usize = item.size.try_into().map_err(|_| {
            CodeGenError::Other(format!("Value {} does not fit in a usize", item.size))
        })?;
        let write_method = Ident::new(&format!("write_{}", item.typ), Span::call_site());
        let read_method = Ident::new(&format!("read_{}", item.typ), Span::call_site());

        impls.push(parse_quote! {
            impl opcua::types::BinaryEncoder for #enum_ident {
                fn byte_len(&self) -> usize {
                    #size
                }

                fn encode<S: std::io::Write>(&self, stream: &mut S) -> opcua::types::EncodingResult<usize> {
                    opcua::types::#write_method(stream, *self as #ty)
                }

                fn decode<S: std::io::Read>(stream: &mut S, _: &opcua::types::DecodingOptions) -> opcua::types::EncodingResult<Self> {
                    let value = opcua::types::#read_method(stream)?;
                    Ok(Self::try_from(value)?)
                }
            }
        });

        let res = ItemEnum {
            attrs,
            vis: Visibility::Public(Token![pub](Span::call_site())),
            enum_token: Token![enum](Span::call_site()),
            ident: enum_ident,
            generics: Generics::default(),
            brace_token: syn::token::Brace(Span::call_site()),
            variants,
        };

        Ok(GeneratedItem {
            item: ItemDefinition::Enum(res),
            impls,
            module: if self.config.enums_single_file {
                "enums".to_owned()
            } else {
                item.name.to_case(Case::Snake)
            },
            name: item.name.clone(),
        })
    }

    fn is_extension_object(&self, typ: &str) -> bool {
        if typ == "ua:ExtensionObject" {
            return true;
        }

        let name = match typ.split_once(":") {
            Some((_, n)) => n,
            None => typ,
        };

        let Some(parent) = self.import_map.get(name) else {
            return false;
        };
        if let Some(p) = &parent.base_type {
            self.is_extension_object(p)
        } else {
            false
        }
    }

    fn generate_struct(&self, item: StructuredType) -> Result<GeneratedItem, CodeGenError> {
        let mut attrs = Vec::new();
        let mut fields = Punctuated::new();

        if let Some(doc) = &item.documentation {
            attrs.push(parse_quote! {
                #[doc = #doc]
            });
        }
        attrs.push(parse_quote! {
            #[derive(Debug, Clone, PartialEq)]
        });
        attrs.push(parse_quote! {
            #[cfg_attr(feature = "json", serde_with::skip_serializing_none)]
        });
        attrs.push(parse_quote! {
            #[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
        });
        attrs.push(parse_quote! {
            #[cfg_attr(feature = "json", serde(rename_all = "PascalCase"))]
        });

        if self.has_default(&item.name) {
            attrs.push(parse_quote! {
                #[derive(Default)]
            });
        }

        let mut impls = Vec::new();
        let struct_ident = Ident::new(&item.name, Span::call_site());

        for field in item.visible_fields() {
            let typ: Type = match &field.typ {
                crate::StructureFieldType::Field(f) => syn::parse_str(&self.get_type_path(f))?,
                crate::StructureFieldType::Array(f) => {
                    let path: Path = syn::parse_str(&self.get_type_path(f))?;
                    parse_quote! { Option<Vec<#path>> }
                }
            };
            let ident = Ident::new(&field.name, Span::call_site());
            fields.push(parse_quote! {
                pub #ident: #typ
            });
        }

        // Generate impls
        // Has message info
        // TODO: This won't work for custom types. It may be possible
        // to change `MessageInfo` to return a NodeId, then figure out the
        // correct value of that during codegen.
        if item
            .base_type
            .as_ref()
            .is_some_and(|v| self.is_extension_object(v))
        {
            let encoding_ident = Ident::new(
                &format!("{}_Encoding_DefaultBinary", item.name),
                Span::call_site(),
            );
            impls.push(parse_quote! {
                impl opcua::types::MessageInfo for #struct_ident {
                    fn object_id(&self) -> opcua::types::ObjectId {
                        opcua::types::ObjectId::#encoding_ident
                    }
                }
            });
        }

        let mut len_impl;
        let mut encode_impl;
        let mut decode_impl = quote! {};
        let mut decode_build = quote! {};

        let mut has_context = false;
        // Special case an empty struct
        if item.fields.is_empty() {
            len_impl = quote! { 0usize };
            encode_impl = quote! { Ok(0) };
            decode_build = quote! { Ok(Self {}) };
        } else {
            len_impl = quote! {
                let mut size = 0usize;
            };
            encode_impl = quote! {
                let mut size = 0usize;
            };
            for field in item.visible_fields() {
                let ident = Ident::new(&field.name, Span::call_site());

                let ty: Type = match &field.typ {
                    crate::StructureFieldType::Field(f) => syn::parse_str(&self.get_type_path(f))?,
                    crate::StructureFieldType::Array(f) => {
                        let path: Path = syn::parse_str(&self.get_type_path(f))?;
                        parse_quote! {
                            Option<Vec<#path>>
                        }
                    }
                };

                len_impl.extend(quote! {
                    size += self.#ident.byte_len();
                });
                encode_impl.extend(quote! {
                    size += self.#ident.encode(stream)?;
                });
                if has_context {
                    decode_impl.extend(quote! {
                        let #ident = <#ty as opcua::types::BinaryEncoder>::decode(stream, decoding_options)
                            .map_err(|e| e.with_request_handle(__request_handle))?;
                    })
                } else {
                    decode_impl.extend(quote! {
                        let #ident = <#ty as opcua::types::BinaryEncoder>::decode(stream, decoding_options)?;
                    });
                }

                decode_build.extend(quote! {
                    #ident,
                });

                if field.name == "request_header" {
                    decode_impl.extend(quote! {
                        let __request_handle = request_header.request_handle;
                    });
                    has_context = true;
                } else if field.name == "response_header" {
                    decode_impl.extend(quote! {
                        let __request_handle = response_header.request_handle;
                    });
                    has_context = true;
                }
            }
            len_impl.extend(quote! {
                size
            });
            encode_impl.extend(quote! {
                Ok(size)
            });
            decode_build = quote! {
                Ok(Self {
                    #decode_build
                })
            };
        }

        impls.push(parse_quote! {
            impl opcua::types::BinaryEncoder for #struct_ident {
                fn byte_len(&self) -> usize {
                    #len_impl
                }

                #[allow(unused_variables)]
                fn encode<S: std::io::Write>(&self, stream: &mut S) -> opcua::types::EncodingResult<usize> {
                    #encode_impl
                }

                #[allow(unused_variables)]
                fn decode<S: std::io::Read>(stream: &mut S, decoding_options: &opcua::types::DecodingOptions) -> opcua::types::EncodingResult<Self> {
                    #decode_impl
                    #decode_build
                }
            }
        });

        let res = ItemStruct {
            attrs,
            vis: Visibility::Public(Token![pub](Span::call_site())),
            struct_token: Token![struct](Span::call_site()),
            ident: struct_ident,
            generics: Generics::default(),
            fields: syn::Fields::Named(FieldsNamed {
                brace_token: syn::token::Brace(Span::call_site()),
                named: fields,
            }),
            semi_token: None,
        };

        Ok(GeneratedItem {
            item: ItemDefinition::Struct(res),
            impls,
            module: if self.config.structs_single_file {
                "structs".to_owned()
            } else {
                item.name.to_case(Case::Snake)
            },
            name: item.name.clone(),
        })
    }
}
