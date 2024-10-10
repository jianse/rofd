//! some mods for work with the ofd xml files
//!
//! - [ofd::OfdXmlFile] for ofd main entry
//! - [document::DocumentXmlFile] for a document
//! - [page::PageXmlFile] for a page or a template
//! - [res::ResourceXmlFile] for public/document/page resources

pub mod document;
pub mod ofd;
mod ofd_parser;
pub mod page;
pub mod res;
