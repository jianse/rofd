
// #[macro_export]

macro_rules! define_add_field_macro {
    ($macro_name:ident,
       $(
        $(#[$new_field_meta:meta])*
        $new_field_vis:vis $new_field_name:ident : $new_field_type:ty
        ),*$(,)?

    )=>(
        define_add_field_macro!{
            #internal,
            [$],
            $macro_name,
            $(
                $(#[$new_field_meta])*
                $new_field_vis $new_field_name : $new_field_type,
            )*
            }
    );
    (#internal,
        [$dollar:tt],
        $macro_name:ident,
        $(
            $(#[$new_field_meta:meta])*
            $new_field_vis:vis $new_field_name:ident : $new_field_type:ty
            ),*$(,)?
    ) => {
        #[macro_export]

        macro_rules! $macro_name {
            ($dollar(#[$meta:meta])*
            $vis:vis struct $struct_name:ident {
                $dollar(
                    $dollar(#[$field_meta:meta])*
                    $field_vis:vis $field_name:ident : $field_type:ty
                ),*$dollar(,)?
            }
        ) => {
                // $($Type::$variant { $dollar($field $dollar(: $p)?,)* .. } )|+
                $dollar(#[$meta])*
                $vis struct $struct_name{
                    $(                        
                        $(#[$new_field_meta])*
                        $new_field_vis $new_field_name : $new_field_type ,
                    )*

                    $dollar(
                        $dollar(#[$field_meta])*
                        $field_vis $field_name : $field_type,
                    )*
                }
            }
        }
    };
}

define_add_field_macro!(ct_layer,
    #[serde(rename = "@Type")]
    r#type: Option<String>,

    #[serde(rename = "@DrawParam")]
    draw_param: Option<StRefId>,
);

define_add_field_macro!{
    ct_graphic_unit,

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