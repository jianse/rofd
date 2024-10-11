//! some mods for work with the ofd xml files
//!
//! - [ofd::OfdXmlFile] for ofd main entry
//! - [document::DocumentXmlFile] for a document
//! - [page::PageXmlFile] for a page or a template
//! - [res::ResourceXmlFile] for public/document/page resources
//! - [annotation::AnnotationsXmlFile] for annotations xml file.
//! - [annotation::AnnotationXmlFile] for annotation xml file.
//! - [signature::SignaturesXmlFile] for signatures xml file.
//! - [signature::SignatureXmlFile] for signature xml file.
//! - [version::VersionXmlFile] for version xml file.
//! - [attachment::AttachmentsXmlFile] for attachments xml file.

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

// signature related
pub mod signature;

// doc version related
pub mod version;

// attachment related
pub mod attachment;

//==============

// the inner parser module
mod ofd_parser;
