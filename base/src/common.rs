use super::base::{StArray, StBox, StLoc, StPos, StRefId};
use crate::file::page::VtGraphicUnit;
use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Deserialize, Serialize, Clone, Copy, EnumString)]
pub enum Join {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, EnumString)]
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

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Actions {
    #[serde(rename = "Action")]
    pub actions: Vec<CtAction>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Event {
    DO,
    PO,
    #[serde(rename = "CLICK")]
    Click,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtAction {
    /// must be one of
    /// - DO
    /// - PO
    /// - CLICK
    #[serde(rename = "@Event")]
    pub event: Event,

    #[serde(rename = "Region")]
    pub region: Option<CtRegion>,

    /// choice
    #[serde(rename = "$value")]
    pub action_type: ActionType,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionType {
    Goto {
        /// choice
        #[serde(rename = "$value")]
        value: VtTo,
    },
    #[serde(rename = "URI")]
    Uri {
        #[serde(rename = "@URI")]
        uri: String,

        #[serde(rename = "@Base")]
        base: Option<String>,
    },
    GotoA {
        /// xs:IDREF
        #[serde(rename = "@AttachID")]
        attach_id: String,

        /// default true
        #[serde(rename = "@NewWindow")]
        new_window: Option<bool>,
    },
    Sound {
        #[serde(rename = "@ResourceID")]
        resource_id: StRefId,
        /// [0, 100] default 100
        #[serde(rename = "@Volume")]
        volume: Option<u32>,

        /// default false
        #[serde(rename = "@Repeat")]
        repeat: Option<bool>,

        /// default false
        #[serde(rename = "@Synchronous")]
        synchronous: Option<bool>,
    },
    Movie {
        #[serde(rename = "@ResourceID")]
        resource_id: StRefId,

        /// default  Play
        /// One of:
        ///  - Play
        ///  - Stop
        ///  - Pause
        ///  - Resume
        #[serde(rename = "@Operator")]
        operator: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VtTo {
    Dest(CtDest),
    Bookmark {
        #[serde(rename = "@Name")]
        name: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtDest {
    #[serde(rename = "@Type")]
    pub r#type: String,
    #[serde(rename = "@PageID")]
    pub page_id: StRefId,
    #[serde(rename = "@Left")]
    pub left: Option<f32>,
    #[serde(rename = "@Right")]
    pub right: Option<f32>,
    #[serde(rename = "@Top")]
    pub top: Option<f32>,
    #[serde(rename = "@Bottom")]
    pub bottom: Option<f32>,
    #[serde(rename = "@Zoom")]
    pub zoom: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtRegion {
    pub areas: Vec<Area>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Area {
    #[serde(rename = "@Start")]
    pub start: StPos,

    #[serde(rename = "$value")]
    pub path: Vec<VtPathOp>,
}

/// virtual type for path ops
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VtPathOp {
    Move {
        #[serde(rename = "@Point1")]
        point1: StPos,
    },
    Line {
        #[serde(rename = "@Point1")]
        point1: StPos,
    },
    QuadraticBezier {
        #[serde(rename = "@Point1")]
        point1: StPos,
        #[serde(rename = "@Point2")]
        point2: StPos,
    },
    CubicBezier {
        #[serde(rename = "@Point1")]
        point1: Option<StPos>,
        #[serde(rename = "@Point2")]
        point2: Option<StPos>,
        #[serde(rename = "@Point3")]
        point3: StPos,
    },
    Arc {
        #[serde(rename = "@SweepDirection")]
        sweep_direction: bool,
        #[serde(rename = "@LargeArc")]
        large_arc: bool,
        #[serde(rename = "@RotationAngle")]
        rotation_angle: f32,

        /// expect 2 elements
        #[serde(rename = "@EllipseSize")]
        ellipse_size: StArray<f32>,

        #[serde(rename = "@EndPoint")]
        end_point: StPos,
    },
    Close,
}
/// 包括基本颜色、底纹和渐变
#[derive(Debug, Serialize, Deserialize, Clone)]
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

    /// 底纹
    #[serde(rename = "Pattern")]
    pub pattern: Option<CtPattern>,

    // a color only contains one of these shadows
    #[serde(rename = "AxialShd")]
    pub axial_shd: Option<CtAxialShd>,

    #[serde(rename = "RadialShd")]
    pub radial_shd: Option<CtRadialShd>,

    #[serde(rename = "GouraudShd")]
    pub gouraud_shd: Option<Box<CtGouraudShd>>,

    #[serde(rename = "LaGouraudShd")]
    pub la_gouraud_shd: Option<Box<CtLaGouraudShd>>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtLaGouraudShd {
    #[serde(rename = "@VerticesPerRow")]
    pub vertices_per_row: u32,

    /// could be 0,1 default is 0
    #[serde(rename = "@Extend")]
    pub extend: Option<u8>,

    /// at least 4 elements
    #[serde(rename = "Point")]
    pub points: Vec<Point>,

    /// must be a basic color
    #[serde(rename = "BackColor")]
    pub back_color: Option<CtColor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtGouraudShd {
    /// could be 0,1 default is 0
    #[serde(rename = "@Extend")]
    pub extend: Option<u8>,

    /// at least 3 elements
    #[serde(rename = "Point")]
    pub points: Vec<Point>,

    /// must be a basic color
    #[serde(rename = "BackColor")]
    pub back_color: Option<CtColor>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub edge_flag: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtRadialShd {
    #[serde(rename = "@MapType")]
    pub map_type: Option<String>,

    #[serde(rename = "@MapUnit")]
    pub map_unit: Option<f32>,

    /// default 0
    #[serde(rename = "@Eccentricity")]
    pub eccentricity: Option<f32>,

    #[serde(rename = "@Angle")]
    pub angle: Option<f32>,

    #[serde(rename = "@StartPoint")]
    pub start_point: StPos,

    /// default 0
    #[serde(rename = "@StartRadius")]
    pub start_radius: Option<f32>,

    #[serde(rename = "@EndPoint")]
    pub end_point: StPos,

    #[serde(rename = "@EndRadius")]
    pub end_radius: f32,

    /// could be 0,1,2,3 default is 0
    #[serde(rename = "@Extend")]
    pub extend: Option<u8>,

    // at lease 2 element
    pub segment: Vec<Segment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtAxialShd {
    #[serde(rename = "@MapType")]
    pub map_type: Option<String>,

    #[serde(rename = "@MapUnit")]
    pub map_unit: Option<f32>,

    /// could be 0,1,2,3 default is 0
    #[serde(rename = "@Extend")]
    pub extend: Option<u8>,

    #[serde(rename = "@StartPoint")]
    pub start_point: StPos,

    #[serde(rename = "@EndPoint")]
    pub end_point: StPos,

    // at lease 2 element
    pub segment: Vec<Segment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Segment {
    /// [0, 1.0]
    #[serde(rename = "@Position")]
    pub position: Option<f32>,

    /// this must be basic color
    #[serde(rename = "Color")]
    pub color: CtColor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CtPattern {
    #[serde(rename = "@Width")]
    pub width: f32,

    #[serde(rename = "@Height")]
    pub height: f32,

    #[serde(rename = "@XStep")]
    pub x_step: Option<f32>,

    #[serde(rename = "@YStep")]
    pub y_step: Option<f32>,

    #[serde(rename = "@ReflectMethod")]
    pub reflect_method: Option<String>,

    /// can be `Page` or `Object`
    /// default `Object`
    #[serde(rename = "@RelativeTo")]
    pub relative_to: Option<String>,

    #[serde(rename = "@CTM")]
    pub ctm: Option<StArray<f32>>,

    #[serde(rename = "CellContent")]
    pub cell_content: Vec<CellContent>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CellContent {
    #[serde(rename = "@Thumbnail")]
    pub thumbnail: Option<StRefId>,

    /// inherit
    /// note: now we do not use serde to do this
    pub base: VtGraphicUnit,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Palette {
    /// this is a table
    /// 0 -> color_value1
    /// 1 -> color_value2
    /// etc.
    #[serde(rename = "CV")]
    pub cv: Vec<StArray<u16>>,
}

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
