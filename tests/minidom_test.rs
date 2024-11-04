use chrono::{NaiveDate, NaiveDateTime};
use eyre::Result;
use minidom::Element;
use ofd_base::common::ActionType::{Goto, Uri};
use ofd_base::common::{Actions, CtAction, Event, VtTo};
use ofd_base::file::document::{
    CommonData, CtOutlineElem, CtPageArea, CtPermission, CtVPreferences, DocumentXmlFile, Outlines,
    Page, Pages, Print, TemplatePage, ValidPeriod,
};
use ofd_base::file::ofd::{CtDocInfo, CustomData, CustomDatas, DocBody, Keywords, OfdXmlFile};
use ofd_base::file::page::PageXmlFile;
use ofd_base::StBox;
use rofd::dom::{TryFromDom, OFD_NS};
use std::fs::File;
use std::io::{BufReader, Read};
use xdom::ser::XmlSer;

#[test]
fn test() -> Result<()> {
    let file = File::open("samples/ano/OFD.xml")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);
    let root: Element = data.parse()?;
    dbg!(&root);

    for child in root.children() {
        dbg!(child.text());
    }
    Ok(())
}

#[test]
fn test_try_from_dom_ofd() -> Result<()> {
    let file = File::open("samples/sample/OFD.xml")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);
    let root: Element = data.parse()?;
    let st = OfdXmlFile::try_from_dom(&root)?;
    // dbg!(&root);
    dbg!(&st);
    Ok(())
}

#[test]
fn test_try_from_dom_doc() -> Result<()> {
    let file = File::open("samples/sample/Doc_0/Document.xml")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();
    let _ = reader.read_to_string(&mut data);
    let root: Element = data.parse()?;
    let st = DocumentXmlFile::try_from_dom(&root)?;
    // dbg!(&root);
    dbg!(&st);
    Ok(())
}

#[test]
fn test_ofd_ser_to_ele() -> Result<()> {
    let doc_info = CtDocInfo {
        doc_id: Some("44107dc257034d38898838015df3e3ed".into()),
        title: Some("123".into()),
        author: Some("zhangsan".into()),
        subject: Some("<test>".into()),
        r#abstract: Some("this is a test for xml ser".into()),
        creation_date: NaiveDate::from_ymd_opt(2024, 10, 15),
        mod_date: NaiveDate::from_ymd_opt(2024, 10, 16),
        doc_usage: Some("test".into()),
        cover: Some("Res/img_0.png".into()),
        keywords: Some(Keywords {
            keywords: vec!["kw1".to_string(), "kw2".to_string(), "kw3".to_string()],
        }),
        creator: Some("rofd".into()),
        creator_version: Some("0.0.1".into()),
        custom_datas: Some(CustomDatas {
            custom_data: vec![
                CustomData {
                    name: "template-version".to_string(),
                    value: "1.0.20.0422".to_string(),
                },
                CustomData {
                    name: "native-producer".to_string(),
                    value: "SuwellFormSDK".to_string(),
                },
            ],
        }),
    };
    let a = OfdXmlFile {
        version: "1.0".to_string(),
        doc_type: "OFD".to_string(),
        doc_body: vec![
            DocBody {
                doc_info: doc_info.clone(),
                doc_root: Some("Doc_0/Document.xml".into()),
                versions: None,
                signatures: None,
            },
            DocBody {
                doc_info,
                doc_root: Some("Doc_1/Document.xml".into()),
                versions: None,
                signatures: Some("Doc_1/Signatures.xml".into()),
            },
        ],
    };

    // serialize
    let ser = XmlSer::builder()
        .name("OFD")
        .ns(OFD_NS)
        .prefix(Some("ofd".into()))
        .build()?;

    let e = ser.ser_to_element(&a)?;

    // to string
    let mut buf = Vec::new();
    e.write_to_decl(&mut buf)?;
    let xml_str = String::from_utf8(buf)?;

    println!("{}", xml_str);
    let mut file = File::create("output/OFD2.xml")?;

    // to file
    e.write_to_decl(&mut file)?;
    Ok(())
}

