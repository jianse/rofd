#![allow(unused)]

use asn1::{Asn1Read, BitString, GeneralizedTime, IA5String, ObjectIdentifier, OctetStringEncoded};
// use der::asn1::OctetString;
// use der::DateTime;

#[derive(Asn1Read, Debug)]
pub struct SesSignature<'a> {
    to_sign: TbsSign<'a>,
    cert: OctetStringEncoded<&'a [u8]>,
    signature_alg_id: ObjectIdentifier,
    signature: BitString<'a>,
    timestamp: Option<BitString<'a>>,
}

#[derive(Asn1Read, Debug)]
pub struct TbsSign<'a> {
    version: i64,
    e_seal: SesSeal<'a>,
    time_info: GeneralizedTime,
    data_hash: BitString<'a>,
    property_info: IA5String<'a>,

    // #[implicit(0)]
    ext_data: Option<ExtensionData>,
}

#[derive(Asn1Read, Debug)]
pub struct ExtensionData {}

#[derive(Asn1Read, Debug)]
pub struct SesSeal<'a> {
    e_seal_info: SesSealInfo<'a>,
}

#[derive(Asn1Read, Debug)]
pub struct SesSealInfo<'a> {
    header: SesHeader,
    es_id: IA5String<'a>,
    property: SesEsPropertyInfo,
    picture: SesEsPictureInfo,
    ext_data: Option<ExtensionData>,
}

#[derive(Asn1Read, Debug)]
pub struct SesEsPictureInfo {}

#[derive(Asn1Read, Debug)]
pub struct SesEsPropertyInfo {}

#[derive(Asn1Read, Debug)]
pub struct SesHeader {}
