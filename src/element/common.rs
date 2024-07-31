use super::base::{StArray, StBox, StLoc, StRefId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Join {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Cap {
    Butt,
    Round,
    Square,
}

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
    alpha: Option<u8>,
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
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct CtColorSpace {
    #[serde(rename = "@Type")]
    pub r#type: String,

    /// default 8
    #[serde(rename = "@BitsPerComponent")]
    pub bits_per_component: Option<u8>,

    #[serde(rename = "@Profile")]
    pub profile: Option<StLoc>,

    #[serde(rename = "Palette")]
    pub palette: Palette,
}

/// TODO: CV field?
#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {}

#[derive(Debug, Serialize, Deserialize)]
pub struct CtDrawParam {
    #[serde(rename = "@Relative")]
    pub relative: Option<StRefId>,

    /// default 0.353
    #[serde(rename = "@LineWidth")]
    pub line_width: Option<f32>,

    /// default Miter
    #[serde(rename = "@Join")]
    pub join: Option<Join>,

    /// default Butt
    #[serde(rename = "@Cap")]
    pub cap: Option<Cap>,

    /// default 0
    #[serde(rename = "@DashOffset")]
    pub dash_offset: Option<f32>,

    #[serde(rename = "@DashPattern")]
    pub dash_pattern: Option<StArray<f32>>,

    /// default 4.234
    #[serde(rename = "@MiterLimit")]
    pub miter_limit: Option<f32>,

    #[serde(rename = "FillColor")]
    pub fill_color: Option<CtColor>,

    #[serde(rename = "StrokeColor")]
    pub stroke_color: Option<CtColor>,
}

/// CT_Font
#[derive(Debug, Serialize, Deserialize)]
pub struct CtFont {
    #[serde(rename = "@FontName")]
    pub font_name: String,

    #[serde(rename = "@FamilyName")]
    pub family_name: Option<String>,
    /// default unicode
    #[serde(rename = "@Charset")]
    pub charset: Option<String>,

    /// default false
    #[serde(rename = "@Italic")]
    pub italic: Option<bool>,

    /// default false
    #[serde(rename = "@Bold")]
    pub bold: Option<bool>,

    /// default false
    #[serde(rename = "@Serif")]
    pub serif: Option<bool>,

    /// default false
    #[serde(rename = "@FixedWidth")]
    pub fixed_width: Option<bool>,

    #[serde(rename = "FontFile")]
    pub font_file: Option<StLoc>,
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
