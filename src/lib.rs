#[macro_use]
extern crate nom;

/// Part of the Header. Supported versions are "87a" and "89a".
///
/// See the GIF89a spec §17
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GIFVersion {
    GIF89a,
    GIF87a,
}

/// See the GIF89a spec §23
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GraphicControlExtension {
    pub byte_size: u8,
    pub packed_field: u8,
    pub delay_time: u16,
    pub transparent_color_index: u8,
}

/// A required block of the Table-Based Image containing the description of an image.
///
/// See the GIF89a spec §20
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageDescriptor {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub packed_field: u8,
}

/// A collections of sub-blocks, each one preceded by a u8 byte denoting its size and terminated by
/// a zero-sized sub-block (block terminator).
///
/// See the GIF89a spec §15-16
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct SubBlocks<'a>(pub &'a [u8]);

/// Table Based Image Data. 
///
/// See the GIF89a spec §22
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ImageData<'a> {
    pub lzw_minimum_code_size: u8,
    pub data: SubBlocks<'a>,
}

/// The various data blocks that comprise the content of a GIF.
///
/// See the GIF89a spec §12 and Appendix B.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Block<'a> {
    // Graphic Block with Graphic-Rendering Block
    GraphicBlock {
        graphic_control_extension: Option<GraphicControlExtension>,
        // Table-Based Image:
        image_descriptor: ImageDescriptor,
        local_color_table: Option<&'a [u8]>,
        image_data: ImageData<'a>,
    },
    // Graphic Block with Plain Text Extension
    TextBlock {
        graphic_control_extension: Option<GraphicControlExtension>,
        text: SubBlocks<'a>,
    },
    ApplicationExtension(SubBlocks<'a>),
    CommentExtension(SubBlocks<'a>),
}

/// The full structure of a GIF.
///
/// See Appendix B.
#[derive(Debug, PartialEq, Eq)]
pub struct GIF<'a> {
    pub version: GIFVersion,
    pub width: u16,
    pub height: u16,
    pub global_color_table: Option<&'a [u8]>,
    pub data: Vec<Block<'a>>,
}

pub mod parser;

pub use self::parser::parse_gif;