#[test]
fn test_doc_ser_to_ele() -> Result<()> {
    let a = DocumentXmlFile {
        common_data: CommonData {
            max_unit_id: 1234,
            page_area: CtPageArea {
                physical_box: StBox {
                    x: 0.0,
                    y: 0.0,
                    w: 1920.0,
                    h: 1080.0,
                },
                application_box: None,
                content_box: None,
                bleed_box: None,
            },
            public_res: Some(vec!["a".into(), "b".into()]),
            document_res: None,
            template_page: Some(vec![TemplatePage {
                id: 0,
                name: None,
                z_order: None,
                base_loc: Default::default(),
            }]),
            default_cs: Some(12),
        },
        pages: Pages {
            page: vec![Page {
                id: 0,
                base_loc: Default::default(),
            }],
        },
        outlines: Some(Outlines {
            outline_elems: vec![CtOutlineElem {
                title: "第一章".to_string(),
                count: Some(2),
                expanded: Some(true),
                actions: None,
                outline_elems: Some(vec![
                    CtOutlineElem {
                        title: "第一节".to_string(),
                        count: None,
                        expanded: None,
                        actions: None,
                        outline_elems: None,
                    },
                    CtOutlineElem {
                        title: "第二节".to_string(),
                        count: None,
                        expanded: None,
                        actions: None,
                        outline_elems: None,
                    },
                ]),
            }],
        }),
        permissions: Some(CtPermission {
            edit: Some(true),
            annot: None,
            export: None,
            signature: None,
            watermark: None,
            print_screen: None,
            print: Some(Print {
                printable: false,
                copies: Some(0),
            }),
            valid_period: Some(ValidPeriod {
                start_date: Some(NaiveDateTime::default()),
                end_date: None,
            }),
        }),
        actions: Some(Actions {
            actions: vec![
                CtAction {
                    event: Event::Click,
                    region: None,
                    action_type: Goto {
                        value: VtTo::Bookmark {
                            name: "??".to_string(),
                        },
                    },
                },
                CtAction {
                    event: Event::DO,
                    region: None,
                    action_type: Uri {
                        uri: "abc".to_string(),
                        base: None,
                    },
                },
            ],
        }),
        v_preferences: Some(CtVPreferences {
            page_mode: Some("None".into()),
            page_layout: None,
            tab_display: None,
            hide_toolbar: None,
            hide_menubar: None,
            hide_window_ui: None,
            zoom_mode: None,
            zoom: None,
        }),
        bookmarks: None,
        annotations: None,
        custom_tags: None,
        attachments: None,
        extensions: None,
    };
    // serialize
    let ser = XmlSer::builder()
        .name("Document")
        .ns(OFD_NS)
        .prefix(Some("ofd".into()))
        .build()?;

    let e = ser.ser_to_element(&a)?;

    // to string
    let mut buf = Vec::new();
    e.write_to_decl(&mut buf)?;
    let xml_str = String::from_utf8(buf)?;

    println!("{}", xml_str);
    let mut file = File::create("output/Doc2.xml")?;

    // to file
    e.write_to_decl(&mut file)?;

    Ok(())
}

#[test]
fn test_page_ser_to_ele() -> Result<()> {
    let file = File::open("samples/sample/Doc_0/Pages/Page_0/Content.xml")?;
    let reader = BufReader::new(file);
    let root = Element::from_reader(reader)?;
    let a = PageXmlFile::try_from_dom(&root)?;
    // dbg!(&a);

    // serialize
    let ser = XmlSer::builder()
        .name("Page")
        .ns(OFD_NS)
        .prefix(Some("ofd".into()))
        .build()?;
    let e = ser.ser_to_element(&a)?;

    // to string
    // let mut buf = Vec::new();
    // e.write_to_decl(&mut buf)?;
    // let xml_str = String::from_utf8(buf)?;

    // println!("{}", xml_str);
    let mut file = File::create("output/Page_0_Content.xml")?;

    // to file
    e.write_to_decl(&mut file)?;

    Ok(())
}

#[test]
fn test_nested_enum() -> Result<()> {
    let value = CtAction {
        event: Event::Click,
        region: None,
        action_type: Goto {
            value: VtTo::Bookmark {
                name: "??".to_string(),
            },
        },
    };
    let ser = XmlSer::builder()
        .name("Action")
        .ns(OFD_NS)
        .prefix(Some("ofd".into()))
        .build()?;

    let e = ser.ser_to_element(&value)?;
    let mut buf = Vec::new();
    e.write_to_decl(&mut buf)?;
    let xml_str = String::from_utf8(buf)?;
    println!("{}", xml_str);

    Ok(())
}

#[test]
fn float_format() -> Result<()> {
    let v = format!("{:.2}", 3.1995).trim_end_matches('0').to_string();
    println!("{}", v);
    Ok(())
}
