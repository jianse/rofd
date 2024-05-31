use super::base::{StArray, StBox, StRefId};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

/// common Layer type
#[serde_as]
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CtLayer {
    #[serde(rename = "@Type")]
    // #[serde_as(as = "FromInto<String>")]
    r#type: Option<String>,

    #[serde(rename = "@DrawParam")]
    #[serde_as(as = "Option<DisplayFromStr>")]
    draw_param: Option<StRefId>,
}
macro_rules! add_field {
    () => {
        
    };
}

#[macro_export]
macro_rules! ct_layer{
    // (@field_def $field_vis:vis $field_name:ident : $field_type:ty) => {
    //     $field_vis $field_name : $field_type,
    // };
    // (@meta $(#[$meta:meta])*)=>{
    //     $(#[$meta])*
    // };
    // (@struct_def $vis:vis struct $struct_name:ident)=>{
    //     $vis:vis struct $struct_name:ident  
    // };
    (
     $(#[$meta:meta])* 
     $vis:vis struct $struct_name:ident {
        $(
        $(#[$field_meta:meta])*
        $field_vis:vis $field_name:ident : $field_type:ty
        ),*$(,)+
    }
    ) => {

            $(#[$meta])*
            $vis struct $struct_name{
                #[serde(rename = "@Type")]
                // #[serde_as(as = "FromInto<String>")]
                r#type: Option<String>,
        
                #[serde(rename = "@DrawParam")]
                draw_param: Option<StRefId>,

                $(
                $(#[$field_meta])*
                pub $field_name : $field_type,
                )*
            }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CtGraphicUnit {
    #[serde(rename = "@Boundary")]
    boundary: StBox,
    name: Option<String>,
    visible: Option<bool>,
    ctm: Option<StArray<f64>>,
    draw_param: Option<StRefId>,
    line_width: Option<f64>,
    cap: Option<String>,
    join: Option<String>,
    miter_limit: Option<f64>,
    dash_offset: Option<f64>,
    dash_pattern: Option<StArray<f64>>,
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

#[cfg(test)]
mod test_super {
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
    fn test_de_flatten() {
        let res = quick_xml::de::from_str::<TestStruct>(
            r#"<TestStruct Type="AAA" DrawParam="4" ID="123">content</TestStruct>"#,
        )
        .unwrap();
        assert_eq!(
            res,
            TestStruct {
                base: CtLayer {
                    r#type: Some("AAA".into()),
                    draw_param: Some(4)
                },
                id: 123
            }
        )
    }
    #[test]
    fn test_de_flatten_on_none() {
        let res = quick_xml::de::from_str::<TestStruct>(
            r#"<TestStruct Type="AAA" ID="123">content</TestStruct>"#,
        )
        .unwrap();
        assert_eq!(
            res,
            TestStruct {
                base: CtLayer {
                    r#type: Some("AAA".into()),
                    draw_param: None
                },
                id: 123
            }
        )
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
