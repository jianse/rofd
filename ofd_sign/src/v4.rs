#![allow(unused)]

use der::asn1::{
    BitString, GeneralizedTime, Ia5String, ObjectIdentifier, OctetString, PrintableString,
};
use der::{Choice, Sequence};

/// entry for sign
#[derive(Sequence, Debug)]
pub struct SesSignature {
    /// 签章信息
    to_sign: TbsSign,

    /// 签章者证书
    cert: OctetString,

    /// 签名算法标识
    signature_alg_id: ObjectIdentifier,

    /// 签名值
    signature: BitString,

    /// 对签名值的时间戳
    timestamp: Option<BitString>,
}

#[derive(Sequence, Debug)]
pub struct TbsSign {
    /// 电子签章版本号，与电子印章版本号保持一致
    version: i64,

    /// 电子印章
    e_seal: SesSeal,

    /// 签章时间
    time_info: GeneralizedTime,

    /// 原文杂凑值
    data_hash: BitString,

    /// 原文数据的属性
    property_info: Ia5String,

    /// 自定义数据
    ext_data: Option<ExtensionData>,
}

pub type ExtensionData = Vec<ExtData>;

#[derive(Sequence, Debug)]
pub struct ExtData {
    //// 自定义扩展字段标识
    extn_id: ObjectIdentifier,

    /// 自定义扩展字段是否关键
    /// 默认 false
    critical: bool,

    /// 自定义扩展字段数据值
    extn: OctetString,
}

/// 电子印章数据
#[derive(Sequence, Debug)]
pub struct SesSeal {
    /// 印章信息
    e_seal_info: SesSealInfo,

    /// 制章者证书
    cert: OctetString,

    /// 签名算法标识
    signature_alg_id: ObjectIdentifier,

    /// 签名值
    signed_value: BitString,
}

/// 印章信息
#[derive(Sequence, Debug)]
pub struct SesSealInfo {
    /// 印章头
    header: SesHeader,

    /// 印章标识
    es_id: Ia5String,

    /// 印章属性
    property: SesEsPropertyInfo,

    /// 印章图像数据
    picture: SesEsPictureInfo,

    /// 自定义数据
    ext_data: Option<ExtensionData>,
}

#[derive(Sequence, Debug)]
pub struct SesEsPictureInfo {
    /// 图像类型
    /// 印章福祥数据格式类型，如 GIF BMP JPG PNG SVG 等
    r#type: Ia5String,

    /// 印章图像数据
    data: OctetString,

    /// 图像显示宽度，单位毫米
    width: i64,

    /// 图像显示高度，单位毫米
    height: i64,
}

/// 印章属性
#[derive(Sequence, Debug)]
pub struct SesEsPropertyInfo {
    /// 印章类型
    r#type: i64,

    /// 印章名称
    name: String,

    /// 签章者证书信息类型
    /// 1 -> 数字证书
    /// 2 -> 数字证书的杂凑值
    cert_list_type: i64,

    /// 签章者证书信息列表
    cert_list: SesCertList,

    /// 印章制作时间
    create_date: GeneralizedTime,

    /// 印章有效期起始时间
    valid_start: GeneralizedTime,

    /// 印章有效期终止时间
    valid_end: GeneralizedTime,
}

#[derive(Choice, Debug)]
pub enum SesCertList {
    /// 签章者证书
    Certs(CertInfoList),

    /// 签章者证书杂凑值
    CertDigestList(CertDigestList),
}

pub type CertInfoList = Vec<OctetString>;

pub type CertDigestList = Vec<CertDigestObj>;

#[derive(Sequence, Debug)]
pub struct CertDigestObj {
    /// 自定义类型
    r#type: PrintableString,

    /// 证书杂凑值
    value: OctetString,
}

/// 印章头
#[derive(Sequence, Debug)]
pub struct SesHeader {
    /// 头标识，固定值 `ES`
    id: Ia5String,

    /// 印章版本，当前为 `4`
    version: i64,

    /// 厂商标识
    vid: Ia5String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use der::Decode;
    use eyre::Result;
    use std::fs::File;
    use std::io::{Read, Write};
    #[test]
    fn read_sign() -> Result<()> {
        let mut f = File::open("../samples/SignedValue.dat")?;
        let mut data = Vec::new();
        let _ = f.read_to_end(&mut data)?;
        let seq = SesSignature::from_der(&data);
        dbg!(&seq);
        let sign = seq?;
        let data = sign.to_sign.e_seal.e_seal_info.picture.data;
        // File::create("../samples/stamp.ofd")?.write_all(data.as_ref())?;
        Ok(())
    }
}
