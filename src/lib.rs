#[macro_use]
extern crate nom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GIFVersion {
    GIF89a,
    GIF87a,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GraphicControlExtension {
    byte_size: u8,
    packed_field: u8,
    delay_time: u16,
    transparent_color_index: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageDescriptor {
    left: u16,
    top: u16,
    width: u16,
    height: u16,
    packed_field: u8,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SubBlocks<'a>(&'a [u8]);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageData<'a> {
    lzw_minimum_code_size: u8,
    data: SubBlocks<'a>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Block<'a> {
    GraphicBlock {
        graphic_control_extension: Option<GraphicControlExtension>,
        image_descriptor: ImageDescriptor,
        local_color_table: Option<&'a [u8]>,
        image_data: ImageData<'a>,
    },
    TextBlock {
        graphic_control_extension: Option<GraphicControlExtension>,
        text: SubBlocks<'a>,
    },
    ApplicationExtension(SubBlocks<'a>),
    CommentExtension(SubBlocks<'a>),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GIF<'a> {
    pub version: GIFVersion,
    pub width: u16,
    pub height: u16,
    global_color_table: Option<&'a [u8]>,
}

pub mod parser;

pub use self::parser::parse_gif;
