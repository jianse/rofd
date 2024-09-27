use minidom::Element;
use thiserror::Error;

mod doc;
mod ofd;
mod res;

pub trait TryFromDom<T>: Sized {
    type Error: std::error::Error;
    fn try_from_dom(dom: T) -> std::result::Result<Self, Self::Error>;
}
#[derive(Error, Debug)]
pub enum TryFromDomError {
    #[error("common error")]
    Common,
    #[error("no attribute named \"{0}\"")]
    NoSuchAttribute(&'static str),
    #[error("warped under layer error {0}")]
    Warp(Box<dyn std::error::Error + Send + Sync>),
    #[error("elementNameNotExpected. expected {0}, got {1}")]
    ElementNameNotExpected(&'static str, String),
}

const OFD_NS: &str = "http://www.ofdspec.org/2016";

#[inline]
fn parse_optional_from_attr<F, R, E>(
    dom: &Element,
    attr: &str,
    map_fn: F,
) -> Result<Option<R>, TryFromDomError>
where
    // R: FromStr,
    F: FnOnce(&str) -> Result<R, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let r = dom
        .attr(attr)
        .map(map_fn)
        .transpose()
        .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;
    Ok(r)
}

#[inline]
fn parse_required_from_attr<F, E, R>(
    dom: &Element,
    attr: &'static str,
    map_fn: F,
) -> Result<R, TryFromDomError>
where
    F: FnOnce(&str) -> Result<R, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let r = parse_optional_from_attr(dom, attr, map_fn)?;
    r.ok_or(TryFromDomError::NoSuchAttribute(attr))
}

#[inline]
fn parse_optional_from_ele<'e, F, E, R>(
    dom: &'e Element,
    el_name: &str,
    map_fn: F,
) -> Result<Option<R>, TryFromDomError>
where
    R: 'e,
    F: FnOnce(&'e Element) -> Result<R, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let r = dom
        .get_child(el_name, OFD_NS)
        .map(map_fn)
        .transpose()
        .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;
    Ok(r)
}

#[inline]
fn parse_required_from_ele<'e, F, E, R>(
    dom: &'e Element,
    el_name: &'static str,
    map_fn: F,
) -> Result<R, TryFromDomError>
where
    R: 'e,
    F: FnOnce(&'e Element) -> Result<R, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let r = parse_optional_from_ele(dom, el_name, map_fn)?;
    r.ok_or(TryFromDomError::NoSuchAttribute(el_name))
}

#[inline]
fn parse_optional_from_text<'e, F, E, R>(
    dom: &'e Element,
    el_name: &str,
    map_fn: F,
) -> Result<Option<R>, TryFromDomError>
where
    R: 'e,
    F: FnOnce(&str) -> Result<R, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let r = dom
        .get_child(el_name, OFD_NS)
        .map(Element::text)
        .map(|s| map_fn(s.as_str()))
        .transpose()
        .map_err(|e| TryFromDomError::Warp(Box::new(e)))?;
    Ok(r)
}

#[inline]
fn parse_required_from_text<'e, F, E, R>(
    dom: &'e Element,
    el_name: &'static str,
    map_fn: F,
) -> Result<R, TryFromDomError>
where
    R: 'e,
    F: FnOnce(&str) -> Result<R, E>,
    E: std::error::Error + Send + Sync + 'static,
{
    let r = parse_optional_from_text(dom, el_name, map_fn)?;
    r.ok_or(TryFromDomError::NoSuchAttribute(el_name))
}
