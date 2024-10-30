pub mod de;
pub mod ser;
mod to_dom;

use chrono::NaiveDate;
use minidom::{Element, Node};
use std::path::PathBuf;

pub use to_dom::ToNode;

pub trait ToElement {
    fn to_element<N: AsRef<str>, NS: Into<String>>(
        &self,
        name: N,
        ns: NS,
        prefix: Option<String>,
    ) -> Option<Element>;
}

impl<T> ToElement for Option<T>
where
    T: ToElement,
{
    fn to_element<N: AsRef<str>, NS: Into<String>>(
        &self,
        _name: N,
        _ns: NS,
        _prefix: Option<String>,
    ) -> Option<Element> {
        match self {
            None => None,
            Some(t) => t.to_element(_name, _ns, _prefix),
        }
    }
}

impl ToElement for String {
    fn to_element<N: AsRef<str>, NS: Into<String>>(
        &self,
        name: N,
        ns: NS,
        prefix: Option<String>,
    ) -> Option<Element> {
        let ns = ns.into();
        let name = name.as_ref();
        let element = if prefix.is_some() {
            Element::builder(name, &ns)
                .prefix(prefix, &ns)
                .unwrap()
                .append(Node::Text(self.clone()))
                .build()
        } else {
            let mut e = Element::bare(name, &ns);
            e.append_node(Node::Text(self.clone()));
            e
        };

        Some(element)
    }
}

impl ToElement for NaiveDate {
    fn to_element<N: AsRef<str>, NS: Into<String>>(
        &self,
        name: N,
        ns: NS,
        prefix: Option<String>,
    ) -> Option<Element> {
        let ns = ns.into();
        if prefix.is_some() {
            Some(
                Element::builder(name, &ns)
                    .prefix(prefix, &ns)
                    .unwrap()
                    .append(Node::Text(self.to_string()))
                    .build(),
            )
        } else {
            Some(
                Element::builder(name, &ns)
                    .append(Node::Text(self.to_string()))
                    .build(),
            )
        }
    }
}

#[allow(unused)]
impl<T> ToElement for Vec<T>
where
    T: ToElement,
{
    fn to_element<N: AsRef<str>, NS: Into<String>>(
        &self,
        name: N,
        ns: NS,
        prefix: Option<String>,
    ) -> Option<Element> {
        todo!()
    }
}

impl ToElement for PathBuf {
    fn to_element<N: AsRef<str>, NS: Into<String>>(
        &self,
        name: N,
        ns: NS,
        prefix: Option<String>,
    ) -> Option<Element> {
        self.to_string_lossy()
            .to_string()
            .to_element(name, ns, prefix)
    }
}
