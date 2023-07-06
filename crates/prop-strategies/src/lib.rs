// mod constructors;
mod generic_types;
mod string;

use asn1_parser::{Asn1Type, OwnedAsn1Type};
// pub use constructors::*;
pub use generic_types::*;
use proptest::collection::vec;
use proptest::prelude::any;
use proptest::prop_oneof;
use proptest::strategy::Strategy;
pub use string::*;

pub fn bytes(size: usize) -> impl Strategy<Value = Vec<u8>> {
    vec(any::<u8>(), 0..size).no_shrink()
}

pub fn string(len: usize) -> impl Strategy<Value = String> {
    vec(any::<char>(), len)
        .prop_map(|v| v.iter().collect::<String>())
        .no_shrink()
}

pub fn any_asn1_type() -> impl Strategy<Value = OwnedAsn1Type> {
    prop_oneof![
        any_octet_string().prop_map(Asn1Type::OctetString),
        any_utf8_string().prop_map(Asn1Type::Utf8String),
        any_bit_string().prop_map(Asn1Type::BitString),
        any_bool().prop_map(Asn1Type::Bool),
    ].no_shrink()
}
