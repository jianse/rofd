use minidom::Element;
use ofd_base::ParseStBoxError;
use std::convert::Infallible;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;
use strum::ParseError;
use thiserror::Error;

mod doc;
mod ofd;
mod page;
mod res;

pub trait TryFromDom<T>: Sized {
    fn try_from_dom(dom: T) -> Result<Self, TryFromDomError>;
}
#[derive(Error, Debug)]
pub enum TryFromDomError {
    #[error("no attribute named \"{0}\"")]
    NoSuchAttribute(&'static str),
    #[error("no element named \"{0}\"")]
    NoSuchElement(&'static str),
    #[error("warped under layer error {0}")]
    Warp(Box<dyn std::error::Error + Send + Sync>),
    #[error("elementNameNotExpected. expected {0}, got {1}")]
    ElementNameNotExpected(&'static str, String),
}

impl From<ParseIntError> for TryFromDomError {
    fn from(value: ParseIntError) -> Self {
        Self::Warp(Box::new(value))
    }
}

impl From<Infallible> for TryFromDomError {
    fn from(value: Infallible) -> Self {
        Self::Warp(Box::new(value))
    }
}
impl From<ParseError> for TryFromDomError {
    fn from(value: ParseError) -> Self {
        Self::Warp(Box::new(value))
    }
}

impl From<ParseFloatError> for TryFromDomError {
    fn from(value: ParseFloatError) -> Self {
        Self::Warp(Box::new(value))
    }
}
impl From<ParseBoolError> for TryFromDomError {
    fn from(value: ParseBoolError) -> Self {
        Self::Warp(Box::new(value))
    }
}
impl From<ParseStBoxError> for TryFromDomError {
    fn from(value: ParseStBoxError) -> Self {
        Self::Warp(Box::new(value))
    }
}

impl From<chrono::ParseError> for TryFromDomError {
    fn from(value: chrono::ParseError) -> Self {
        Self::Warp(Box::new(value))
    }
}

pub const OFD_NS: &str = "http://www.ofdspec.org/2016";

#[inline]
fn parse_optional_from_attr<F, R, E>(
    dom: &Element,
    attr: &str,
    map_fn: F,
) -> Result<Option<R>, TryFromDomError>
where
    F: FnOnce(&str) -> Result<R, E>,
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
{
    let r = dom.attr(attr).map(map_fn).transpose()?;
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
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
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
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
{
    let r = dom.get_child(el_name, OFD_NS).map(map_fn).transpose()?;
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
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
{
    let r = parse_optional_from_ele(dom, el_name, map_fn)?;
    r.ok_or(TryFromDomError::NoSuchElement(el_name))
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
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
{
    let r = dom
        .get_child(el_name, OFD_NS)
        .map(Element::text)
        .map(|s| map_fn(s.as_str()))
        .transpose()?;
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
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
{
    let r = parse_optional_from_text(dom, el_name, map_fn)?;
    r.ok_or(TryFromDomError::NoSuchElement(el_name))
}

#[inline]
fn parse_optional_vec<'e, F, E, R>(
    dom: &'e Element,
    el_name: Option<&str>,
    map_fn: F,
) -> Result<Option<Vec<R>>, TryFromDomError>
where
    R: 'e,
    F: FnMut(&'e Element) -> Result<R, E>,
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
{
    let result = dom
        .children()
        .filter(|e| match el_name {
            None => true,
            Some(n) => e.name() == n,
        })
        .map(map_fn)
        .collect::<Result<Vec<_>, _>>()?;
    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

#[inline]
fn parse_required_vec<'e, F, E, R>(
    dom: &'e Element,
    el_name: Option<&'static str>,
    map_fn: F,
) -> Result<Vec<R>, TryFromDomError>
where
    R: 'e,
    F: FnMut(&'e Element) -> Result<R, E>,
    E: Into<TryFromDomError>,
    TryFromDomError: From<E>,
{
    let r = parse_optional_vec(dom, el_name, map_fn)?;
    r.ok_or(TryFromDomError::NoSuchElement(el_name.unwrap_or("$value")))
}
