#![cfg(feature = "url-impl")]
#![allow(unused)]

use ts_rs::TS;
use url::Url;

#[test]
fn contains_url() {

    #[derive(TS)]
    pub struct StructWithUrl {
        some_url: Url,
    }

    assert_eq!(
        StructWithUrl::decl(),
        "interface StructWithUrl { some_url: string, }"
    )
}


#[test]
fn newtype_url() {

    #[derive(TS)]
    pub struct SomeUrl(Url);

    assert_eq!(
        SomeUrl::decl(),
        "type SomeUrl = string;"
    )
}
