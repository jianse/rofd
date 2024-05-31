mod common {
    use serde::{de::Visitor, Deserialize, Serialize};
    use super::base::StRefId;
    /// Layer type
    pub struct CtLayer {
        #[serde(rename = "@Type")]
        r#type: Option<String>,
        #[serde(rename = "@DrawParam")]
        draw_param: Option<StRefId>,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CtLayer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field2_finish(
                f,
                "CtLayer",
                "type",
                &self.r#type,
                "draw_param",
                &&self.draw_param,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CtLayer {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "CtLayer",
                    false as usize + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "@Type",
                    &self.r#type,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "@DrawParam",
                    &self.draw_param,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CtLayer {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "@Type" => _serde::__private::Ok(__Field::__field0),
                            "@DrawParam" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"@Type" => _serde::__private::Ok(__Field::__field0),
                            b"@DrawParam" => _serde::__private::Ok(__Field::__field1),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CtLayer>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CtLayer;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CtLayer",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct CtLayer with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<StRefId>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct CtLayer with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CtLayer {
                            r#type: __field0,
                            draw_param: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<StRefId>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("@Type"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "@DrawParam",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<StRefId>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("@Type")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("@DrawParam")?
                            }
                        };
                        _serde::__private::Ok(CtLayer {
                            r#type: __field0,
                            draw_param: __field1,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["@Type", "@DrawParam"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CtLayer",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CtLayer>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    /// deserialize Layer
    struct TestStruct {
        #[serde(flatten)]
        base: CtLayer,
        #[serde(rename = "@ID")]
        id: u64,
        #[serde(rename = "$value")]
        val: String,
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for TestStruct {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field3_finish(
                f,
                "TestStruct",
                "base",
                &self.base,
                "id",
                &self.id,
                "val",
                &&self.val,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for TestStruct {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field<'de> {
                    __field1,
                    __field2,
                    __other(_serde::__private::de::Content<'de>),
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field<'de>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_bool<__E>(
                        self,
                        __value: bool,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Bool(__value),
                            ),
                        )
                    }
                    fn visit_i8<__E>(
                        self,
                        __value: i8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::I8(__value)),
                        )
                    }
                    fn visit_i16<__E>(
                        self,
                        __value: i16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I16(__value),
                            ),
                        )
                    }
                    fn visit_i32<__E>(
                        self,
                        __value: i32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I32(__value),
                            ),
                        )
                    }
                    fn visit_i64<__E>(
                        self,
                        __value: i64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::I64(__value),
                            ),
                        )
                    }
                    fn visit_u8<__E>(
                        self,
                        __value: u8,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::U8(__value)),
                        )
                    }
                    fn visit_u16<__E>(
                        self,
                        __value: u16,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U16(__value),
                            ),
                        )
                    }
                    fn visit_u32<__E>(
                        self,
                        __value: u32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U32(__value),
                            ),
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::U64(__value),
                            ),
                        )
                    }
                    fn visit_f32<__E>(
                        self,
                        __value: f32,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F32(__value),
                            ),
                        )
                    }
                    fn visit_f64<__E>(
                        self,
                        __value: f64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::F64(__value),
                            ),
                        )
                    }
                    fn visit_char<__E>(
                        self,
                        __value: char,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(
                                _serde::__private::de::Content::Char(__value),
                            ),
                        )
                    }
                    fn visit_unit<__E>(
                        self,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        _serde::__private::Ok(
                            __Field::__other(_serde::__private::de::Content::Unit),
                        )
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "@ID" => _serde::__private::Ok(__Field::__field1),
                            "$value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::String(
                                    _serde::__private::ToString::to_string(__value),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"@ID" => _serde::__private::Ok(__Field::__field1),
                            b"$value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::ByteBuf(
                                    __value.to_vec(),
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_str<__E>(
                        self,
                        __value: &'de str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "@ID" => _serde::__private::Ok(__Field::__field1),
                            "$value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Str(__value);
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                    fn visit_borrowed_bytes<__E>(
                        self,
                        __value: &'de [u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"@ID" => _serde::__private::Ok(__Field::__field1),
                            b"$value" => _serde::__private::Ok(__Field::__field2),
                            _ => {
                                let __value = _serde::__private::de::Content::Bytes(
                                    __value,
                                );
                                _serde::__private::Ok(__Field::__other(__value))
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field<'de> {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<TestStruct>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = TestStruct;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct TestStruct",
                        )
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field1: _serde::__private::Option<u64> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __collect = _serde::__private::Vec::<
                            _serde::__private::Option<
                                (
                                    _serde::__private::de::Content,
                                    _serde::__private::de::Content,
                                ),
                            >,
                        >::new();
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("@ID"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u64>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("$value"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__other(__name) => {
                                    __collect
                                        .push(
                                            _serde::__private::Some((
                                                __name,
                                                _serde::de::MapAccess::next_value(&mut __map)?,
                                            )),
                                        );
                                }
                            }
                        }
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("@ID")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("$value")?
                            }
                        };
                        let __field0: CtLayer = _serde::de::Deserialize::deserialize(
                            _serde::__private::de::FlatMapDeserializer(
                                &mut __collect,
                                _serde::__private::PhantomData,
                            ),
                        )?;
                        _serde::__private::Ok(TestStruct {
                            base: __field0,
                            id: __field1,
                            val: __field2,
                        })
                    }
                }
                _serde::Deserializer::deserialize_map(
                    __deserializer,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<TestStruct>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
}
