#[macro_use]
extern crate nom;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GIFVersion {
    GIF89a,
    GIF87a,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GIF<'a> {
    pub version: GIFVersion,
    pub width: u16,
    pub height: u16,
    global_color_table: Option<&'a [u8]>,
}

pub mod parser;

pub use self::parser::parse_gif;
