//! some mods for work with the ofd xml files
//!
//! - [ofd::OfdXmlFile] for ofd main entry
//! - [document::DocumentXmlFile] for a document
//! - [page::PageXmlFile] for a page or a template
//! - [res::ResourceXmlFile] for public/document/page resources
//! - [annotation::AnnotationsXmlFile] for annotations xml file.
//! - [annotation::AnnotationXmlFile] for annotation xml file.

// ofd file related
pub mod ofd;

// document related
pub mod document;

// page related
pub mod page;

// resource related
pub mod res;

// annotation related
pub mod annotation;

//==============

// the inner parser module
mod ofd_parser;
