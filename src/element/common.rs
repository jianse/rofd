use super::base::{StArray, StBox, StRefId};
use serde::{Deserialize, Serialize};

/// common Layer type
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CtLayer {
    #[serde(rename = "@Type")]
    // #[serde_as(as = "FromInto<String>")]
    r#type: Option<String>,

    #[serde(rename = "@DrawParam")]
    draw_param: Option<StRefId>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CtGraphicUnit {
    #[serde(rename = "@Boundary")]
    boundary: StBox,
    name: Option<String>,
    visible: Option<bool>,
    ctm: Option<StArray<f32>>,
    draw_param: Option<StRefId>,
    line_width: Option<f32>,
    cap: Option<String>,
    join: Option<String>,
    miter_limit: Option<f32>,
    dash_offset: Option<f32>,
    dash_pattern: Option<StArray<f32>>,
    alapha: Option<u8>,
    #[serde(rename = "Actions")]
    actions: Option<Actions>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Actions {
    #[serde(rename = "Action")]
    actions: Vec<CtAction>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CtAction {}

/// 包括基本颜色、底纹和渐变
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CtColor {
    /// 各通道颜色的分量
    #[serde(rename = "@Value")]
    pub value: Option<StArray<u16>>,

    /// 调色板颜色
    #[serde(rename = "@Index")]
    pub index: Option<usize>,

    /// 引用资源文件中的颜色空间的标识
    /// 默认为文档设定的颜色空间 Document.xml/DefaultCS
    #[serde(rename = "@ColorSpace")]
    pub color_space: Option<StRefId>,

    /// 颜色透明度 取值范围是0~255 默认255
    #[serde(rename = "@Alpha")]
    pub alpha: Option<u8>,
    // TODO: p39
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestStruct {
        #[serde(flatten)]
        base: CtLayer,
        #[serde(rename = "@ID")]
        id: u64,
        // #[serde(rename = "$value")]
        // val: String,
    }

    #[test]
    fn test_de_on_none() {
        let res =
            quick_xml::de::from_str::<CtLayer>(r#"<CtLayer Type="AAA" ID="123">content</CtLayer>"#)
                .unwrap();
        assert_eq!(
            res,
            CtLayer {
                r#type: Some("AAA".into()),
                draw_param: None
            },
        )
    }
}
