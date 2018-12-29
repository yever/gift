#[macro_use]
extern crate nom;

mod model;
mod parser;

pub use self::model::{
    Block, GIFVersion, GraphicControlExtension, ImageData, ImageDescriptor, SubBlocks, GIF,
};
pub use self::parser::parse_gif;
