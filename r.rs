#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
mod app {
    use chalk_rs::Chalk;
    use lazy_static::lazy_static;
    mod build {
        use ahqstore_types::{
            AHQStoreApplication, DownloadUrl, InstallerFormat, InstallerOptions,
            InstallerOptionsAndroid, InstallerOptionsLinux, InstallerOptionsWindows, Str,
        };
        use lazy_static::lazy_static;
        use reqwest::blocking::{Client, ClientBuilder};
        use serde::{Deserialize, Serialize};
        use serde_json::{from_str, to_string};
        use std::{collections::HashMap, env, fs, process};
        use crate::app::ERR;
        use super::INFO;
        mod config {
            use std::{env, fs, process};
            use serde_json::from_str;
            use crate::app::{
                shared::{Config, Finder, IMetadata},
                ERR,
            };
            use super::{GHAsset, GHRelease};
            pub fn get_config<'a>() -> IMetadata<'a> {
                let Ok(config) = fs::read_to_string("./.ahqstore/config.json") else {
                    ERR.println(&"Unable to read config file!");
                    process::exit(1);
                };
                let config = config.leak();
                let Ok(mut config) = from_str::<'a, Config>(config) else {
                    ERR.println(&"Unable to read config file!");
                    process::exit(1);
                };
                if let Ok(app_id) = env::var("APP_ID") {
                    config.remove(&app_id).expect("Key not present in JSON")
                } else {
                    config.into_values().nth(0).expect("No Key present in JSON")
                }
            }
            pub fn find_assets<'a>(
                gh_r: &'a GHRelease,
                finder: &'a Finder,
            ) -> Vec<&'a GHAsset> {
                gh_r.assets
                    .iter()
                    .filter(|a| {
                        if let Some(x) = finder.startsWith {
                            if !a.name.starts_with(&x) {
                                return false;
                            }
                        }
                        if let Some(x) = finder.contains {
                            if !a.name.contains(&x) {
                                return false;
                            }
                        }
                        if let Some(x) = finder.endsWith {
                            if !a.name.ends_with(&x) {
                                return false;
                            }
                        }
                        true
                    })
                    .collect::<Vec<_>>()
            }
        }
        mod icon {
            use crate::app::ERR;
            use image::{load_from_memory_with_format as load_img, ImageFormat};
            use std::fs;
            use std::process;
            pub fn get_icon(uid: &str) -> Vec<u8> {
                let base_img = ::alloc::__export::must_use({
                    ::alloc::fmt::format(
                        format_args!("./.ahqstore/images/{0}/icon.png", uid),
                    )
                });
                let Ok(icon) = fs::read(&base_img) else {
                    ERR.println(&"Unable to read icon file!");
                    process::exit(1);
                };
                validate_png(&icon);
                icon
            }
            pub fn get_images(uid: &str) -> Vec<Vec<u8>> {
                let base_img = ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("./.ahqstore/images/{0}", uid))
                });
                let Ok(image_dir) = fs::read_dir(&base_img) else {
                    ERR.println(&"Unable to read image dir!");
                    process::exit(1);
                };
                let mut entries = image_dir
                    .map(|res| res.expect("Unable to unwrap dir entry").path())
                    .filter(|f| !f.ends_with("icon.png"))
                    .map(|res| fs::read(res).expect("Unable to read bytes"))
                    .map(|img| {
                        validate_png(&img);
                        return img;
                    })
                    .collect::<Vec<_>>();
                entries.truncate(10);
                entries
            }
            pub fn validate_png(data: &Vec<u8>) {
                let Ok(_) = load_img(&data, ImageFormat::Png) else {
                    ERR.println(&"Invalid PNG");
                    process::exit(1);
                };
            }
        }
        mod release {
            use std::process;
            use serde_json::{from_str, to_string};
            use sha2::{Digest, Sha256};
            use crate::app::{
                build::{GHRelease, Str},
                ERR, WARN,
            };
            use super::CLIENT;
            pub fn fetch_release(
                repo: &str,
                r_id: &str,
                gh_token: &str,
            ) -> (Str, GHRelease) {
                let Ok(resp) = ({
                    let mut resp = CLIENT
                        .get(
                            ::alloc::__export::must_use({
                                ::alloc::fmt::format(
                                    format_args!(
                                        "https://api.github.com/repos/{0}/releases/{1}",
                                        repo,
                                        r_id,
                                    ),
                                )
                            }),
                        );
                    if gh_token != "" {
                        resp = resp.bearer_auth(gh_token);
                    } else {
                        WARN.println(
                            &"You may set GH_TOKEN environment variable to load private repos",
                        );
                    }
                    resp.send()
                }) else {
                    ERR.println(&"Unable to fetch release");
                    process::exit(1)
                };
                let Ok(release) = resp.text() else {
                    ERR.println(&"Unable to read release");
                    process::exit(1);
                };
                let Ok(resp) = from_str::<GHRelease>(&release) else {
                    ERR.println(&"Unable to parse release");
                    process::exit(1);
                };
                let mut hasher = Sha256::new();
                hasher.update(release.as_bytes());
                let hashed = hasher.finalize();
                let hashed = hashed.to_vec();
                let version = to_string(&hashed).unwrap_or("**UNKNOWN**".to_string());
                (version, resp)
            }
        }
        use config::*;
        use icon::*;
        use release::*;
        #[macro_use]
        mod macros {}
        #[allow(missing_copy_implementations)]
        #[allow(non_camel_case_types)]
        #[allow(dead_code)]
        pub struct CLIENT {
            __private_field: (),
        }
        #[doc(hidden)]
        #[allow(non_upper_case_globals)]
        pub static CLIENT: CLIENT = CLIENT { __private_field: () };
        impl ::lazy_static::__Deref for CLIENT {
            type Target = Client;
            fn deref(&self) -> &Client {
                #[inline(always)]
                fn __static_ref_initialize() -> Client {
                    ClientBuilder::new()
                        .user_agent("AHQ Store / App Builder")
                        .build()
                        .unwrap()
                }
                #[inline(always)]
                fn __stability() -> &'static Client {
                    static LAZY: ::lazy_static::lazy::Lazy<Client> = ::lazy_static::lazy::Lazy::INIT;
                    LAZY.get(__static_ref_initialize)
                }
                __stability()
            }
        }
        impl ::lazy_static::LazyStatic for CLIENT {
            fn initialize(lazy: &Self) {
                let _ = &**lazy;
            }
        }
        struct GHRelease {
            pub tag_name: String,
            pub upload_url: String,
            pub assets: Vec<GHAsset>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GHRelease {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "GHRelease",
                    "tag_name",
                    &self.tag_name,
                    "upload_url",
                    &self.upload_url,
                    "assets",
                    &&self.assets,
                )
            }
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
            impl<'de> _serde::Deserialize<'de> for GHRelease {
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
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
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
                                2u64 => _serde::__private::Ok(__Field::__field2),
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
                                "tag_name" => _serde::__private::Ok(__Field::__field0),
                                "upload_url" => _serde::__private::Ok(__Field::__field1),
                                "assets" => _serde::__private::Ok(__Field::__field2),
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
                                b"tag_name" => _serde::__private::Ok(__Field::__field0),
                                b"upload_url" => _serde::__private::Ok(__Field::__field1),
                                b"assets" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
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
                        marker: _serde::__private::PhantomData<GHRelease>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = GHRelease;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct GHRelease",
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
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GHRelease with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GHRelease with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Vec<GHAsset>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct GHRelease with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(GHRelease {
                                tag_name: __field0,
                                upload_url: __field1,
                                assets: __field2,
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
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Vec<GHAsset>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "tag_name",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "upload_url",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("assets"),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Vec<GHAsset>,
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
                                    _serde::__private::de::missing_field("tag_name")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("upload_url")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("assets")?
                                }
                            };
                            _serde::__private::Ok(GHRelease {
                                tag_name: __field0,
                                upload_url: __field1,
                                assets: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "tag_name",
                        "upload_url",
                        "assets",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "GHRelease",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<GHRelease>,
                            lifetime: _serde::__private::PhantomData,
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
            impl _serde::Serialize for GHRelease {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "GHRelease",
                        false as usize + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "tag_name",
                        &self.tag_name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "upload_url",
                        &self.upload_url,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "assets",
                        &self.assets,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        struct GHAsset {
            pub name: String,
            pub browser_download_url: String,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for GHAsset {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "GHAsset",
                    "name",
                    &self.name,
                    "browser_download_url",
                    &&self.browser_download_url,
                )
            }
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
            impl<'de> _serde::Deserialize<'de> for GHAsset {
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
                    #[automatically_derived]
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
                                "name" => _serde::__private::Ok(__Field::__field0),
                                "browser_download_url" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
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
                                b"name" => _serde::__private::Ok(__Field::__field0),
                                b"browser_download_url" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
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
                        marker: _serde::__private::PhantomData<GHAsset>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = GHAsset;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct GHAsset",
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
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GHAsset with 2 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GHAsset with 2 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(GHAsset {
                                name: __field0,
                                browser_download_url: __field1,
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
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "browser_download_url",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
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
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("name")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field(
                                        "browser_download_url",
                                    )?
                                }
                            };
                            _serde::__private::Ok(GHAsset {
                                name: __field0,
                                browser_download_url: __field1,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "name",
                        "browser_download_url",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "GHAsset",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<GHAsset>,
                            lifetime: _serde::__private::PhantomData,
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
            impl _serde::Serialize for GHAsset {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "GHAsset",
                        false as usize + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "name",
                        &self.name,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "browser_download_url",
                        &self.browser_download_url,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
                }
            }
        };
        pub fn build_config(upload: bool, gh_action: bool) {
            let Some(_) = fs::read_dir("./.ahqstore").ok() else {
                ERR.println(&".ahqstore dir couldn't be accessed!");
                process::exit(1);
            };
            if !gh_action {
                INFO.print(&"INFO ");
                {
                    ::std::io::_print(format_args!("Checking .ahqstore\n"));
                };
            }
            let config = get_config();
            let repo = env::var("GITHUB_REPOSITORY").unwrap_or("%NUL".into());
            if &repo == "%NUL" {
                ERR.println(&"GITHUB_REPOSITORY not set");
                process::exit(1);
            }
            let r_id = env::var("RELEASE_ID").unwrap_or("latest".into());
            if &r_id == "latest" && upload {
                ERR.println(&"RELEASE_ID variable not present");
                process::exit(1);
            }
            if &r_id == "latest" {
                INFO.print(&"INFO ");
                {
                    ::std::io::_print(format_args!("Getting latest release\n"));
                };
            }
            let gh_token = env::var("GH_TOKEN").unwrap_or("".into());
            if &gh_token == "" && upload {
                ERR.println(&"GH_TOKEN variable not present");
                process::exit(1);
            }
            let (version, gh_r) = fetch_release(&repo, &r_id, &gh_token);
            let icon = get_icon(&config.appId);
            let dspl_images = get_images(&config.appId);
            let mut resources = HashMap::new();
            resources.insert(0, icon);
            #[allow(non_snake_case)]
            let displayImages = dspl_images
                .into_iter()
                .enumerate()
                .map(|(uid, icon)| {
                    resources.insert(uid as u8 + 1u8, icon);
                    uid as u8
                })
                .collect();
            let app_id = config.appId.clone();
            let mut final_config: AHQStoreApplication = AHQStoreApplication {
                releaseTagName: gh_r.tag_name.clone(),
                appDisplayName: config.appDisplayName,
                appId: config.appId,
                appShortcutName: config.appShortcutName,
                authorId: config.authorId,
                description: config.description,
                downloadUrls: HashMap::default(),
                displayImages,
                resources: Some(resources),
                license_or_tos: config.license_or_tos,
                install: InstallerOptions {
                    linux: None,
                    android: None,
                    linuxArm64: None,
                    linuxArm7: None,
                    winarm: None,
                    win32: None,
                },
                repo: config.repo,
                version,
                site: config.site,
                source: config.redistributed,
                verified: false,
            };
            let mut num = 0;
            num += 1;
            if let Some(platform) = config.platform.winAmd64Platform {
                if #[allow(non_exhaustive_omitted_patterns)]
                match &platform {
                    &InstallerFormat::LinuxAppImage | &InstallerFormat::AndroidApkZip => {
                        true
                    }
                    _ => false,
                } {
                    ERR.println(&"Invalid File Format, expected a valid windows format");
                    process::exit(1);
                }
                let Some(options) = config.platform.winAmd64Options else {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} Options not found!", "win32"),
                            )
                        }),
                    );
                    process::exit(1);
                };
                let Some(finder) = config.finder.windowsAmd64Finder else {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} Finder config not found!", "win32"),
                            )
                        }),
                    );
                    process::exit(1);
                };
                let assets = crate::app::build::find_assets(&gh_r, &finder);
                if assets.len() > 1 {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Multiple assets found while parsing {0}",
                                    "win32",
                                ),
                            )
                        }),
                    );
                    process::exit(1);
                }
                if assets.len() == 0 {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("No assets found while parsing {0}", "win32"),
                            )
                        }),
                    );
                    process::exit(1);
                }
                final_config
                    .downloadUrls
                    .insert(
                        num,
                        DownloadUrl {
                            installerType: platform,
                            asset: assets[0].name.clone(),
                            url: "".into(),
                        },
                    );
                final_config.install.win32 = Some(InstallerOptionsWindows {
                    assetId: num,
                    exec: options.zip_file_exec.map_or(None, |a| Some(a.to_string())),
                    installerArgs: options
                        .exe_installer_args
                        .map_or(
                            None,
                            |a| Some(a.iter().map(|x| x.to_string()).collect()),
                        ),
                    scope: options.scope,
                });
            }
            num += 1;
            if let Some(platform) = config.platform.winArm64Platform {
                if #[allow(non_exhaustive_omitted_patterns)]
                match &platform {
                    &InstallerFormat::LinuxAppImage | &InstallerFormat::AndroidApkZip => {
                        true
                    }
                    _ => false,
                } {
                    ERR.println(&"Invalid File Format, expected a valid windows format");
                    process::exit(1);
                }
                let Some(options) = config.platform.winArm64Options else {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} Options not found!", "winarm"),
                            )
                        }),
                    );
                    process::exit(1);
                };
                let Some(finder) = config.finder.windowsArm64Finder else {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} Finder config not found!", "winarm"),
                            )
                        }),
                    );
                    process::exit(1);
                };
                let assets = crate::app::build::find_assets(&gh_r, &finder);
                if assets.len() > 1 {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Multiple assets found while parsing {0}",
                                    "winarm",
                                ),
                            )
                        }),
                    );
                    process::exit(1);
                }
                if assets.len() == 0 {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("No assets found while parsing {0}", "winarm"),
                            )
                        }),
                    );
                    process::exit(1);
                }
                final_config
                    .downloadUrls
                    .insert(
                        num,
                        DownloadUrl {
                            installerType: platform,
                            asset: assets[0].name.clone(),
                            url: "".into(),
                        },
                    );
                final_config.install.winarm = Some(InstallerOptionsWindows {
                    assetId: num,
                    exec: options.zip_file_exec.map_or(None, |a| Some(a.to_string())),
                    installerArgs: options
                        .exe_installer_args
                        .map_or(
                            None,
                            |a| Some(a.iter().map(|x| x.to_string()).collect()),
                        ),
                    scope: options.scope,
                });
            }
            num += 1;
            if let Some(platform) = config.platform.linuxAmd64Platform {
                if !#[allow(non_exhaustive_omitted_patterns)]
                match &platform {
                    &InstallerFormat::LinuxAppImage => true,
                    _ => false,
                } {
                    ERR.println(&"Invalid File Format, expected LinuxAppImage");
                }
                let Some(finder) = config.finder.linuxAmd64Finder else {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} Finder Config not found!", "linux"),
                            )
                        }),
                    );
                    process::exit(1);
                };
                let assets = find_assets(&gh_r, &finder);
                if assets.len() > 1 {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Multiple assets found while parsing {0}",
                                    "linux",
                                ),
                            )
                        }),
                    );
                    process::exit(1);
                }
                final_config
                    .downloadUrls
                    .insert(
                        num,
                        DownloadUrl {
                            installerType: platform,
                            asset: assets[0].name.clone(),
                            url: "".into(),
                        },
                    );
                final_config.install.linux = Some(InstallerOptionsLinux {
                    assetId: num,
                });
            }
            num += 1;
            if let Some(platform) = config.platform.linuxArm64Platform {
                if !#[allow(non_exhaustive_omitted_patterns)]
                match &platform {
                    &InstallerFormat::LinuxAppImage => true,
                    _ => false,
                } {
                    ERR.println(&"Invalid File Format, expected LinuxAppImage");
                }
                let Some(finder) = config.finder.linuxArm64Finder else {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} Finder Config not found!", "linuxArm64"),
                            )
                        }),
                    );
                    process::exit(1);
                };
                let assets = find_assets(&gh_r, &finder);
                if assets.len() > 1 {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Multiple assets found while parsing {0}",
                                    "linuxArm64",
                                ),
                            )
                        }),
                    );
                    process::exit(1);
                }
                final_config
                    .downloadUrls
                    .insert(
                        num,
                        DownloadUrl {
                            installerType: platform,
                            asset: assets[0].name.clone(),
                            url: "".into(),
                        },
                    );
                final_config.install.linuxArm64 = Some(InstallerOptionsLinux {
                    assetId: num,
                });
            }
            num += 1;
            if let Some(platform) = config.platform.linuxArm32Platform {
                if !#[allow(non_exhaustive_omitted_patterns)]
                match &platform {
                    &InstallerFormat::LinuxAppImage => true,
                    _ => false,
                } {
                    ERR.println(&"Invalid File Format, expected LinuxAppImage");
                }
                let Some(finder) = config.finder.linuxArm32Finder else {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!("{0} Finder Config not found!", "linuxArm7"),
                            )
                        }),
                    );
                    process::exit(1);
                };
                let assets = find_assets(&gh_r, &finder);
                if assets.len() > 1 {
                    ERR.println(
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(
                                format_args!(
                                    "Multiple assets found while parsing {0}",
                                    "linuxArm7",
                                ),
                            )
                        }),
                    );
                    process::exit(1);
                }
                final_config
                    .downloadUrls
                    .insert(
                        num,
                        DownloadUrl {
                            installerType: platform,
                            asset: assets[0].name.clone(),
                            url: "".into(),
                        },
                    );
                final_config.install.linuxArm7 = Some(InstallerOptionsLinux {
                    assetId: num,
                });
            }
            num += 1;
            if let Some(platform) = config.platform.androidUniversal {
                if !#[allow(non_exhaustive_omitted_patterns)]
                match platform {
                    InstallerFormat::AndroidApkZip => true,
                    _ => false,
                } {
                    ERR.println(&"Invalid File Format, expected AndroidApkZip");
                }
                let Some(finder) = config.finder.androidUniversalFinder else {
                    ERR.println(&"Android Finder Config not found!");
                    process::exit(1);
                };
                let Some(options) = config.platform.androidOptions else {
                    ERR.println(&"Android Options not found!");
                    process::exit(1);
                };
                let assets = find_assets(&gh_r, &finder);
                if assets.len() > 1 {
                    ERR.println(&"Multiple assets found while parsing android");
                    process::exit(1);
                }
                final_config
                    .downloadUrls
                    .insert(
                        num,
                        DownloadUrl {
                            installerType: platform,
                            asset: assets[0].name.clone(),
                            url: "".into(),
                        },
                    );
                final_config.install.android = Some(InstallerOptionsAndroid {
                    assetId: num,
                    min_sdk: options.minSdk,
                    abi: options.abi,
                });
            }
            INFO.println(&"Validating config");
            match final_config.validate() {
                Ok(x) => {
                    {
                        ::std::io::_print(format_args!("{0}\n", x));
                    };
                }
                Err(x) => {
                    ERR.println(&"An error occured!");
                    {
                        ::std::io::_print(format_args!("{0}\n", x));
                    };
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("👆🏼 Please fix the above issues!"),
                        );
                    };
                }
            }
            let config_file = to_string(&final_config).unwrap();
            if !gh_action {
                {
                    ::std::io::_print(format_args!("{0} {1}.json\n", &*INFO, &app_id));
                };
                {
                    ::std::io::_print(format_args!("{0}\n", &config_file));
                };
            }
            if upload {
                let uup = gh_r
                    .upload_url
                    .replace(
                        "{?name,label}",
                        &::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("?name={0}.json", app_id))
                        }),
                    );
                let resp = CLIENT
                    .post(uup)
                    .header("Content-Length", config_file.len())
                    .header("Content-Type", "text/plain")
                    .header("Accept", "application/json")
                    .body(config_file)
                    .bearer_auth(&gh_token)
                    .send()
                    .unwrap()
                    .text()
                    .unwrap();
                if gh_action {
                    let val: GHAsset = from_str(&resp).unwrap();
                    {
                        ::std::io::_print(
                            format_args!(
                                "AHQ_STORE_FILE_URL={0}\n",
                                &val.browser_download_url,
                            ),
                        );
                    };
                } else {
                    INFO.println(&"GitHub Response");
                    {
                        ::std::io::_print(format_args!("{0}\n", resp));
                    };
                }
            }
        }
    }
    mod create {
        mod inquire {
            use std::process;
            use ahqstore_types::AppRepo;
            use inquire::{Editor, Text, validator::{ErrorMessage, Validation}};
            use rand::seq::IndexedRandom;
            use crate::app::{
                shared::{Config, IMetadata, IPlatform},
                ERR, INFO,
            };
            pub fn inquire<'a>() -> (String, Config<'a>) {
                INFO.println(&"Generating a random Application ID");
                let Ok(app_id) = Text::new("Application ID:")
                    .with_default(&gen_appid())
                    .prompt() else {
                    ERR.println(&"Must Enter an ID");
                    process::exit(1);
                };
                let Ok(app_name) = Text::new("Start menu entry name:")
                    .with_default("Application")
                    .prompt() else {
                    ERR.println(&"Must Enter a name");
                    process::exit(1);
                };
                let Ok(display_name) = Text::new("Application Display Name:")
                    .with_default("My Cool App")
                    .prompt() else {
                    ERR.println(&"Must Enter a name");
                    process::exit(1);
                };
                let Ok(user_id) = Text::new("Your AHQ Store Author ID:").prompt() else {
                    ERR.println(&"Must Enter an ID");
                    process::exit(1);
                };
                let Ok(desc) = Editor::new("Enter your app description").prompt() else {
                    ERR.println(&"Must Enter a description");
                    process::exit(1);
                };
                let Ok(repo) = Text::new("Enter your app repository:")
                    .with_default("owner/repoName")
                    .with_validator(|string: &str| {
                        if string.split("/").collect::<Vec<_>>().len() == 2 {
                            Ok(Validation::Valid)
                        } else {
                            Ok(
                                Validation::Invalid(
                                    ErrorMessage::Custom(
                                        "Must be in the format owner/repoName".into(),
                                    ),
                                ),
                            )
                        }
                    })
                    .prompt() else {
                    ERR.println(&"Must Enter a repository");
                    process::exit(1);
                };
                let [owner, repo] = repo.split("/").collect::<Vec<_>>()[..] else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Repo Parsing Failed"),
                        );
                    }
                };
                INFO.println(&"Validating author id & repo");
                (
                    app_id.clone(),
                    IMetadata::new(
                        app_id,
                        app_name,
                        display_name,
                        user_id,
                        desc,
                        AppRepo {
                            author: owner.into(),
                            repo: repo.into(),
                        },
                        IPlatform::new(),
                    ),
                )
            }
            fn gen_appid() -> String {
                let mut string = String::with_capacity(40);
                let val = <[_]>::into_vec(
                    ::alloc::boxed::box_new([
                        "a",
                        "b",
                        "c",
                        "d",
                        "e",
                        "f",
                        "g",
                        "h",
                        "i",
                        "j",
                        "k",
                        "l",
                        "m",
                        "n",
                        "o",
                        "p",
                        "q",
                        "r",
                        "s",
                        "t",
                        "u",
                        "v",
                        "w",
                        "s",
                        "y",
                        "z",
                        "A",
                        "B",
                        "C",
                        "D",
                        "E",
                        "F",
                        "G",
                        "H",
                        "I",
                        "J",
                        "K",
                        "L",
                        "M",
                        "N",
                        "O",
                        "P",
                        "Q",
                        "R",
                        "S",
                        "T",
                        "U",
                        "V",
                        "W",
                        "S",
                        "Y",
                        "Z",
                        "0",
                        "1",
                        "2",
                        "3",
                        "4",
                        "5",
                        "6",
                        "7",
                        "8",
                        "9",
                    ]),
                );
                for _ in 0..40 {
                    let val = val.choose(&mut rand::rng()).unwrap();
                    string.push_str(val);
                }
                string
            }
        }
        use std::{fs, process};
        use inquire::*;
        use serde_json::to_string_pretty;
        use super::{ERR, INFO, WARN};
        pub fn create(force: bool) {
            let (id, config) = inquire();
            create_dir(force);
            let succ = (|| {
                let config_file = to_string_pretty(&config).ok()?;
                fs::write("./.ahqstore/config.json", config_file).ok()?;
                let base_img = ::alloc::__export::must_use({
                    ::alloc::fmt::format(format_args!("./.ahqstore/images/{0}", &id))
                });
                fs::create_dir_all(&base_img).ok()?;
                let icon = b"\x89PNG\r\n\x1a\n\x00\x00\x00\rIHDR\x00\x00\x02\x00\x00\x00\x02\x00\x08\x03\x00\x00\x00\xc3\xa6$\xc8\x00\x00\x00\x04gAMA\x00\x00\xb1\x8f\x0b\xfca\x05\x00\x00\x00\x01sRGB\x00\xae\xce\x1c\xe9\x00\x00\x03\x00PLTE\x00\x00\x00\xf7\xc4{\xf3\xc0{\xf6\xc2y\xf4\xc1y\xe1\x97G\xf3\xbcr\xff\xce|\xe7\xa6]\xf3\xc0{\xf9\xc6{\xf7\xc4z\xf8\xc5{\xf9\xc7{\xf9\xc5{\xf3\xc0{\xff\xce|\xf9\xc5{\xf3\xc0{\xd5\x92R\xff\xce|\xff\xce|\xf3\xc0{\xf2\xbfz\xe1\x96G\xf9\xc7{\xe1\x97G\xe1\x97G\xf3\xc0{\xe1\x97F\xde\x95H\xf3\xc0{\xd9\x93M\xf2\xc0{\xf5\xc0{\xf4\xbfy\xf3\xc0z\xf3\xbf{\xf2\xbfz\xf3\xc0{\xf3\xc0{\xf3\xc0{\xff\xce{\xdc\x94J\xff\xce{\xdb\x94N\xf3\xc0z\xf3\xc0z\xf3\xc1{\xff\xcd|\xf5\xbfy\xd3\x90P\xff\xce|\xe1\x97G\xff\xcd{\xff\xce{\xe2\x97H\xe1\x97G\xff\xce|\xff\xce{\xf3\xc0{\xd7\x93R\xff\xce{\xff\xcd{\xf3\xc0{\xff\xce{\xe1\x97G\xff\xce|\xe1\x97F\xe1\x97F\xe1\x97G\xe1\x97G\xf3\xc0z\xf2\xc0y\xe1\x97F\xf3\xc0{\xff\xce|\xe1\x97F\xff\xcf|\xe1\x97F\xf3\xc0{\xff\xcd|\xf3\xc0{\xff\xce{\xff\xce|\xe1\x97G\xf3\xc1|\xff\xce|\xff\xce{\xe1\x97F\xff\xce|\xe1\x96F\xe1\x97F\xe1\x97F\xff\xcbz\xe2\x97G\xe1\x97G\xe1\x97F\xe1\x97F\xff\xcd{\xff\xce|\xf3\xc0z\xf3\xc0z\xf3\xc0{\xf3\xc0{\xe1\x96G\xf2\xbex\xf3\xc0{\xca\x94f\xf3\xc0z\xf3\xc0z\xb9\x7f]\xc9\x90e\xb8~\\\xc4\x8bb\xf3\xc0{\xaftX\xca\x91e\xd0\x99i\xd6\xa0l\xe1\x97G\xbc\x83^\xf3\xc0z\xe0\x98E\xe1\x97F\xf3\xc0{\xb3xZ\xca\x92e\xca\x94g\xc5\x8bb\xd5\xa0l\xcc\x92f\xd2\x9cj\xd7\x9dl\xff\xce|\xaerW\xdf\xabr\xed\xb3j\xe7\xa2Q\xe9\xa5U\xf6\xc4{\xff\xce|\xe1\x97G\xf3\xc0{\xa5iS\x8cUC\xd3\xd3\xd3\xe8\xa4T\xfe\xcd|\xf3\xbfz\xc5\x81F\xd3\x8cF\xd8\xbd\xa1\xa8lT\xf2\xbey\xfd\xcb{\xf5\xc2{\xaanU\xf6\xc4{\xfa\xc8{\xfb\xca{\xeb\xb7v\xe8\xb3t\xf8\xc5{\xe2\x99I\xa6jS\xacqV\xc9\x91e\xe3\x9bL\xb6|[\xee\xbax\xf1\xbdy\xdc\xa6n\xd8\xa2m\xef\xbcy\xd3\xd3\xd2\xd5\x9ek\xb9\x7f]\xa4hR\xe3\xaer\xc6\x8ed\x90XD\xe0\x99K\xaftX\xbc\x83_\xc3\x8ab\xf1\xbcv\xe0\xabq\x94[F\xb1wY\xbf\x86`\xcc\x95g\xd5\xc9\xbc\xd3\xd1\xcd\xd2\x9bi\xb3yZ\xeb\xadb\xed\xb2i\x8eVC\xe4\x9eP\xe9\xa9^\xa0eO\xe6\xa2T\xde\x94F\xde\xa8p\xaesW\xe0\x9bQ\xdf\x9eW\xd4\xce\xc8\xde\xa1`\xdb\xb0\x81\xd0\x99i\x98^J\xee\xb5m\xe7\xa4W\xf0\xb8p\xe5\xb0s\xd9\xb8\x94\xd8\xbc\x9d\x9cbM\xf0\xbar\xe8\xa7[\xd9\x90F\xb9xE\xcf\x98h\xd6\xc5\xb3\xdd\xa5i\xd7\xc2\xad\xec\xb0f\xc2\x89a\x99_C\xb1rD\xd7\xc0\xa6\xe4\xb0s\xda\xb3\x87\xf0\xbat\xd1\x8aF\xd4\xcc\xc3\xa4gD\xdc\xabu\xdc\xadz\xda\xb4\x8b\x9fdC\xc1~E\xcb\x86F\xc9\x84E\xd4\xcb\xc1\xabmD\xa8jD\xf1\xb4c\xea\xa8W\xcc\x87F\xce\x8bL\xd2\x92V\xf8\xc3q\xe1\xafvoS\x1a\x8c\x00\x00\x00\x8dtRNS\x00\x11\xfa\x06\x02\xfd\td\x01\xfe\r\'\x17-L\xf6\xfd!\xe8\x1d\xf9\xed\xed8%<\xf9\xf3\xf2\xcd\x15\xe3\x05\xa4\x99ox\xddE\xb2\xba\xd7\xb1\x10C8\xc6\x7fT\x861\x0c\xc9\xed4\xb9M\xd6\xdf\x8e\xcaF\xa9V\xc0\x1c\xe8}=\x81^g\x88\x9f\xc5\xab\xe7\xb5]\xa1e\x97ht\xf2\xab]\xd0\xc1p\xd7V\x88x\xa0\x99\xdd\xe3\xbcj\xf5\xd2\x1d\x91\x1b.`\x900\xd0\x85\xde\x7f\xef\xce\xcf\xf9\x8fl[\x90\xa0\x8c\x8c\x92X\xf6\xc2\xb0\xe1\xb1\xb8\xa4Q\x96\xfdv\xef\x8d\xea\xe9 \xaf\xa0\xcf\x00\x00\x13\x84IDATx\xda\xed\x9dix\x15\xd5\x1d\xc6\x87\x10i\x12 l\x1a\x04Q\xc2&\x9b\x8a\xa0(*n\x15\x17D\x04qi\xadZ\x95VQlq\xad\xad\xda\xda\xda}yf\xe6\xe6\xde\xa0\xc4\x10\x12V\xc3fB\xc1\x08\"K@\xa9\x15\x01\xc1\xb6\x02Z\x05\x11\x10\xb5\xae-\xadvyz\xd9BBn\xee\x9d9s\x96\xff9\xe7\xfd}\xe8\xc7r3\xf3\xf3\xfc\xef=\xef{f\x1c\x07\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x80(\xcd\xc6w\xbf\xbf}V\x97\xae}.\xeb\x94\x8d\xaba\x1d\x1dGd\xf9ut\xed\x99\x87+b\x15\xad\xba\xe5\xfa\rh?>\x07W\xc5\x1a\n{t\xf1\x1bqAk\\\x18[V\xff\xfb\xfdTd\xdd\x859`\xc5w\xbf\xee\xb9~\x13t=\x15\x97\xc7trF\x17\xf8i\xe8\xd3\x12\x97\xc8hZ\x9f\xe2\xa7\'kB\x0b\\%c\xc9\x1b\x97\xe5g\xa4W>.\x94\xa1\x9c\xda\xd5\x0fD\xefv\xb8V\x06\xd2\xb2\x8f\x1f\x946=\x9b\xe3z\x19F\x8b\tY~\x08\xfa\x9e\x88Kf\x14\xf9\xbd\xfcp\xe4\x8eh\x85\xabf\x0c\xedz\xfb\xe1)\xe8\x81\x88\xc8\x0c\x9a\xf7l\xe33\xf1\xdd\x8e\xb8x\x06pb_\x9f\x95\xdc\xee\xcdp\xfd4\xa7Q\xec\x17r\x0e\x8cFH\xa83)c\xbfp\x9c2\x08\x97Q[:\xde\xe6G\x07!\xa1\xae\xa4\x89\xfd\xc2\xd1\xfe:\\L\xfd\xc8N\x1f\xfb\x85\xe3\x02\x84\x84\xba\x911\xf6\x0b9\x07.CH\xa8\x13\x81b\xbfptEH\xa8\x0f?j\xef\x0b\x00!\xa1&\xb4\xbc\xc0\x17C\x9b\x01\x08\t\xe9\xd3\xe2\xb2,_\x18\xbd\x10\x12R\'t\xec\x176$\xbc\x1d\xd7\x980L\xb1_8\xba\xf4,\xc4u&\ns\xec\x87\x90\xd0\x08\"\xc4~!\xe7\xc0p\x94E\xe8q\xc7\x88\\_\x1a\x05cP\x16\xa1\x05\x87\xd8\x0f!\xa1\xc6t\xba\xcd\x97\r\xca\"th6<\xd7W\x00BB\x1ad\x8f)\xf0\x15\x81\x13\xe5\x04\x184\xd4W\x07BB\xe5\xab\xff\xb8\\_)8Q\xae\x92\x9c\xeb\xda\xfb\xca\xc1\x89re\x08\x8b\xfd\x10\x12\xea\x80\xd0\xd8\x0f!!y\xf2\xbb\xfa\x84\xe8\xdd\x01wD*\x12b\xbf0L\\\xf5\xc0=\x85\xb8+\xd2h>\xa0\r\xa9\xfb\xbfpv,\x16\xbb\xfb\\\xdc\x18IH\x8b\xfd\x821gIl?3\x7f\x829 \x83;F\x90\xba\xfd\xb9\xd5\xb3b\x87x\xe0\x1e\x84\x84\xa2\x91\x1e\xfbe`\xf3\xecX}\xee>\x19\xb7H(\nb\xbft<\xfd\xd4\xccXCJ\x1e:\x01wI\xdc\xc6\xef\xf0\\R\xdf\xfd\xe7\xaf\x8b5\xe6\xd1\x07q\xa2\\\x0c\nc\xbf\x94L[\x10K\xcd}\xdf\xc1\xcd\x12@\xc7\xa1\xb4V\xff\x15%\xb1\xa6({\xe84\xdc/\xde\xab\xff8R\xab\xbf?\xef\xd9X:\xee\xfd>n\x19OH\xc4~\xf5\x98\xba8\x96\x81\x99?\xc6\x1c\xe0\x07\x91\xd8\xef\x10s\xb7\x95\xc42\xf3\xab\xefa\x0e\xf0!\x8fN\xec\xb7\x9fW\x96\xc5\x82q\xf7/p\xf38pjWZ\xab\xff\xceX`J~\x86\xcd\xe1\xa8\x10\x8b\xfd\xe6\xbeV\x16\x0b\x03B\xc2h\xb4 \x16\xfb-]\x1d\x0b\x0bB\xc2\x08\x9c\xd8\x8b\xd4\xed?\x14\xfb\x85\xa3\x0c!!#\xc4b\xbf\x89\xd5e16\x1e}\x10!ax\n{\xd2Z\xfd\xf7\xcc\x8e13\xf3>\x84\x84a!\x16\xfb\xcdY\x14\x8b\xc4\xac\x9f\xf7\xc7=\rA\xc4G<s/}\xcc\x9f\x15\x8b\xca\xbd\x08\t\x03\xa3M\xec\x17\x8a\x12\x84\x84\x01\xa1\x16\xfb5*}\xb0\x82\xcd\xe1 p{\xc43\xaf\xd8o]\x8c\x1f\x08\t3A-\xf6\x9b\xb68\xc6\x95\x12\x84\x84iiM,\xf6[Q\x12\xe3\xcd2\xcc\x81&\xd16\xf6\x0b\xc7c\x98\x03\xa9\xa1\x16\xfb-\x8f\t\xa2\xec\xa7\xd8\x1cnL\xbb>\xb4V\xff\xd7Jb\xe2x\x14!\xa1y\xb1_\xb8\xcd\xe1\xc7\x10\x12\xd6\'\x9fV\xec\x17\xa6\xf4\xc1\xbc9\x8c9P\xc7\xed\xc4b\xbfUe1\x19\xdc\xfbk\xcc\x81}4\'\x16\xfb-\\\x1d\x93CY\xf5mx\xec4\xbd\xd8o\x89\xa4\xdb\x1f\xdb9\x15\x8f\x9d\xa6\x17\xfbU\xcf\x92t\xfb\x97-=\xf0/\xda\xfd\xd8\xe9\xec1\xa4\xcfz\x8b\xa3d\xdb\xdc\xba\x7ft\xa8\xbd\x8f\x9d66\xf6\xcb\xc4\xf2\xa9\r\x96\x1dK\x1f;M,\xf6K}\xd6[\x04\xcf\xce;\xf2\xdf\xb6\xf1\xb1\xd39\xe3\x89\xc5~\x0bd\xad\xfe+\xe6\xa6\xf8\xe7\xad{\xec4\xe77\xbbF^\xfdW\x94H\xba\xff\x8b\xa7\xa5\xfe\x04Y\x0f\xdb\xf4\xd8\xe9\xbc\x87i\xc5~\x19\xcez\xf3c\xdd\xfc\xa6?\x84E\x8f\x9d\xa6\x16\xfb-\x96\xb5\xfa?\xf5t\xda\x0fb\xc9c\xa7\xa9\xc5~\xdbd\xad\xfe\x0b\xa6e\xfa,Y\x03\xcc\x9f\x03\xe4b\xbfe\x92n\xff\xac\xea ?z\x8c\x7f\xec\xb4\x85\xb1\xdf\x81\xf8w\xd1\x9c\x80\x1f\xa9\xb7\xc9\xef\xa6%\x16\xfb\x85=\xeb\xcd\xce\xec=\xc1?U\x97\x9e\xa6\xbe~\xc0\xe2\xd8ob\xa8\x0f\xd6\xd7\xcc9\xd0\x89\xe6#\x9e\xc5\xb3dN\xd8\xcf\x96\xdb\xcd\xbc\x90\x90X\xec\xc7~\xd6;,\xab\x17\xb2|\xbe\x82\x1ef\x85\x84\xd9\xc4\x1e\xf1\xbcGV\xecW\xf6\xdaD\xc6\x8fh\xd4;\xca\x89\xc5~s\x16\xc9\x8a\xfdvN\x8d\xd0M0&$$\x16\xfb\xf18\xeb\x1d\xae\xf4\xc1J\xc1h\x13N\x94\xe7\x8c7\xf1\xacw\xc8\xd2\x07+\xa7\xe8\x1f\x12R\x8b\xfd\x14\x95>X\xc9\xba+O\xef\xd8o\x9c\xa5\xb1_\xe3\xd2\x07+\xed\x7f\x84\xd8\x8f\xdb\xea/-\xf6[1\x97\xe3\xc7\xd66$lI,\xf6S^\xfa`\x9e\x03\x13t\x0c\t[L\xb0\xe2\xacw\xb8\xd2\x07+\xbd\xf2\x11\xfbE\x8c\xfd\x96\xcbZ\xfd3\x94>X\xe9\xddN\xaf\xd8O\xefG<\x8b,}\xb0\xa2\xd3;\xca\xa9\xc5~Ke\xc5~\xc1J\x1f\xach\x13\x12R{\xb3+\xbd\xd2\x07\xeb\x1e\xe6\x88;\x10\xfb\x85\x8e\xfdVI+}l\x16\xff\xd7t!\x1f\x12R{\xb3\xebBi\xb1_\xf5D)\x7f\x10\xf1\x90\xb0\xe3\xfd\xc4b?\xba\xa5\x0f\xe69@\xf8D9\xb5\xd3~\xd2\xcez\xb3\x95>L\x0b\tsF\xd3\x8a\xfd6\xd3/}0\x87\x84\x14O\x94\x0f\xb25\xf6\xdb9U\xfe_\x97{\x17\xb5\xcd\xe1\x9c\x01\xa46~\xe5\x9d\xf5\x8eZ\xfa`\xde\x14\xa0\x95\x105\xef\x86\xd2\x87d\nH\x8d\x81\xe1v\xc6~\xcb\xa7*\xfc3\xdb\x13:B4\x06\xa5\x0f\x05\x0c%\xb3)t;\xa1\xcd\x1fy\xb1\xdf\x8a\xb9\xaa\xff\xd6\x1eT\x04\xe8Ng\xf5\xdf\xa6k\xe9\x83i\x08\x10\xf9)\xd0\x8cL\xf6\xb7T\xe7\xd2\x07\x03\xa3i\x080>\xf9Q>x\xf3I\xf5\xab\xbf\xac\xd8OT\xe9#\x0c\x93\'\'\xff\xa7\x0f\r\x01\xbaM|\xb7\xd2\xf3\xbc\xbf\xbc\xa9\xf4\x8a<i@\xe9#0\xd3\x8b]\xb7\xb8\xdc\xcf\xa2\xd1\x11\xe9\xfb\x92w\x80\x97\x14\xae\x02\xd2b?\xb1\xa5\x8f@L)u\xf7S\xea\xd3\xd8\x0b\xf8\x8dw\x88\xca\x17U\xc5~\xb2\xcez\x0b/}\x04\xa0\xc2=D\x05\x8d\xae\xe8\xcb\xdea^V1\x07r\xabM*}\x04X\xfd\xeb\xa0\xf1d\xb9\x19^}\xe4\xcf\x81=\xa6\x95>\xd2}\xf7+u\xebs>\t\x01\xbc\x86H\x9e\x03\xf2\xcez/Q\xbf\xfa\x97\xbb\r\xf9\nE\x01\x92\xbf\x07>\x90\xb7\xfa\xcf7\xb3\xf4\x91y\xf5\xa7,\x80\xe7\xbd i\x0eH\x8b\xfd\xe4\x97>2\xad\xfe\xb4\x05\xf0\xfe\xf0g\t\x97\xc4\xec\xd2\xc7\x91\xab\x7f\xb1\xab\x93\x00\xc99\xf0\x9c1\xb1\x9f\xaa\xd2G\xda\xd5\x9f\xbc\x00\xde\x8c\x17\x84\xae\x9a\xf2\xcezoS\x1e\xfbM\xaep]\xfd\x04\x10:\x07\xe4\xc5~\xcb\x89\xae\xfe:\x08\xe0y\x7f\x154\x07\xa4\x9d\xf5~\xf6\x15\xe5\xb7\x7fJ\xa9\xebj+\x80W\xf9\xae\x809`S\xe9\xc3\xafp]\x9d\x05H\xce\x81\xd7y\xaf\xfe\xd2b?\x02\xa5\x8f\xa6W\x7fm\x04\xe0\xbd9,\xed\xac\xf7\xbay\xa4W\x7f\x8d\x04\xf0*_\xe46\x07\xa4\xc5~\x14J\x1f\x15\xaek\x86\x00\xdcBByg\xbd\x89\x94>\x8c\x11\x80\xcf\x1c\xb0\xb1\xf4a\x8c\x00\xd1CBig\xbdg.R\xbf\xfa\x97\xbb\xaei\x02$\xe7@\x94\x90P\xdeYoj\xa5\x0fs\x04\x88\x12\x12N\xb3\xb7\xf4a\x92\x00^%\xdb\xe6\xb0\xbc\xd8\x8f`\xe9\xc3(\x01\x98BByg\xbdI\x96>\x0c\x13 |H(-\xf6+[\xa5\xd3\xea\xaf\xad\x00!CByg\xbdwR\x8e\xfd\x8c\x12 LHhS\xe9cJ\xd8\xdb\xaf\xaf\x00A\xe7\x80\xb4\xd8\xaf\x8cp\xe9\xc3H\x01\x02\x85\x84(}\x98,@\xe69 -\xf6#^\xfa0V\x00\xaf\xf2\xddt\xab\xbf\xb4\xb3\xde\xd4K\x1f\xe6\n\xe0y/\xbf\xae<\xf6\xa3_\xfa0Y\x80\xa6BBi\xb1\x9f\x0e\xa5\x0f\xb3\x05H\x15\x12\xa2\xf4a\x93\x00\x8d\xca\"\xb9\xd2b?MJ\x1f\xc6\x0b\xd0p\x0eH;\xeb=k\xbev\x1b\xbf\xc6\np8$\x94v\xd6[\xa3\xd2\x87\r\x02\x1c<Q./\xf6\xd3\xa9\xf4a\x87\x00\xde\x8c\x17\x9e\x94w\xd6\xbb\xda\x80\xd5\xdf4\x01<\xef?\x7fD\xe9\xc3j\x01\x1e/\xfa\xd3\xefe\xac\xfe\x0b\xcdX\xfdM\x14\xa0\xe8\x997P\xfa\xb0Z\x80\xa2\xa2\xe7\xc5\xce\x81\x9d\xba\xc6~\xd6\x08P$r\x0eP(}p\xfc\xcf\xdfP\x01\x84\xcd\x01]K\x1f\xd6\t\x90\x9c\x03\"\x16\x81\xe5\x86\xad\xfe&\x0bP\xb4\x86\xfb\x1c\xd0\xb8\xf4a\xa3\x00\xc99\xc0\xf5\xcb\xa0\xd6\xa5\x0f;\x05\xe0:\x07\x16O3\xe8\xa7\xbf5\x02\x14\xady\x83\x8f\x02\xba\x97>\xac\x15\x80\xcf\x1c\xd0\xbf\xf4a\xb1\x00\x1c6\x05\x16\x18\xbb\xfa\xdb!@r\x0eX_\xfa\xb0[\x80(\x9b\xc3\x86\x94>l\x17\x80y\x0e,\xd8l\xf4\xeao\x91\x00L\x9b\xc3\xe6\x94> \x00\xc3\x1c\x98iP\xe9\x03\x02\x84\xdf\x1c6\xaa\xf4\x01\x01\xc2n\n\x18V\xfa\x80\x00!7\x87w\x9a\x18\xfbA\x80\xa0\x9b\xc3\xab\x8d+}@\x80\x10s\xc0\xc4\xd2\x07\x04\x08\xbe)\xb0\xdc\xa2\xd5\xdfV\x01\xd2l\x0e\x1bZ\xfa\x80\x00\xc16\x05\x8c-}@\x80@s`\xf14K~\xfaC\x80T\x9b\xc3&\x97> @\xc69`v\xe9\x03\x02d\x9a\x03\x0b\xac\\\xfd!\xc0\xc1M\x01\xe3K\x1f\x10 \xdd\xe6\xb0\x05\xa5\x0f\x08\x90nS\xe0\xbf\xcf\xd9\xba\xfaC\x80\xfd<!\xf8\x1d\xe5tW\x7f\x08p@\x00\xa1\xef(\xa7\xbc\xfaC\x80C\x02\x88{G9\xe9\xd5\x1f\x02\x1c\x16@\xcc;\xcaI\xc5~\x10 \xad\x00\x02\xdeQN,\xf6\x83\x00\x19\x04\xe0\xfd\x8er\x92\x1b\xbf\x10 \x9d\x00<\xdfQ\xae\xc3\xea\x0f\x01\x8e\x14\x80\xdb;\xca\xf5X\xfd!@c\x01\xe4\xcc\x01\"\xab?\x04H%@\xf4w\x94\x13\x8d\xfd @@\x01\x0e>v\xda\xe0\x9f\xfe\x10 \x83\x00\xfb\x1e;m\xc3\xea\x0f\x01\x9a\x12\x80\xf9\x1d\xe5\xe47~!@@\x01\x98\xdeQ\xae\xdb\xea\x0f\x01\xd2\t\xe0q\x0f\t\'\x97\xba.\x04\xd0G\x00\xde!a\xb9\xebB\x00\xbd\x04\xe0\x19\x12R\\\xfd!@F\x01\xbc\x19|BB\x9a\xab\xbfy\x02\xfc\x9d\xbb\x00|B\xc2\xf2b\x17\x02\xc8\x10`\xed\x1a\xfe\x02D\x9f\x03\xd3\t\xdf~\xc3\x04\xf0v\x8b\x10 \xda\xe6\xf0\xe4\n\xd7\x85\x00\xb2\x04X\xfb\x91\x08\x01\xd2\xbc\xa3\\\xeb\xd5\xdf@\x01<\xef\xedgD\x08\xc0\x1a\x12N)u]\x08 U\x00\xef\xad\x7f\n\x11\x80e\x0eP_\xfd\xcd\x14 \xf9[`\x97\x08\x01\xc2\x97E\xc8\xaf\xfe\xa6\n\xe0\xad}{\x8d\x08\x01</LH\xa8\xc1\xeao\xac\x00\xc99\xf0/!\x02\x84\x08\t+\\\x17\x02\xa8\x13\xc0\xf3\xfe\xb1K\x84\x00AC\xc2\xe9\xc5.\x04P+@\xf09\x10N\x80 !\xa1.\xab\xbf\xd9\x02x\xde\x13\x1f\x89\x10 sHX\xee\xba\x10\x80\x82\x00\x9e\xf7\xf8\xf3\"\x04H?\x074Z\xfd\xcd\x17\xc0[\xbb{\x8d\x08\x01\x9a\x9e\x03\x93K]\x17\x02\xd0\x11\xc0\xf3>\xfcL\x84\x00M\x85\x84\xe5\xae\x0b\x01h\t\xe0y\x9f?/B\x80T!\xa1n\xab\xbf%\x02xo\xed\x16\"\xc0\x91\'\xca\'W\xb8.\x04\xa0(@\x869\xf0\x04\xfb\xffo\xfd\x90\xb0\xbc\xd8\x85\x00T\x05H\xce\x81gD\x08p8$\x9c\xae\xe9\xed\xb7F\x804!a$\x01\x0e\x84\x84\x9a\xae\xfeV\t\xe0y\xff\xde%B\x80}!ay\xb1\x0b\x01\xe8\x0b\xe0\xadM]\x16\x89*\x80\x97p]\x08\xa0\x83\x00M\x84\x84\x10\xc0\x1e\x01R\x86\x84\x10\xc0&\x01R\x84\x84\x10\xc0*\x01\x1a\x87\x84\x10\xc02\x01\x8e\x0c\t\xdf\x82\x00\xb6\t\xd0 $\xdc\xe5A\x00\xeb\x04\xa8\xbf9\xfc9\x04\xb0Q\x80\xba\x90\xf0m\x0f\x02\xd8)@\xf2\xf7\xc0G\x9f\xed\xfe\xd0\x83\x00\xb6\n\xc0\r\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01 \x00\x04\x80\x00\x10\x00\x02@\x00\x08\x00\x01\xb8\xd3\x19\x02\xa8\xe2J\x12\x02\x0c\x86\x00\xaa\xc8\'!\xc0Y\x10@\x15g\x90\x10`$\x04PE+\x12\x02<\x02\x01\x14q\x0c\x89\xfb\xef\xdc\x00\x01\x14q&\r\x01:@\x00E\\DC\x00\xad\xbf\x05&\xf0\x1d0:\x17\xcb\xbek\xefl\xda\xb4\x11\x02\xb8g\xe7\x10\x11\xe0X\xb9\xb7\x7f\xc3\xf6x<^\xf5\xde\x06\xeb\x05\x18\xe8Pa\xac\xcc\xfb_\xb32\xbe\x9f\xdaOk\xec\x16\xa0\xed\x85d\x04\x18\"S\x80O\xe2\x87X9\xcfj\x01\xces\xe8p\xb9D\x01\xb6\xd6\t\xc0c\x0e\xe8+@\xdb\x96\x84\x04\x18&Q\x80I\xf1z\xd4\xbe_c\xab\x00\xbft(1J\x91\x00\xc99\xf0\x8e\x9d\x02\\\xd1\x8c\x94\x00\xfdoU%@\xbcj\xd2\x06\x1b\x05\xc8wh1\xac\xb3*\x01\xa2\xcd\x01]\x058\xc7\xa1\xc6\xa5\xea\x04\x882\x074\x15\xe0\xccBr\x0287*\x14 ^\xb5u\xbdM\x02\x9c\x9e\xe78\xd6\x1a\x90R\x80x\xfc\xd5M3\xac\x11\xe0\xf4f\x0eI\xae?J\xa1\x00\x8cs@G\x01\xfa\xe59D\x192X\xa5\x00Ls@?\x01\xda\x0e\xccv\xc8\xd2a\xa4J\x01\xf6\xcd\x01\xe3\x058\xfb$\x874\x97|U\xa5\x00\xc99\xb0\xd1h\x01\x8e>\'\xcf!N\xff\x1f\x1e\xa5R\x80\xb0s@/\x01\xbe\xde\xda\xd1\x80;\x8fW)@\xc89\xa0\x93\x00\xc7\\\xe9\xe8A\xf6\xa5\x83U\n\x10\x8fo\xdfh\xa0\x00m\xafj\xe6h\xc3\t7\x1f\xa5R\x80\x10s@\x1b\x01N?\xc3\xd1\x8a\x93\x8fW)@\xf09\xa0\x89\x00\xc7|+\xdb\xd1\x8c\xc2\xeb\x8fS)@\xd09\xa0\x85\x00m\xbf\xd9\xca\xd1\x90\x13F)\x15 ^\xb5e\xbd\x19\x02|\xad\x93\xa3)\xe7\x9e\xa5R\x80\xe4\x1c\xf8\xc4\x00\x01\xae\xb9:\xdb\xd1\x96\xc2\x9b\x8eS)@\x809@^\x80\xf3.t\xb4\xa6\xc3\xb5J\x05H\xce\x81\x1a\x9d\x05\xf8\xc6I\x8e\xf6\x0c\xb9E\xa5\x00\x99\xe6\x00i\x01\xbe}Qs\xc7\x00Z\\\xdcY\xa5\x00\xf1\xf8\x8e\x8dz\n\xd0\xaf\x9dc\x08\xc7\x8eU*@\xbc\xf6}\r\x058;\xdf1\x88\x1bnU)@<>\xa9F3\x01\x8e\x1e\xd8\xc21\x8a\xd3n\xec\xacR\x80\xf8V\xbd\x048\xb3\xa5c\x1cw^\xaeR\x80\xf8\xc7\x1a\tp\xc5\xf9\x8e\x89\xe4\xf0)\x8b0\n\xb0\xb7F\x17\x014(}(-\x8b0\n\xd0\xc4\x12@O\x00=J\x1f\nCBV\x01&i!\x806\xa5\x0fue\x11V\x01Vj \x80V\xa5\x0fU!\xe1VF\x01j\xe9\x0b\xa0[\xe9\x83\x95a\x91\xe6\xc0\x16F\x01\xaa\xa8\x0b\xa0a\xe9CIYd\x13\xa3\x00q\xda\x02hZ\xfa`\x9e\x03\xec!\xe1\x86*\x13\x05\xd0\xb7\xf4!\xbf,\xb2\xdd<\x01\xb4.}H/\x8b\xcc3N\x00\xddK\x1f\xcce\x91\x91R\x7f\x08R\x15\xc0\x84\xd2\x87\xdc\xb2H\xcdv\x83\x040\xa4\xf4!\xb7,R\xb3\xc3\x18\x01\xcc)}\xc8-\x8blz\xd5\x08\x01\xcc*}H-\x8b\xac\xdfZ\xa5\xbd\x00\xc6\x95>\x98\xcb\"\x8f\xb0\x84\x84\xef\xac\xd4\\\x00\x13K\x1fR\xcb\"5\x9bj5\x16\xc0\xd0\xd2\x87\xdc\xb2\xc8\xfaIU\x9a\n`p\xe9\x83\x95\xfe7\x8b\x9e\x03\x84\x040\xbb\xf4\xc1\nSY\xa4\xe6\xfdZ\xed\x040\xbe\xf4\xc1\n[Yd\xc3{UZ\t`E\xe9\x839$d*\x8b\xcc[\xa9\x91\x00\xb6\x94>X\x19&n\x0eP\x10\xc0\xa2\xd2\x07sH\xc8T\x16\xd9\xf0\x9e\x0e\x02XV\xfa`\x0e\t\xaf\x153\x07\x94\x0b`_\xe9\x83\x15\xa6\xb2H\xcd\xa7\xb5\xa4\x05\xb0\xb2\xf4\xc1<\x07n\x120\x07\xd4\n`k\xe9\x839$d*\x8b\xbc\xb2\x97\xa8\x006\x97>Xa*\x8b\xa4\x9b\x03\xea\x04\xb0\xbc\xf4\xc1\n[Yd\xc3\x0er\x02\xa0\xf4\xc1<\x07\xc6\xf2\x9c\x03\x8a\x04@\xe9#\nLe\x91\x9a-\xb5d\x04@\xe9#\"\xa7\xfd\x80%$\xdc\xb8\x83\x88\x00(}D\x87\xed\xc9\"\x1f\xff\x8d\x80\x00(}p\x81\xad,R\xb3\xa5J\xb1\x00(}p\x83\xed\xf5\x03G\xce\x01\xc9\x02\xa0\xf4\xc1\x13\xb6\'\x8b|\xfc\xaa\xb2\xe3\xe1(}p\x86\xad,\xb2\xbe\xde\x1c\xd8+Q\x00\x94>D\xcc\x01\xa6\xb2\xc8\xc6\xbacd[\xe4\t\x80\xd2\x87\x18\xd8\x9e,\xf2\xc9\x819\xb0]\xdac\xe2P\xfa\x10\x06[Yd\xfd\x96\xbd\xb5+?\xad\x91\xf4\xa0H\x94>\x84\xc2\xfd\xf5\x03\xbc\x05@\xe9C4\x9c_C\xc3W\x80k\xae.\xc4\x1d\x12Ms\xae\xaf\x1f\xe0*\x00J\x1fr8v$I\x01P\xfa\x90\x07\xbf\xd7\xd0p\x13\x00\xa5\x0f\xa9p{\xfd\x00/\x01P\xfa\x90>\x07\xc6\x12\x12\x00\xa5\x0f\x15py\r\r\x0f\x01P\xfaP\x04\x8f\xd7\x0fp\x10\x00\xa5\x0fuD\x7f\rMd\x01P\xfaPJ\xce%\x83\x95\n\x80\xd2\x87r\xd8\xca\"\x9c\x04@\xe9\x83\x02\x91^C\x13E\x00\x94>\x88\x90\x1d\xe1\xf5\x03\xec\x02\xa0\xf4Ai\x0e\x8c\x92.\x00J\x1f\xb4\x18v\x96T\x01P\xfa \x07\xe3\xeb\x07\x98\x04@\xe9\x83$Le\x11\x16\x01P\xfa\xa0\xca\xb9\xb7H\x10\x00\xa5\x0f\xc2\x84?Q\x9e@\xe9\xc3,\xc2\x96E\x12(}\x98\xc6\x90[\x85\t\x80\xd2\x87\x16\x84*\x8b$P\xfa0q\x0e\x8c\x15 \x00J\x1f:\x11\xf8Dy\x02\xa5\x0f3\xe9\x1f\xf0\xc9\"\t\x94>L%XY$\x81\xd2\x87\xb1\x04:Q\x9e@\xe9\xc3`\x02\x94E\x12(}\x18M\xc6\xb2H\"c\xe9#\x07WQg2\x9d(O\xa0\xf4a\xfc\x1c\x18\xc5,\x00J\x1ff\x90\xae,\x92@\xe9\xc3\x869\xd0tY$\x81\xd2\x87\x154Y\x16I\xa0\xf4a\tM\x9c(O\xa0\xf4a\x0b\xa9\xcb\"\t\x94>\xec!UY$\x81\xd2\x87M4>Q\x9e@\xe9\xc3*\x1a\x95E\x12(}\xd86\x07.O#\x00J\x1f\x16\xd0\xf0\xf5\x03_\xa2\xf4a\x1f\xfd\xeb\x85\x84_\xa0\xf4a#u!ae\x05J\x1fV\x92}\xe9\xfe\xcd\xe1\xca\xff\xa1\xf4a+\x1d~\xfb\xbb/\xbf(=x\xfb\xfb\xa1\xf4a\xe3\xb7\xc1A=\xae\xea\xd7\xef\xbc\x81\xe7\xb7\xc6w?\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\xd0\xe6\xffK\xf6\x18+vfxE\x00\x00\x00\x00IEND\xaeB`\x82";
                fs::write(
                        ::alloc::__export::must_use({
                            ::alloc::fmt::format(format_args!("{0}/icon.png", &base_img))
                        }),
                        icon,
                    )
                    .ok()?;
                let readme = "# AHQ Store Cli\r\n\r\n## config.json\r\n\r\nFollows the schema as presented [here](https://docs.rs/ahqstore_cli_rs/latest/ahqstore_cli_rs/shared/struct.IMetadata.html)\r\n\r\n## images/<app-id>/icon.png\r\n\r\nYour application icon that\'ll be bundled in the app metadata file\r\n\r\n## images/<app-id>/\\*\r\n\r\nPlace any image(s) [upto 10] that will be placed in the app modal in AHQ Store\r\n";
                fs::write("./.ahqstore/README.md", readme).ok()
            })()
                .is_some();
            if !succ {
                ERR.println(&"Failed to populate .ahqstore");
                process::exit(1);
            } else {
                {
                    ::std::io::_print(
                        format_args!(
                            "🇩\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}🇴\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}🇳\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}🇪\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\n",
                        ),
                    );
                };
                {
                    ::std::io::_print(
                        format_args!(
                            "████████████████████████████████████████████████████████████████▀█\n█─█─██▀▄─██▄─▄▄─█▄─▄▄─█▄─█─▄███─▄▄▄─█─▄▄─█▄─▄▄▀█▄─▄█▄─▀█▄─▄█─▄▄▄▄█\n█─▄─██─▀─███─▄▄▄██─▄▄▄██▄─▄████─███▀█─██─██─██─██─███─█▄▀─██─██▄─█\n▀▄▀▄▀▄▄▀▄▄▀▄▄▄▀▀▀▄▄▄▀▀▀▀▄▄▄▀▀▀▀▄▄▄▄▄▀▄▄▄▄▀▄▄▄▄▀▀▄▄▄▀▄▄▄▀▀▄▄▀▄▄▄▄▄▀\n\n",
                        ),
                    );
                };
                INFO.println(
                    &"Do not forget to edit config.json and finder.json\nMore details about all the files is present in README.md",
                );
            }
        }
        pub fn create_dir(force: bool) {
            if let Err(_) = fs::create_dir("./.ahqstore") {
                if force {
                    WARN.println(&"--force detected\nRemoving dir .ahqstore");
                    let succ = (|| {
                        fs::remove_dir_all("./.ahqstore").ok()?;
                        fs::create_dir_all("./.ahqstore").ok()?;
                        Some(())
                    })()
                        .is_some();
                    if succ {
                        INFO.println(
                            &".ahqstore directory created, initializing data...",
                        );
                    } else {
                        ERR.println(&"Failed to create .ahqstore directory");
                        process::exit(1);
                    }
                } else {
                    ERR.println(
                        &"Failed to create .ahqstore directory\nHint: Use --force option to ignore this error",
                    );
                    process::exit(1);
                }
            } else {
                INFO.println(&"Created .ahqstore directory, initializing data...");
            }
        }
    }
    mod help {
        use chalk_rs::Chalk;
        pub fn main_help() -> String {
            let mut chalk = Chalk::new();
            let cli = chalk.blue().bold().string(&"AHQ Store CLI");
            let usage = chalk.green().bold().string(&"Usage:");
            let cmds = chalk.green().bold().string(&"Commands:");
            let help = chalk.cyan().bold().string(&"help");
            let create = chalk.cyan().bold().string(&"create");
            let build = chalk.cyan().bold().string(&"build");
            let upload = chalk.cyan().bold().string(&"build");
            let opt = chalk.yellow().bold().string(&"Options:");
            let force = chalk.cyan().bold().string(&"--force, -f");
            let env = chalk.yellow().bold().string(&"Required ENV:");
            let app_id = chalk.cyan().bold().string(&"APP_ID");
            let gh_token = chalk.cyan().bold().string(&"GH_TOKEN");
            let gh_repo = chalk.cyan().bold().string(&"GITHUB_REPOSITORY");
            let gh_action = chalk.cyan().bold().string(&"GITHUB_ACTION");
            let r_id = chalk.cyan().bold().string(&"RELEASE_ID");
            let optional = chalk.yellow().bold().string(&"(Optional)");
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0}\nShip your apps to the ahq store quickly and efficiently\n\n{1}\n  ahqstore (command) [options]\n{2}\n  {3}\n    Shows the help menu\n  {4}\n    Generates AHQ Store config files required to ship your apps\n    {5}\n      {6} Override Existing contents if .ahqstore dir isn\'t empty\n  {7}\n    Build & Upload ahqstore config file\n    {8}\n    {9} {10} Application Id (required if your config has more than 1 appIds)\n    {11} GitHub Release Id\n    {12} GitHub Token\n    {13} GitHub owner & repo name, eg ahqstore/app\n                      (Equivalent to GITHUB_REPOSITORY variable in GitHub actions)\n    {14} {10} Set this env to anything to specify that it is running in an GitHub Action\n                      (Outputs AHQ_STORE_FILE_URL=<url>)\n  {15}\n    Build the ahqstore config file\n    {8}\n      {9} {10} Application Id (required if your config has more than 1 appIds)\n      {11} GitHub Release Id\n      {12} GitHub Token\n      {13} GitHub owner & repo name, eg ahqstore/app\n                        (Equivalent to GITHUB_REPOSITORY variable in GitHub actions)\n      {14} {10} Set this env to anything to specify that it is running in an GitHub Action\n                        (Outputs AHQ_STORE_FILE_URL=<url>)",
                        cli,
                        usage,
                        cmds,
                        help,
                        create,
                        opt,
                        force,
                        upload,
                        env,
                        app_id,
                        optional,
                        r_id,
                        gh_token,
                        gh_repo,
                        gh_action,
                        build,
                    ),
                )
            })
        }
        pub fn not_found(name: &str) -> String {
            let mut chalk = Chalk::new();
            let cmd = chalk.red().bold().string(&"Command not found:");
            let tip = chalk.green().bold().string(&"Tip:");
            let astore = chalk.cyan().bold().string(&"ahqstore");
            ::alloc::__export::must_use({
                ::alloc::fmt::format(
                    format_args!(
                        "{0} {1}\n\n{2}\n  Write {3} to view the help menu",
                        cmd,
                        name,
                        tip,
                        astore,
                    ),
                )
            })
        }
    }
    pub mod shared {
        use serde::{Deserialize, Serialize};
        use std::collections::HashMap;
        use ahqstore_types::AppRepo;
        pub type Str = String;
        pub type Config<'a> = HashMap<String, IMetadata<'a>>;
        mod file_sorter {
            use serde::{Deserialize, Serialize};
            #[allow(non_snake_case)]
            /// # Self Explanatory
            pub struct FileFinder<'a> {
                #[serde(borrow)]
                pub windowsAmd64Finder: Option<Finder<'a>>,
                #[serde(borrow)]
                pub windowsArm64Finder: Option<Finder<'a>>,
                #[serde(borrow)]
                pub linuxAmd64Finder: Option<Finder<'a>>,
                #[serde(borrow)]
                pub linuxArm64Finder: Option<Finder<'a>>,
                #[serde(borrow)]
                pub linuxArm32Finder: Option<Finder<'a>>,
                #[serde(borrow)]
                pub androidUniversalFinder: Option<Finder<'a>>,
            }
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl<'a> ::core::fmt::Debug for FileFinder<'a> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "windowsAmd64Finder",
                        "windowsArm64Finder",
                        "linuxAmd64Finder",
                        "linuxArm64Finder",
                        "linuxArm32Finder",
                        "androidUniversalFinder",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.windowsAmd64Finder,
                        &self.windowsArm64Finder,
                        &self.linuxAmd64Finder,
                        &self.linuxArm64Finder,
                        &self.linuxArm32Finder,
                        &&self.androidUniversalFinder,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "FileFinder",
                        names,
                        values,
                    )
                }
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
                impl<'a> _serde::Serialize for FileFinder<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "FileFinder",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "windowsAmd64Finder",
                            &self.windowsAmd64Finder,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "windowsArm64Finder",
                            &self.windowsArm64Finder,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "linuxAmd64Finder",
                            &self.linuxAmd64Finder,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "linuxArm64Finder",
                            &self.linuxArm64Finder,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "linuxArm32Finder",
                            &self.linuxArm32Finder,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "androidUniversalFinder",
                            &self.androidUniversalFinder,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
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
                impl<'de: 'a, 'a> _serde::Deserialize<'de> for FileFinder<'a> {
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
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
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
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
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
                                    "windowsAmd64Finder" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    "windowsArm64Finder" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    "linuxAmd64Finder" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    "linuxArm64Finder" => {
                                        _serde::__private::Ok(__Field::__field3)
                                    }
                                    "linuxArm32Finder" => {
                                        _serde::__private::Ok(__Field::__field4)
                                    }
                                    "androidUniversalFinder" => {
                                        _serde::__private::Ok(__Field::__field5)
                                    }
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
                                    b"windowsAmd64Finder" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    b"windowsArm64Finder" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    b"linuxAmd64Finder" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    b"linuxArm64Finder" => {
                                        _serde::__private::Ok(__Field::__field3)
                                    }
                                    b"linuxArm32Finder" => {
                                        _serde::__private::Ok(__Field::__field4)
                                    }
                                    b"androidUniversalFinder" => {
                                        _serde::__private::Ok(__Field::__field5)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
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
                        struct __Visitor<'de: 'a, 'a> {
                            marker: _serde::__private::PhantomData<FileFinder<'a>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de: 'a, 'a> _serde::de::Visitor<'de>
                        for __Visitor<'de, 'a> {
                            type Value = FileFinder<'a>;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct FileFinder",
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
                                    Option<Finder<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct FileFinder with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<Finder<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct FileFinder with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Option<Finder<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct FileFinder with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 = match _serde::de::SeqAccess::next_element::<
                                    Option<Finder<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                3usize,
                                                &"struct FileFinder with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field4 = match _serde::de::SeqAccess::next_element::<
                                    Option<Finder<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                4usize,
                                                &"struct FileFinder with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field5 = match _serde::de::SeqAccess::next_element::<
                                    Option<Finder<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                5usize,
                                                &"struct FileFinder with 6 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(FileFinder {
                                    windowsAmd64Finder: __field0,
                                    windowsArm64Finder: __field1,
                                    linuxAmd64Finder: __field2,
                                    linuxArm64Finder: __field3,
                                    linuxArm32Finder: __field4,
                                    androidUniversalFinder: __field5,
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
                                let mut __field0: _serde::__private::Option<
                                    Option<Finder<'a>>,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Option<Finder<'a>>,
                                > = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<
                                    Option<Finder<'a>>,
                                > = _serde::__private::None;
                                let mut __field3: _serde::__private::Option<
                                    Option<Finder<'a>>,
                                > = _serde::__private::None;
                                let mut __field4: _serde::__private::Option<
                                    Option<Finder<'a>>,
                                > = _serde::__private::None;
                                let mut __field5: _serde::__private::Option<
                                    Option<Finder<'a>>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "windowsAmd64Finder",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<Finder<'a>>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "windowsArm64Finder",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<Finder<'a>>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "linuxAmd64Finder",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<Finder<'a>>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "linuxArm64Finder",
                                                    ),
                                                );
                                            }
                                            __field3 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<Finder<'a>>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private::Option::is_some(&__field4) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "linuxArm32Finder",
                                                    ),
                                                );
                                            }
                                            __field4 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<Finder<'a>>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private::Option::is_some(&__field5) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "androidUniversalFinder",
                                                    ),
                                                );
                                            }
                                            __field5 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<Finder<'a>>,
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
                                        _serde::__private::de::missing_field("windowsAmd64Finder")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("windowsArm64Finder")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("linuxAmd64Finder")?
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("linuxArm64Finder")?
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("linuxArm32Finder")?
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::__private::Some(__field5) => __field5,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field(
                                            "androidUniversalFinder",
                                        )?
                                    }
                                };
                                _serde::__private::Ok(FileFinder {
                                    windowsAmd64Finder: __field0,
                                    windowsArm64Finder: __field1,
                                    linuxAmd64Finder: __field2,
                                    linuxArm64Finder: __field3,
                                    linuxArm32Finder: __field4,
                                    androidUniversalFinder: __field5,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "windowsAmd64Finder",
                            "windowsArm64Finder",
                            "linuxAmd64Finder",
                            "linuxArm64Finder",
                            "linuxArm32Finder",
                            "androidUniversalFinder",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "FileFinder",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<FileFinder<'a>>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl<'a> ::core::default::Default for FileFinder<'a> {
                #[inline]
                fn default() -> FileFinder<'a> {
                    FileFinder {
                        windowsAmd64Finder: ::core::default::Default::default(),
                        windowsArm64Finder: ::core::default::Default::default(),
                        linuxAmd64Finder: ::core::default::Default::default(),
                        linuxArm64Finder: ::core::default::Default::default(),
                        linuxArm32Finder: ::core::default::Default::default(),
                        androidUniversalFinder: ::core::default::Default::default(),
                    }
                }
            }
            #[allow(non_snake_case)]
            pub struct Finder<'a> {
                #[serde(borrow)]
                pub startsWith: Option<&'a str>,
                #[serde(borrow)]
                pub contains: Option<&'a str>,
                #[serde(borrow)]
                pub endsWith: Option<&'a str>,
            }
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl<'a> ::core::fmt::Debug for Finder<'a> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "Finder",
                        "startsWith",
                        &self.startsWith,
                        "contains",
                        &self.contains,
                        "endsWith",
                        &&self.endsWith,
                    )
                }
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
                impl<'a> _serde::Serialize for Finder<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "Finder",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "startsWith",
                            &self.startsWith,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "contains",
                            &self.contains,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "endsWith",
                            &self.endsWith,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
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
                impl<'de: 'a, 'a> _serde::Deserialize<'de> for Finder<'a> {
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
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
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
                                    2u64 => _serde::__private::Ok(__Field::__field2),
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
                                    "startsWith" => _serde::__private::Ok(__Field::__field0),
                                    "contains" => _serde::__private::Ok(__Field::__field1),
                                    "endsWith" => _serde::__private::Ok(__Field::__field2),
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
                                    b"startsWith" => _serde::__private::Ok(__Field::__field0),
                                    b"contains" => _serde::__private::Ok(__Field::__field1),
                                    b"endsWith" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
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
                        struct __Visitor<'de: 'a, 'a> {
                            marker: _serde::__private::PhantomData<Finder<'a>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de: 'a, 'a> _serde::de::Visitor<'de>
                        for __Visitor<'de, 'a> {
                            type Value = Finder<'a>;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct Finder",
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
                                    Option<&'a str>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct Finder with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<&'a str>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct Finder with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Option<&'a str>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct Finder with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(Finder {
                                    startsWith: __field0,
                                    contains: __field1,
                                    endsWith: __field2,
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
                                let mut __field0: _serde::__private::Option<
                                    Option<&'a str>,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Option<&'a str>,
                                > = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<
                                    Option<&'a str>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "startsWith",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<&'a str>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "contains",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<&'a str>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "endsWith",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<&'a str>,
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
                                        _serde::__private::de::missing_field("startsWith")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("contains")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("endsWith")?
                                    }
                                };
                                _serde::__private::Ok(Finder {
                                    startsWith: __field0,
                                    contains: __field1,
                                    endsWith: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "startsWith",
                            "contains",
                            "endsWith",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "Finder",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<Finder<'a>>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl<'a> ::core::default::Default for Finder<'a> {
                #[inline]
                fn default() -> Finder<'a> {
                    Finder {
                        startsWith: ::core::default::Default::default(),
                        contains: ::core::default::Default::default(),
                        endsWith: ::core::default::Default::default(),
                    }
                }
            }
            impl<'a> FileFinder<'a> {
                pub fn new() -> Self {
                    Self {
                        windowsAmd64Finder: Some(Finder {
                            startsWith: Some("This-is"),
                            contains: Some("an"),
                            endsWith: Some(".example"),
                        }),
                        ..Default::default()
                    }
                }
            }
        }
        mod platforms {
            use ahqstore_types::{InstallerFormat, WindowsInstallScope, AndroidAbi};
            use serde::{Deserialize, Serialize};
            #[allow(non_snake_case)]
            /// # Format to be used in
            /// - winAmd64Platform: Windows X64
            /// - winArm64Platform : Windows Arm64
            /// - linuxAmd64Platform: Linux X64
            /// - linuxArm64Platform: Linux Arm64
            /// - linuxArm32Platform: Linux Arm32
            /// - androidUniversal: Android
            ///
            /// ## is as follows:
            /// (any value from the list)
            /// - "WindowsZip"
            /// - "WindowsInstallerMsi"
            /// - "WindowsInstallerExe"
            /// - "WindowsUWPMsix"
            /// - "LinuxAppImage"
            /// - "AndroidApkZip"
            pub struct IPlatform<'a> {
                pub winAmd64Platform: Option<InstallerFormat>,
                pub winArm64Platform: Option<InstallerFormat>,
                pub linuxAmd64Platform: Option<InstallerFormat>,
                pub linuxArm64Platform: Option<InstallerFormat>,
                pub linuxArm32Platform: Option<InstallerFormat>,
                pub androidUniversal: Option<InstallerFormat>,
                /// Click on IOAndroid for documentation
                pub androidOptions: Option<IOAndroid>,
                #[serde(borrow)]
                /// Click on IOWin<'a> for documentation
                pub winAmd64Options: Option<IOWin<'a>>,
                #[serde(borrow)]
                /// Click on IOWin<'a> for documentation
                pub winArm64Options: Option<IOWin<'a>>,
            }
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl<'a> ::core::fmt::Debug for IPlatform<'a> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let names: &'static _ = &[
                        "winAmd64Platform",
                        "winArm64Platform",
                        "linuxAmd64Platform",
                        "linuxArm64Platform",
                        "linuxArm32Platform",
                        "androidUniversal",
                        "androidOptions",
                        "winAmd64Options",
                        "winArm64Options",
                    ];
                    let values: &[&dyn ::core::fmt::Debug] = &[
                        &self.winAmd64Platform,
                        &self.winArm64Platform,
                        &self.linuxAmd64Platform,
                        &self.linuxArm64Platform,
                        &self.linuxArm32Platform,
                        &self.androidUniversal,
                        &self.androidOptions,
                        &self.winAmd64Options,
                        &&self.winArm64Options,
                    ];
                    ::core::fmt::Formatter::debug_struct_fields_finish(
                        f,
                        "IPlatform",
                        names,
                        values,
                    )
                }
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
                impl<'a> _serde::Serialize for IPlatform<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "IPlatform",
                            false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "winAmd64Platform",
                            &self.winAmd64Platform,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "winArm64Platform",
                            &self.winArm64Platform,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "linuxAmd64Platform",
                            &self.linuxAmd64Platform,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "linuxArm64Platform",
                            &self.linuxArm64Platform,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "linuxArm32Platform",
                            &self.linuxArm32Platform,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "androidUniversal",
                            &self.androidUniversal,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "androidOptions",
                            &self.androidOptions,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "winAmd64Options",
                            &self.winAmd64Options,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "winArm64Options",
                            &self.winArm64Options,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
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
                impl<'de: 'a, 'a> _serde::Deserialize<'de> for IPlatform<'a> {
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
                            __field2,
                            __field3,
                            __field4,
                            __field5,
                            __field6,
                            __field7,
                            __field8,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
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
                                    2u64 => _serde::__private::Ok(__Field::__field2),
                                    3u64 => _serde::__private::Ok(__Field::__field3),
                                    4u64 => _serde::__private::Ok(__Field::__field4),
                                    5u64 => _serde::__private::Ok(__Field::__field5),
                                    6u64 => _serde::__private::Ok(__Field::__field6),
                                    7u64 => _serde::__private::Ok(__Field::__field7),
                                    8u64 => _serde::__private::Ok(__Field::__field8),
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
                                    "winAmd64Platform" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    "winArm64Platform" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    "linuxAmd64Platform" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    "linuxArm64Platform" => {
                                        _serde::__private::Ok(__Field::__field3)
                                    }
                                    "linuxArm32Platform" => {
                                        _serde::__private::Ok(__Field::__field4)
                                    }
                                    "androidUniversal" => {
                                        _serde::__private::Ok(__Field::__field5)
                                    }
                                    "androidOptions" => _serde::__private::Ok(__Field::__field6),
                                    "winAmd64Options" => {
                                        _serde::__private::Ok(__Field::__field7)
                                    }
                                    "winArm64Options" => {
                                        _serde::__private::Ok(__Field::__field8)
                                    }
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
                                    b"winAmd64Platform" => {
                                        _serde::__private::Ok(__Field::__field0)
                                    }
                                    b"winArm64Platform" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    b"linuxAmd64Platform" => {
                                        _serde::__private::Ok(__Field::__field2)
                                    }
                                    b"linuxArm64Platform" => {
                                        _serde::__private::Ok(__Field::__field3)
                                    }
                                    b"linuxArm32Platform" => {
                                        _serde::__private::Ok(__Field::__field4)
                                    }
                                    b"androidUniversal" => {
                                        _serde::__private::Ok(__Field::__field5)
                                    }
                                    b"androidOptions" => {
                                        _serde::__private::Ok(__Field::__field6)
                                    }
                                    b"winAmd64Options" => {
                                        _serde::__private::Ok(__Field::__field7)
                                    }
                                    b"winArm64Options" => {
                                        _serde::__private::Ok(__Field::__field8)
                                    }
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
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
                        struct __Visitor<'de: 'a, 'a> {
                            marker: _serde::__private::PhantomData<IPlatform<'a>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de: 'a, 'a> _serde::de::Visitor<'de>
                        for __Visitor<'de, 'a> {
                            type Value = IPlatform<'a>;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct IPlatform",
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
                                    Option<InstallerFormat>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<InstallerFormat>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Option<InstallerFormat>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field3 = match _serde::de::SeqAccess::next_element::<
                                    Option<InstallerFormat>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                3usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field4 = match _serde::de::SeqAccess::next_element::<
                                    Option<InstallerFormat>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                4usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field5 = match _serde::de::SeqAccess::next_element::<
                                    Option<InstallerFormat>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                5usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field6 = match _serde::de::SeqAccess::next_element::<
                                    Option<IOAndroid>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                6usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field7 = match _serde::de::SeqAccess::next_element::<
                                    Option<IOWin<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                7usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field8 = match _serde::de::SeqAccess::next_element::<
                                    Option<IOWin<'a>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                8usize,
                                                &"struct IPlatform with 9 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(IPlatform {
                                    winAmd64Platform: __field0,
                                    winArm64Platform: __field1,
                                    linuxAmd64Platform: __field2,
                                    linuxArm64Platform: __field3,
                                    linuxArm32Platform: __field4,
                                    androidUniversal: __field5,
                                    androidOptions: __field6,
                                    winAmd64Options: __field7,
                                    winArm64Options: __field8,
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
                                let mut __field0: _serde::__private::Option<
                                    Option<InstallerFormat>,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Option<InstallerFormat>,
                                > = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<
                                    Option<InstallerFormat>,
                                > = _serde::__private::None;
                                let mut __field3: _serde::__private::Option<
                                    Option<InstallerFormat>,
                                > = _serde::__private::None;
                                let mut __field4: _serde::__private::Option<
                                    Option<InstallerFormat>,
                                > = _serde::__private::None;
                                let mut __field5: _serde::__private::Option<
                                    Option<InstallerFormat>,
                                > = _serde::__private::None;
                                let mut __field6: _serde::__private::Option<
                                    Option<IOAndroid>,
                                > = _serde::__private::None;
                                let mut __field7: _serde::__private::Option<
                                    Option<IOWin<'a>>,
                                > = _serde::__private::None;
                                let mut __field8: _serde::__private::Option<
                                    Option<IOWin<'a>>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "winAmd64Platform",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<InstallerFormat>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "winArm64Platform",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<InstallerFormat>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "linuxAmd64Platform",
                                                    ),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<InstallerFormat>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field3 => {
                                            if _serde::__private::Option::is_some(&__field3) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "linuxArm64Platform",
                                                    ),
                                                );
                                            }
                                            __field3 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<InstallerFormat>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field4 => {
                                            if _serde::__private::Option::is_some(&__field4) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "linuxArm32Platform",
                                                    ),
                                                );
                                            }
                                            __field4 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<InstallerFormat>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field5 => {
                                            if _serde::__private::Option::is_some(&__field5) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "androidUniversal",
                                                    ),
                                                );
                                            }
                                            __field5 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<InstallerFormat>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field6 => {
                                            if _serde::__private::Option::is_some(&__field6) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "androidOptions",
                                                    ),
                                                );
                                            }
                                            __field6 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<IOAndroid>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field7 => {
                                            if _serde::__private::Option::is_some(&__field7) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "winAmd64Options",
                                                    ),
                                                );
                                            }
                                            __field7 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<IOWin<'a>>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field8 => {
                                            if _serde::__private::Option::is_some(&__field8) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "winArm64Options",
                                                    ),
                                                );
                                            }
                                            __field8 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<IOWin<'a>>,
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
                                        _serde::__private::de::missing_field("winAmd64Platform")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("winArm64Platform")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("linuxAmd64Platform")?
                                    }
                                };
                                let __field3 = match __field3 {
                                    _serde::__private::Some(__field3) => __field3,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("linuxArm64Platform")?
                                    }
                                };
                                let __field4 = match __field4 {
                                    _serde::__private::Some(__field4) => __field4,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("linuxArm32Platform")?
                                    }
                                };
                                let __field5 = match __field5 {
                                    _serde::__private::Some(__field5) => __field5,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("androidUniversal")?
                                    }
                                };
                                let __field6 = match __field6 {
                                    _serde::__private::Some(__field6) => __field6,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("androidOptions")?
                                    }
                                };
                                let __field7 = match __field7 {
                                    _serde::__private::Some(__field7) => __field7,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("winAmd64Options")?
                                    }
                                };
                                let __field8 = match __field8 {
                                    _serde::__private::Some(__field8) => __field8,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("winArm64Options")?
                                    }
                                };
                                _serde::__private::Ok(IPlatform {
                                    winAmd64Platform: __field0,
                                    winArm64Platform: __field1,
                                    linuxAmd64Platform: __field2,
                                    linuxArm64Platform: __field3,
                                    linuxArm32Platform: __field4,
                                    androidUniversal: __field5,
                                    androidOptions: __field6,
                                    winAmd64Options: __field7,
                                    winArm64Options: __field8,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "winAmd64Platform",
                            "winArm64Platform",
                            "linuxAmd64Platform",
                            "linuxArm64Platform",
                            "linuxArm32Platform",
                            "androidUniversal",
                            "androidOptions",
                            "winAmd64Options",
                            "winArm64Options",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "IPlatform",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<IPlatform<'a>>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[allow(non_snake_case)]
            /// Android Information Specification
            /// # minSdk
            /// - Must be a valid Android SDK
            ///
            /// # abi
            /// An array of the following values
            /// - "Aarch64" **Arm64 Phone**
            /// - "Armv7" **Arm32 Phone**
            /// - "X86" **Intel/AMD 32bit Phone**
            /// - "X64" **Intel/AMD 64bit Phone**
            pub struct IOAndroid {
                pub minSdk: u32,
                pub abi: Vec<AndroidAbi>,
            }
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl ::core::fmt::Debug for IOAndroid {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field2_finish(
                        f,
                        "IOAndroid",
                        "minSdk",
                        &self.minSdk,
                        "abi",
                        &&self.abi,
                    )
                }
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
                impl _serde::Serialize for IOAndroid {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "IOAndroid",
                            false as usize + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "minSdk",
                            &self.minSdk,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "abi",
                            &self.abi,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
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
                impl<'de> _serde::Deserialize<'de> for IOAndroid {
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
                        #[automatically_derived]
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
                                    "minSdk" => _serde::__private::Ok(__Field::__field0),
                                    "abi" => _serde::__private::Ok(__Field::__field1),
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
                                    b"minSdk" => _serde::__private::Ok(__Field::__field0),
                                    b"abi" => _serde::__private::Ok(__Field::__field1),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
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
                            marker: _serde::__private::PhantomData<IOAndroid>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                            type Value = IOAndroid;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct IOAndroid",
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
                                    u32,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct IOAndroid with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Vec<AndroidAbi>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct IOAndroid with 2 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(IOAndroid {
                                    minSdk: __field0,
                                    abi: __field1,
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
                                let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Vec<AndroidAbi>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("minSdk"),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<u32>(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("abi"),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Vec<AndroidAbi>,
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
                                        _serde::__private::de::missing_field("minSdk")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("abi")?
                                    }
                                };
                                _serde::__private::Ok(IOAndroid {
                                    minSdk: __field0,
                                    abi: __field1,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &["minSdk", "abi"];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "IOAndroid",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<IOAndroid>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            #[allow(non_snake_case)]
            impl ::core::clone::Clone for IOAndroid {
                #[inline]
                fn clone(&self) -> IOAndroid {
                    IOAndroid {
                        minSdk: ::core::clone::Clone::clone(&self.minSdk),
                        abi: ::core::clone::Clone::clone(&self.abi),
                    }
                }
            }
            /// # zip_file_exec
            /// - Executable to be linked to the Start Menu
            /// - For **WindowsZip** only
            ///
            /// # exe_installer_args
            /// - An array of args passed to the exe installer
            /// - For **WindowsInstallerExe** only
            ///
            /// **The array is internally joined with " "**
            ///
            /// # scope
            /// - Scope of the Windows Installer
            /// - For **WindowsInstallerExe** or **WindowsZip** apps only
            /// - Is required for **WindowsInstallerExe**
            /// - For **WindowsZip**, keeping it empty means that it can be installed both as user or system application
            ///
            /// One of the two values:
            /// - "User"
            /// - "Machine"
            pub struct IOWin<'a> {
                #[serde(borrow)]
                pub zip_file_exec: Option<&'a str>,
                #[serde(borrow)]
                pub exe_installer_args: Option<Vec<&'a str>>,
                pub scope: Option<WindowsInstallScope>,
            }
            #[automatically_derived]
            impl<'a> ::core::fmt::Debug for IOWin<'a> {
                #[inline]
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    ::core::fmt::Formatter::debug_struct_field3_finish(
                        f,
                        "IOWin",
                        "zip_file_exec",
                        &self.zip_file_exec,
                        "exe_installer_args",
                        &self.exe_installer_args,
                        "scope",
                        &&self.scope,
                    )
                }
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
                impl<'a> _serde::Serialize for IOWin<'a> {
                    fn serialize<__S>(
                        &self,
                        __serializer: __S,
                    ) -> _serde::__private::Result<__S::Ok, __S::Error>
                    where
                        __S: _serde::Serializer,
                    {
                        let mut __serde_state = _serde::Serializer::serialize_struct(
                            __serializer,
                            "IOWin",
                            false as usize + 1 + 1 + 1,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "zip_file_exec",
                            &self.zip_file_exec,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "exe_installer_args",
                            &self.exe_installer_args,
                        )?;
                        _serde::ser::SerializeStruct::serialize_field(
                            &mut __serde_state,
                            "scope",
                            &self.scope,
                        )?;
                        _serde::ser::SerializeStruct::end(__serde_state)
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
                impl<'de: 'a, 'a> _serde::Deserialize<'de> for IOWin<'a> {
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
                            __field2,
                            __ignore,
                        }
                        #[doc(hidden)]
                        struct __FieldVisitor;
                        #[automatically_derived]
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
                                    2u64 => _serde::__private::Ok(__Field::__field2),
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
                                    "zip_file_exec" => _serde::__private::Ok(__Field::__field0),
                                    "exe_installer_args" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    "scope" => _serde::__private::Ok(__Field::__field2),
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
                                    b"zip_file_exec" => _serde::__private::Ok(__Field::__field0),
                                    b"exe_installer_args" => {
                                        _serde::__private::Ok(__Field::__field1)
                                    }
                                    b"scope" => _serde::__private::Ok(__Field::__field2),
                                    _ => _serde::__private::Ok(__Field::__ignore),
                                }
                            }
                        }
                        #[automatically_derived]
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
                        struct __Visitor<'de: 'a, 'a> {
                            marker: _serde::__private::PhantomData<IOWin<'a>>,
                            lifetime: _serde::__private::PhantomData<&'de ()>,
                        }
                        #[automatically_derived]
                        impl<'de: 'a, 'a> _serde::de::Visitor<'de>
                        for __Visitor<'de, 'a> {
                            type Value = IOWin<'a>;
                            fn expecting(
                                &self,
                                __formatter: &mut _serde::__private::Formatter,
                            ) -> _serde::__private::fmt::Result {
                                _serde::__private::Formatter::write_str(
                                    __formatter,
                                    "struct IOWin",
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
                                    Option<&'a str>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                0usize,
                                                &"struct IOWin with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field1 = match _serde::de::SeqAccess::next_element::<
                                    Option<Vec<&'a str>>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                1usize,
                                                &"struct IOWin with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                let __field2 = match _serde::de::SeqAccess::next_element::<
                                    Option<WindowsInstallScope>,
                                >(&mut __seq)? {
                                    _serde::__private::Some(__value) => __value,
                                    _serde::__private::None => {
                                        return _serde::__private::Err(
                                            _serde::de::Error::invalid_length(
                                                2usize,
                                                &"struct IOWin with 3 elements",
                                            ),
                                        );
                                    }
                                };
                                _serde::__private::Ok(IOWin {
                                    zip_file_exec: __field0,
                                    exe_installer_args: __field1,
                                    scope: __field2,
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
                                let mut __field0: _serde::__private::Option<
                                    Option<&'a str>,
                                > = _serde::__private::None;
                                let mut __field1: _serde::__private::Option<
                                    Option<Vec<&'a str>>,
                                > = _serde::__private::None;
                                let mut __field2: _serde::__private::Option<
                                    Option<WindowsInstallScope>,
                                > = _serde::__private::None;
                                while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                    __Field,
                                >(&mut __map)? {
                                    match __key {
                                        __Field::__field0 => {
                                            if _serde::__private::Option::is_some(&__field0) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "zip_file_exec",
                                                    ),
                                                );
                                            }
                                            __field0 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<&'a str>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field1 => {
                                            if _serde::__private::Option::is_some(&__field1) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field(
                                                        "exe_installer_args",
                                                    ),
                                                );
                                            }
                                            __field1 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<Vec<&'a str>>,
                                                >(&mut __map)?,
                                            );
                                        }
                                        __Field::__field2 => {
                                            if _serde::__private::Option::is_some(&__field2) {
                                                return _serde::__private::Err(
                                                    <__A::Error as _serde::de::Error>::duplicate_field("scope"),
                                                );
                                            }
                                            __field2 = _serde::__private::Some(
                                                _serde::de::MapAccess::next_value::<
                                                    Option<WindowsInstallScope>,
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
                                        _serde::__private::de::missing_field("zip_file_exec")?
                                    }
                                };
                                let __field1 = match __field1 {
                                    _serde::__private::Some(__field1) => __field1,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("exe_installer_args")?
                                    }
                                };
                                let __field2 = match __field2 {
                                    _serde::__private::Some(__field2) => __field2,
                                    _serde::__private::None => {
                                        _serde::__private::de::missing_field("scope")?
                                    }
                                };
                                _serde::__private::Ok(IOWin {
                                    zip_file_exec: __field0,
                                    exe_installer_args: __field1,
                                    scope: __field2,
                                })
                            }
                        }
                        #[doc(hidden)]
                        const FIELDS: &'static [&'static str] = &[
                            "zip_file_exec",
                            "exe_installer_args",
                            "scope",
                        ];
                        _serde::Deserializer::deserialize_struct(
                            __deserializer,
                            "IOWin",
                            FIELDS,
                            __Visitor {
                                marker: _serde::__private::PhantomData::<IOWin<'a>>,
                                lifetime: _serde::__private::PhantomData,
                            },
                        )
                    }
                }
            };
            #[automatically_derived]
            impl<'a> ::core::clone::Clone for IOWin<'a> {
                #[inline]
                fn clone(&self) -> IOWin<'a> {
                    IOWin {
                        zip_file_exec: ::core::clone::Clone::clone(&self.zip_file_exec),
                        exe_installer_args: ::core::clone::Clone::clone(
                            &self.exe_installer_args,
                        ),
                        scope: ::core::clone::Clone::clone(&self.scope),
                    }
                }
            }
            impl<'a> IPlatform<'a> {
                pub fn new() -> Self {
                    let io_win = IOWin {
                        exe_installer_args: Some(::alloc::vec::Vec::new()),
                        zip_file_exec: None,
                        scope: None,
                    };
                    let io_android = IOAndroid {
                        minSdk: 29,
                        abi: <[_]>::into_vec(
                            ::alloc::boxed::box_new([
                                AndroidAbi::Aarch64,
                                AndroidAbi::Armv7,
                                AndroidAbi::X86,
                                AndroidAbi::X64,
                            ]),
                        ),
                    };
                    Self {
                        winAmd64Platform: None,
                        winArm64Platform: None,
                        linuxAmd64Platform: None,
                        linuxArm32Platform: None,
                        linuxArm64Platform: None,
                        androidUniversal: None,
                        androidOptions: Some(io_android),
                        winAmd64Options: Some(io_win.clone()),
                        winArm64Options: Some(io_win),
                    }
                }
            }
        }
        pub use file_sorter::*;
        pub use platforms::*;
        #[allow(non_snake_case)]
        /// # MUST EDIT FIELDS
        /// - platform
        /// - finder
        ///
        /// ## MAY EDIT
        /// - site
        /// - license_or_tos
        pub struct IMetadata<'a> {
            #[doc(hidden)]
            #[serde(rename = "$schema")]
            pub schema: String,
            /// Application ID: **Auto set by the cli**
            pub appId: Str,
            /// Application Name (as it appears in start menu): **Auto set by the cli**
            pub appShortcutName: Str,
            /// Application Display Name (as it appears in app): **Auto set by the cli**
            pub appDisplayName: Str,
            /// Author ID: **Auto set by the cli**
            pub authorId: Str,
            /// Application Description: **Auto set by the cli**
            pub description: Str,
            /// Application Repository Information: **Auto set by the cli**
            pub repo: AppRepo,
            #[serde[borrow]]
            /// Platform Information **MUST EDIT**
            pub platform: IPlatform<'a>,
            #[serde[borrow]]
            /// Binary Finder Information **MUST EDIT**
            pub finder: FileFinder<'a>,
            /// Your Application Site: **MAY EDIT**
            pub site: Option<Str>,
            /// DO NOT TOUCH THIS
            /// THIS IS FOR INTERNAL USAGE
            pub redistributed: Option<Str>,
            /// Specify your license or preferably a url to the app TOS & LICENSE
            pub license_or_tos: Option<Str>,
        }
        #[automatically_derived]
        #[allow(non_snake_case)]
        impl<'a> ::core::fmt::Debug for IMetadata<'a> {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "schema",
                    "appId",
                    "appShortcutName",
                    "appDisplayName",
                    "authorId",
                    "description",
                    "repo",
                    "platform",
                    "finder",
                    "site",
                    "redistributed",
                    "license_or_tos",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &self.schema,
                    &self.appId,
                    &self.appShortcutName,
                    &self.appDisplayName,
                    &self.authorId,
                    &self.description,
                    &self.repo,
                    &self.platform,
                    &self.finder,
                    &self.site,
                    &self.redistributed,
                    &&self.license_or_tos,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "IMetadata",
                    names,
                    values,
                )
            }
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
            impl<'a> _serde::Serialize for IMetadata<'a> {
                fn serialize<__S>(
                    &self,
                    __serializer: __S,
                ) -> _serde::__private::Result<__S::Ok, __S::Error>
                where
                    __S: _serde::Serializer,
                {
                    let mut __serde_state = _serde::Serializer::serialize_struct(
                        __serializer,
                        "IMetadata",
                        false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "$schema",
                        &self.schema,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "appId",
                        &self.appId,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "appShortcutName",
                        &self.appShortcutName,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "appDisplayName",
                        &self.appDisplayName,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "authorId",
                        &self.authorId,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "description",
                        &self.description,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "repo",
                        &self.repo,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "platform",
                        &self.platform,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "finder",
                        &self.finder,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "site",
                        &self.site,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "redistributed",
                        &self.redistributed,
                    )?;
                    _serde::ser::SerializeStruct::serialize_field(
                        &mut __serde_state,
                        "license_or_tos",
                        &self.license_or_tos,
                    )?;
                    _serde::ser::SerializeStruct::end(__serde_state)
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
            impl<'de: 'a, 'a> _serde::Deserialize<'de> for IMetadata<'a> {
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
                        __field2,
                        __field3,
                        __field4,
                        __field5,
                        __field6,
                        __field7,
                        __field8,
                        __field9,
                        __field10,
                        __field11,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
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
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                4u64 => _serde::__private::Ok(__Field::__field4),
                                5u64 => _serde::__private::Ok(__Field::__field5),
                                6u64 => _serde::__private::Ok(__Field::__field6),
                                7u64 => _serde::__private::Ok(__Field::__field7),
                                8u64 => _serde::__private::Ok(__Field::__field8),
                                9u64 => _serde::__private::Ok(__Field::__field9),
                                10u64 => _serde::__private::Ok(__Field::__field10),
                                11u64 => _serde::__private::Ok(__Field::__field11),
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
                                "$schema" => _serde::__private::Ok(__Field::__field0),
                                "appId" => _serde::__private::Ok(__Field::__field1),
                                "appShortcutName" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                "appDisplayName" => _serde::__private::Ok(__Field::__field3),
                                "authorId" => _serde::__private::Ok(__Field::__field4),
                                "description" => _serde::__private::Ok(__Field::__field5),
                                "repo" => _serde::__private::Ok(__Field::__field6),
                                "platform" => _serde::__private::Ok(__Field::__field7),
                                "finder" => _serde::__private::Ok(__Field::__field8),
                                "site" => _serde::__private::Ok(__Field::__field9),
                                "redistributed" => _serde::__private::Ok(__Field::__field10),
                                "license_or_tos" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
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
                                b"$schema" => _serde::__private::Ok(__Field::__field0),
                                b"appId" => _serde::__private::Ok(__Field::__field1),
                                b"appShortcutName" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                b"appDisplayName" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                b"authorId" => _serde::__private::Ok(__Field::__field4),
                                b"description" => _serde::__private::Ok(__Field::__field5),
                                b"repo" => _serde::__private::Ok(__Field::__field6),
                                b"platform" => _serde::__private::Ok(__Field::__field7),
                                b"finder" => _serde::__private::Ok(__Field::__field8),
                                b"site" => _serde::__private::Ok(__Field::__field9),
                                b"redistributed" => {
                                    _serde::__private::Ok(__Field::__field10)
                                }
                                b"license_or_tos" => {
                                    _serde::__private::Ok(__Field::__field11)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
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
                    struct __Visitor<'de: 'a, 'a> {
                        marker: _serde::__private::PhantomData<IMetadata<'a>>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de: 'a, 'a> _serde::de::Visitor<'de> for __Visitor<'de, 'a> {
                        type Value = IMetadata<'a>;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct IMetadata",
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
                                String,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                Str,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                Str,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                Str,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field4 = match _serde::de::SeqAccess::next_element::<
                                Str,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            4usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field5 = match _serde::de::SeqAccess::next_element::<
                                Str,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            5usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field6 = match _serde::de::SeqAccess::next_element::<
                                AppRepo,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            6usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field7 = match _serde::de::SeqAccess::next_element::<
                                IPlatform<'a>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            7usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field8 = match _serde::de::SeqAccess::next_element::<
                                FileFinder<'a>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            8usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field9 = match _serde::de::SeqAccess::next_element::<
                                Option<Str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            9usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field10 = match _serde::de::SeqAccess::next_element::<
                                Option<Str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            10usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            let __field11 = match _serde::de::SeqAccess::next_element::<
                                Option<Str>,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            11usize,
                                            &"struct IMetadata with 12 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(IMetadata {
                                schema: __field0,
                                appId: __field1,
                                appShortcutName: __field2,
                                appDisplayName: __field3,
                                authorId: __field4,
                                description: __field5,
                                repo: __field6,
                                platform: __field7,
                                finder: __field8,
                                site: __field9,
                                redistributed: __field10,
                                license_or_tos: __field11,
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
                            let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<Str> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<Str> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<Str> = _serde::__private::None;
                            let mut __field4: _serde::__private::Option<Str> = _serde::__private::None;
                            let mut __field5: _serde::__private::Option<Str> = _serde::__private::None;
                            let mut __field6: _serde::__private::Option<AppRepo> = _serde::__private::None;
                            let mut __field7: _serde::__private::Option<IPlatform<'a>> = _serde::__private::None;
                            let mut __field8: _serde::__private::Option<
                                FileFinder<'a>,
                            > = _serde::__private::None;
                            let mut __field9: _serde::__private::Option<Option<Str>> = _serde::__private::None;
                            let mut __field10: _serde::__private::Option<Option<Str>> = _serde::__private::None;
                            let mut __field11: _serde::__private::Option<Option<Str>> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "$schema",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("appId"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "appShortcutName",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "appDisplayName",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field4 => {
                                        if _serde::__private::Option::is_some(&__field4) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "authorId",
                                                ),
                                            );
                                        }
                                        __field4 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field5 => {
                                        if _serde::__private::Option::is_some(&__field5) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "description",
                                                ),
                                            );
                                        }
                                        __field5 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<Str>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field6 => {
                                        if _serde::__private::Option::is_some(&__field6) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("repo"),
                                            );
                                        }
                                        __field6 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<AppRepo>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field7 => {
                                        if _serde::__private::Option::is_some(&__field7) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "platform",
                                                ),
                                            );
                                        }
                                        __field7 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                IPlatform<'a>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field8 => {
                                        if _serde::__private::Option::is_some(&__field8) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("finder"),
                                            );
                                        }
                                        __field8 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                FileFinder<'a>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field9 => {
                                        if _serde::__private::Option::is_some(&__field9) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("site"),
                                            );
                                        }
                                        __field9 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Str>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field10 => {
                                        if _serde::__private::Option::is_some(&__field10) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "redistributed",
                                                ),
                                            );
                                        }
                                        __field10 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Str>,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field11 => {
                                        if _serde::__private::Option::is_some(&__field11) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "license_or_tos",
                                                ),
                                            );
                                        }
                                        __field11 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                Option<Str>,
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
                                    _serde::__private::de::missing_field("$schema")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("appId")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("appShortcutName")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("appDisplayName")?
                                }
                            };
                            let __field4 = match __field4 {
                                _serde::__private::Some(__field4) => __field4,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("authorId")?
                                }
                            };
                            let __field5 = match __field5 {
                                _serde::__private::Some(__field5) => __field5,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("description")?
                                }
                            };
                            let __field6 = match __field6 {
                                _serde::__private::Some(__field6) => __field6,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("repo")?
                                }
                            };
                            let __field7 = match __field7 {
                                _serde::__private::Some(__field7) => __field7,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("platform")?
                                }
                            };
                            let __field8 = match __field8 {
                                _serde::__private::Some(__field8) => __field8,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("finder")?
                                }
                            };
                            let __field9 = match __field9 {
                                _serde::__private::Some(__field9) => __field9,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("site")?
                                }
                            };
                            let __field10 = match __field10 {
                                _serde::__private::Some(__field10) => __field10,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("redistributed")?
                                }
                            };
                            let __field11 = match __field11 {
                                _serde::__private::Some(__field11) => __field11,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("license_or_tos")?
                                }
                            };
                            _serde::__private::Ok(IMetadata {
                                schema: __field0,
                                appId: __field1,
                                appShortcutName: __field2,
                                appDisplayName: __field3,
                                authorId: __field4,
                                description: __field5,
                                repo: __field6,
                                platform: __field7,
                                finder: __field8,
                                site: __field9,
                                redistributed: __field10,
                                license_or_tos: __field11,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "$schema",
                        "appId",
                        "appShortcutName",
                        "appDisplayName",
                        "authorId",
                        "description",
                        "repo",
                        "platform",
                        "finder",
                        "site",
                        "redistributed",
                        "license_or_tos",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "IMetadata",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<IMetadata<'a>>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        impl<'a> IMetadata<'a> {
            #[allow(non_snake_case)]
            pub fn new(
                appId: Str,
                appShortcutName: Str,
                appDisplayName: Str,
                authorId: Str,
                description: Str,
                repo: AppRepo,
                platform: IPlatform<'a>,
            ) -> Config<'a> {
                let mut config = Config::new();
                config
                    .insert(
                        appId.clone(),
                        Self {
                            schema: ::alloc::__export::must_use({
                                ::alloc::fmt::format(format_args!("./spec.schema.json"))
                            }),
                            appId,
                            appShortcutName,
                            appDisplayName,
                            authorId,
                            description,
                            repo,
                            platform,
                            finder: FileFinder::new(),
                            site: None,
                            redistributed: None,
                            license_or_tos: None,
                        },
                    );
                config
            }
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct INFO {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    static INFO: INFO = INFO { __private_field: () };
    impl ::lazy_static::__Deref for INFO {
        type Target = Chalk;
        fn deref(&self) -> &Chalk {
            #[inline(always)]
            fn __static_ref_initialize() -> Chalk {
                {
                    let mut chalk = Chalk::new();
                    chalk.blue().bold();
                    chalk
                }
            }
            #[inline(always)]
            fn __stability() -> &'static Chalk {
                static LAZY: ::lazy_static::lazy::Lazy<Chalk> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for INFO {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct WARN {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    static WARN: WARN = WARN { __private_field: () };
    impl ::lazy_static::__Deref for WARN {
        type Target = Chalk;
        fn deref(&self) -> &Chalk {
            #[inline(always)]
            fn __static_ref_initialize() -> Chalk {
                {
                    let mut chalk = Chalk::new();
                    chalk.yellow().bold();
                    chalk
                }
            }
            #[inline(always)]
            fn __stability() -> &'static Chalk {
                static LAZY: ::lazy_static::lazy::Lazy<Chalk> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for WARN {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    #[allow(missing_copy_implementations)]
    #[allow(non_camel_case_types)]
    #[allow(dead_code)]
    struct ERR {
        __private_field: (),
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    static ERR: ERR = ERR { __private_field: () };
    impl ::lazy_static::__Deref for ERR {
        type Target = Chalk;
        fn deref(&self) -> &Chalk {
            #[inline(always)]
            fn __static_ref_initialize() -> Chalk {
                {
                    let mut chalk = Chalk::new();
                    chalk.red().bold();
                    chalk
                }
            }
            #[inline(always)]
            fn __stability() -> &'static Chalk {
                static LAZY: ::lazy_static::lazy::Lazy<Chalk> = ::lazy_static::lazy::Lazy::INIT;
                LAZY.get(__static_ref_initialize)
            }
            __stability()
        }
    }
    impl ::lazy_static::LazyStatic for ERR {
        fn initialize(lazy: &Self) {
            let _ = &**lazy;
        }
    }
    pub fn start(args: Vec<String>, gh: bool) {
        if args.len() >= 1 {
            match args[0].as_str() {
                "create" => {
                    create::create(
                        args.len() > 1 && (&args[1] == "--force" || &args[1] == "-f"),
                    )
                }
                "build" => build::build_config(false, false),
                "upload" => build::build_config(true, gh),
                "help" => {
                    ::std::io::_print(format_args!("{0}\n", help::main_help()));
                }
                a => {
                    ::std::io::_print(format_args!("{0}\n", help::not_found(a)));
                }
            }
        } else {
            {
                ::std::io::_print(format_args!("{0}\n", help::main_help()));
            };
        }
    }
}
pub use app::shared;
pub fn node_entrypoint(args: Vec<String>, gh: bool) {
    app::start(args, gh);
}
