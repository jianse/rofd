#![allow(unused)]

use crate::{ESealAppearance, Sign};
use der::asn1::{
    BitString, GeneralizedTime, Ia5String, ObjectIdentifier, OctetString, PrintableString,
};
use der::{Choice, Sequence};

/// entry for sign
#[derive(Sequence, Debug)]
pub struct SesSignature {
    /// 签章信息
    pub to_sign: TbsSign,

    /// 签章者证书
    pub cert: OctetString,

    /// 签名算法标识
    pub signature_alg_id: ObjectIdentifier,

    /// 签名值
    pub signature: BitString,

    /// 对签名值的时间戳
    pub timestamp: Option<BitString>,
}

impl Sign for SesSignature {
    fn appearance(&self) -> ESealAppearance {
        let pic = &self.to_sign.e_seal.e_seal_info.picture;
        pic.into()
    }
}

impl From<&SesEsPictureInfo> for ESealAppearance {
    fn from(val: &SesEsPictureInfo) -> ESealAppearance {
        ESealAppearance {
            height: val.height,
            width: val.width,
            data: val.data.clone().into_bytes(),
            r#type: val.r#type.to_string(),
        }
    }
}

#[derive(Sequence, Debug)]
pub struct TbsSign {
    /// 电子签章版本号，与电子印章版本号保持一致
    pub version: i64,

    /// 电子印章
    pub e_seal: SesSeal,

    /// 签章时间
    pub time_info: GeneralizedTime,

    /// 原文杂凑值
    pub data_hash: BitString,

    /// 原文数据的属性
    pub property_info: Ia5String,

    /// 自定义数据
    pub ext_data: Option<ExtensionData>,
}

pub type ExtensionData = Vec<ExtData>;

#[derive(Sequence, Debug)]
pub struct ExtData {
    //// 自定义扩展字段标识
    pub extn_id: ObjectIdentifier,

    /// 自定义扩展字段是否关键
    /// 默认 false
    pub critical: bool,

    /// 自定义扩展字段数据值
    pub extn: OctetString,
}

/// 电子印章数据
#[derive(Sequence, Debug)]
pub struct SesSeal {
    /// 印章信息
    pub e_seal_info: SesSealInfo,

    /// 制章者证书
    pub cert: OctetString,

    /// 签名算法标识
    pub signature_alg_id: ObjectIdentifier,

    /// 签名值
    pub signed_value: BitString,
}

/// 印章信息
#[derive(Sequence, Debug)]
pub struct SesSealInfo {
    /// 印章头
    pub header: SesHeader,

    /// 印章标识
    pub es_id: Ia5String,

    /// 印章属性
    pub property: SesEsPropertyInfo,

    /// 印章图像数据
    pub picture: SesEsPictureInfo,

    /// 自定义数据
    pub ext_data: Option<ExtensionData>,
}

#[derive(Sequence, Debug)]
pub struct SesEsPictureInfo {
    /// 图像类型
    /// 印章福祥数据格式类型，如 GIF BMP JPG PNG SVG 等
    pub r#type: Ia5String,

    /// 印章图像数据
    pub data: OctetString,

    /// 图像显示宽度，单位毫米
    pub width: u64,

    /// 图像显示高度，单位毫米
    pub height: u64,
}

/// 印章属性
#[derive(Sequence, Debug)]
pub struct SesEsPropertyInfo {
    /// 印章类型，可根据业务需要自行定义
    pub r#type: i64,

    /// 印章名称
    pub name: String,

    /// 签章者证书信息类型
    /// 1 -> 数字证书
    /// 2 -> 数字证书的杂凑值
    pub cert_list_type: i64,

    /// 签章者证书信息列表
    pub cert_list: SesCertList,

    /// 印章制作时间
    pub create_date: GeneralizedTime,

    /// 印章有效期起始时间
    pub valid_start: GeneralizedTime,

    /// 印章有效期终止时间
    pub valid_end: GeneralizedTime,
}

#[derive(Choice, Debug)]
pub enum SesCertList {
    /// 签章者证书
    Certs(CertInfoList),

    /// 签章者证书杂凑值
    CertDigestList(CertDigestList),
}

/// 签章者证书列表
pub type CertInfoList = Vec<OctetString>;

/// 签章者证书杂凑值列表
pub type CertDigestList = Vec<CertDigestObj>;

#[derive(Sequence, Debug)]
pub struct CertDigestObj {
    /// 自定义类型
    pub r#type: PrintableString,

    /// 证书杂凑值
    pub value: OctetString,
}

/// 印章头
#[derive(Sequence, Debug)]
pub struct SesHeader {
    /// 头标识，固定值 `ES`
    pub id: Ia5String,

    /// 印章版本，当前为 `4`
    pub version: i64,

    /// 厂商标识
    pub vid: Ia5String,
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

    #[test]
    fn read_sign1() -> Result<()> {
        let mut f = File::open("../samples/001/Doc_0/Signs/Sign_0/SignedValue.dat")?;
        let mut data = Vec::new();
        let _ = f.read_to_end(&mut data)?;
        let seq = SesSignature::from_der(&data);
        // dbg!(&seq);
        let sign = seq?;
        let image_type = sign.to_sign.e_seal.e_seal_info.picture.r#type.to_string();
        dbg!(&image_type);
        // File::create("../samples/stamp.ofd")?.write_all(data.as_ref())?;
        Ok(())
    }
}
