use crate::{ESealAppearance, Sign};
use der::asn1::{BitString, Ia5String, ObjectIdentifier, OctetString, UtcTime};
use der::Sequence;

/// 电子印章数据
#[derive(Debug, Sequence)]
pub struct SeSeal {
    /// 印章信息
    e_seal_info: SesSealInfo,

    /// 制章人对印章签名的信息
    sign_info: SesSignInfo,
}

/// 印章签名信息
#[derive(Debug, Sequence)]
pub struct SesSignInfo {
    cert: OctetString,
    signature_algorithm: ObjectIdentifier,
    sign_data: BitString,
}

#[derive(Debug, Sequence)]
pub struct SesSealInfo {
    /// 头信息
    header: SesHeader,

    /// 电子印章标识
    es_id: Ia5String,

    /// 印章属性信息
    property: SesEsPropertyInfo,

    /// 电子印章图片数据
    picture: SesEsPictureInfo,

    #[asn1(tag_mode = "EXPLICIT")]
    ext_data: Option<ExtensionData>,
}

/// 印章属性信息
#[derive(Debug, Sequence)]
pub struct SesEsPropertyInfo {
    /// 印章类型
    /// 如 1 为代为印章，2为个人印章
    r#type: u64,

    /// 印章名称
    name: String,

    /// 签章人证书列表
    cert_list: Vec<Cert>,

    /// 印章制作日期
    create_date: UtcTime,

    /// 印章有效起始日期
    valid_start: UtcTime,

    /// 印章有效终止日期
    valid_end: UtcTime,
}

pub type Cert = OctetString;

/// 印章图片信息
#[derive(Debug, Sequence)]
pub struct SesEsPictureInfo {
    /// 图片类型
    r#type: Ia5String,

    /// 图片数据
    data: OctetString,

    /// 图片显示宽度
    width: u64,

    /// 图片显示高度
    height: u64,
}

/// 头信息
#[derive(Debug, Sequence)]
pub struct SesHeader {
    /// 电子信息数据标识
    /// 固定为 `ES`
    id: Ia5String,

    /// 电子印章数据版本号标识
    version: u64,

    /// 电子印章厂商ID
    vid: Ia5String,
}

pub type ExtensionData = Vec<ExtData>;

#[derive(Debug, Sequence)]
pub struct ExtData {
    /// 自定义扩展字段标识
    extn_id: ObjectIdentifier,

    /// 自定义扩展字段是否关键
    #[asn1(default = "default_false")]
    critical: bool,

    /// 自定义扩展字段数据值
    extn_value: OctetString,
}

fn default_false() -> bool {
    false
}

/// 电子签章数据
#[derive(Debug, Sequence)]
pub struct SesSignature {
    /// 待电子签章数据
    to_sign: TbsSign,

    /// 电子签章中签名值
    signature: BitString,
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
            height: val.width,
            width: val.height,
            data: val.data.clone().into_bytes(),
            r#type: val.r#type.to_string(),
        }
    }
}

#[derive(Debug, Sequence)]
pub struct TbsSign {
    /// 版本信息
    version: i64,

    /// 电子印章
    e_seal: SeSeal,

    /// 签章时间信息
    time_info: BitString,

    /// 原文杂凑值
    data_hash: BitString,

    /// 原文数据的属性信息
    property_info: Ia5String,

    /// 签章人对应的签名证书
    cert: OctetString,

    /// 签名算法标识
    signature_algorithm: ObjectIdentifier,
}

#[cfg(test)]
mod tests {
    use crate::v1::SeSeal;
    use der::Decode;
    use eyre::Result;
    use std::fs::File;
    use std::io::Read;

    /// read e seal
    #[test]
    fn it_works() -> Result<()> {
        let mut file = File::open("../samples/UserV1.esl")?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)?;
        let eseal = SeSeal::from_der(&buf)?;
        dbg!(eseal);
        Ok(())
    }
}
