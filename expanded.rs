#![feature(prelude_import)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
use jetapi::{App, run_sync, Result, JetError};
use serde::{Deserialize, Serialize};
use axum::extract::{Path, State};
use axum::Json;
struct User {
    id: u32,
    name: String,
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for User {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private229::Result<Self, __D::Error>
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
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private229::Ok(__Field::__field0),
                        1u64 => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "id" => _serde::__private229::Ok(__Field::__field0),
                        "name" => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private229::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"id" => _serde::__private229::Ok(__Field::__field0),
                        b"name" => _serde::__private229::Ok(__Field::__field1),
                        _ => _serde::__private229::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private229::Result<Self, __D::Error>
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
                marker: _serde::__private229::PhantomData<User>,
                lifetime: _serde::__private229::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = User;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private229::Formatter,
                ) -> _serde::__private229::fmt::Result {
                    _serde::__private229::Formatter::write_str(
                        __formatter,
                        "struct User",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        u32,
                    >(&mut __seq)? {
                        _serde::__private229::Some(__value) => __value,
                        _serde::__private229::None => {
                            return _serde::__private229::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct User with 2 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        String,
                    >(&mut __seq)? {
                        _serde::__private229::Some(__value) => __value,
                        _serde::__private229::None => {
                            return _serde::__private229::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct User with 2 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private229::Ok(User {
                        id: __field0,
                        name: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private229::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private229::Option<u32> = _serde::__private229::None;
                    let mut __field1: _serde::__private229::Option<String> = _serde::__private229::None;
                    while let _serde::__private229::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private229::Option::is_some(&__field0) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                    );
                                }
                                __field0 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private229::Option::is_some(&__field1) {
                                    return _serde::__private229::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                    );
                                }
                                __field1 = _serde::__private229::Some(
                                    _serde::de::MapAccess::next_value::<String>(&mut __map)?,
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
                        _serde::__private229::Some(__field0) => __field0,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("id")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private229::Some(__field1) => __field1,
                        _serde::__private229::None => {
                            _serde::__private229::de::missing_field("name")?
                        }
                    };
                    _serde::__private229::Ok(User {
                        id: __field0,
                        name: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["id", "name"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "User",
                FIELDS,
                __Visitor {
                    marker: _serde::__private229::PhantomData::<User>,
                    lifetime: _serde::__private229::PhantomData,
                },
            )
        }
    }
};
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for User {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private229::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "User",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "id",
                &self.id,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "name",
                &self.name,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[automatically_derived]
impl ::core::clone::Clone for User {
    #[inline]
    fn clone(&self) -> User {
        User {
            id: ::core::clone::Clone::clone(&self.id),
            name: ::core::clone::Clone::clone(&self.name),
        }
    }
}
struct AppState {
    users: Vec<User>,
}
#[automatically_derived]
impl ::core::clone::Clone for AppState {
    #[inline]
    fn clone(&self) -> AppState {
        AppState {
            users: ::core::clone::Clone::clone(&self.users),
        }
    }
}
async fn get_user_impl(id: u32, state: AppState) -> Result<Json<User>> {
    let user = state
        .users
        .iter()
        .find(|u| u.id == id)
        .cloned()
        .ok_or_else(|| JetError::NotFound(
            ::alloc::__export::must_use({
                ::alloc::fmt::format(format_args!("User {0} not found", id))
            }),
        ))?;
    Ok(Json(user))
}
async fn create_user_impl(user: User, _state: AppState) -> Result<Json<User>> {
    Ok(Json(user))
}
async fn list_users_impl(state: AppState) -> Json<Vec<User>> {
    Json(state.users)
}
async fn health() -> &'static str {
    "OK"
}
fn main() -> Result<()> {
    let state = AppState {
        users: ::alloc::boxed::box_assume_init_into_vec_unsafe(
            ::alloc::intrinsics::write_box_via_move(
                ::alloc::boxed::Box::new_uninit(),
                [
                    User {
                        id: 1,
                        name: "Alice".into(),
                    },
                ],
            ),
        ),
    };
    let app = App::with_state_type::<AppState>()
        .get(
            "/users/:id",
            |Path(id): Path<u32>, State(state): State<AppState>| async move {
                get_user_impl(id, state).await
            },
        )
        .post(
            "/users",
            |Json(user): Json<User>, State(state): State<AppState>| async move {
                create_user_impl(user, state).await
            },
        )
        .get(
            "/users",
            |State(state): State<AppState>| async move { list_users_impl(state).await },
        )
        .get("/health", health)
        .with_state(state);
    run_sync(app, "0.0.0.0:3000")?;
    Ok(())
}
