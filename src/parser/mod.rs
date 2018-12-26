mod subblocks;

#[cfg(test)]
mod tests;

use self::subblocks::data_subblocks;
use super::{
    Block, GIFVersion, GraphicControlExtension, ImageData, ImageDescriptor, SubBlocks, GIF,
};
use nom::{le_u16, le_u8};
use std::result::Result;

fn get_version(bytes: &[u8]) -> Result<GIFVersion, ()> {
    match bytes {
        b"87a" => Ok(GIFVersion::GIF87a),
        b"89a" => Ok(GIFVersion::GIF89a),
        _ => Err(()),
    }
}

named!(version<&[u8], GIFVersion>, map_res!(alt!(tag!("87a") | tag!("89a")), get_version));

named!(graphic_control_extension<&[u8], GraphicControlExtension>,
       do_parse!(
                                    tag!([0x21, 0xf9]) >>
           byte_size:               le_u8              >>
           packed_field:            le_u8              >>
           delay_time:              le_u16             >>
           transparent_color_index: le_u8              >>
                                    tag!([0x00])       >>
           (GraphicControlExtension {
               byte_size: byte_size,
               packed_field: packed_field,
               delay_time: delay_time,
               transparent_color_index: transparent_color_index,
           })
       )
);

named!(image_descriptor<&[u8], ImageDescriptor>,
       do_parse!(
                         tag!([0x2c]) >>
           left:         le_u16       >>
           top:          le_u16       >>
           width:        le_u16       >>
           height:       le_u16       >>
           packed_field: le_u8        >>
           (ImageDescriptor {
                left: left,
                top: top,
                width: width,
                height: height,
                packed_field: packed_field,
           })
       )
);

named!(image_data<&[u8], ImageData>,
       do_parse!(
           lzw_minimum_code_size: le_u8          >>
           data:                  data_subblocks >>
           (ImageData {
               lzw_minimum_code_size: lzw_minimum_code_size,
               data: data
           })
        )
);

named!(graphic_block<&[u8], Block>,
       do_parse!(
           graphic_control_extension: opt!(graphic_control_extension) >>
           image_descriptor:          image_descriptor                >>
           local_color_table:         cond!(

               image_descriptor.packed_field & 0b_1000_0000 != 0,
               take!(3 * (1 << ((image_descriptor.packed_field & 0b_0000_0111) + 1)))
                                      )                               >>
           image_data:                image_data                      >>
           (Block::GraphicBlock {
               graphic_control_extension: graphic_control_extension,
               image_descriptor: image_descriptor,
               local_color_table: local_color_table,
               image_data: image_data
           })
       )
);

named!(plain_text_block<&[u8], Block>,
       do_parse!(
           graphic_control_extension: opt!(graphic_control_extension) >>
                                      tag!([0x21, 0x01])              >>
           text:                      data_subblocks                  >>
           (Block::TextBlock {
               graphic_control_extension: graphic_control_extension,
               text: text
           })
       )
);

named!(application_extension<&[u8], Block>,
       do_parse!(
                 tag!([0x21, 0xff]) >>
           data: data_subblocks     >>
           (Block::ApplicationExtension(data))
       )
);

named!(comment_extension<&[u8], Block>,
       do_parse!(
                 tag!([0x21, 0xfe]) >>
           data: data_subblocks     >>
           (Block::CommentExtension(data))
       )
);

named!(gif<&[u8], GIF>,
       do_parse!(
                                    tag!("GIF") >>
           version:                 version     >>
           width:                   le_u16      >>
           height:                  le_u16      >>
           packed_field:            le_u8       >>
           _background_color_index: le_u8       >>
           _pixel_aspect_ratio:     le_u8       >>
           global_color_table:      cond!(
               packed_field & 0b_1000_0000 != 0,
               take!(3 * (1 << ((packed_field & 0b_0000_0111) + 1)))
                                    )           >>
           (GIF {
               version: version,
               width: width,
               height: height,
               global_color_table: global_color_table
           })
       )
);

pub fn parse_gif(gif_data: &[u8]) -> Result<GIF, ()> {
    use std::result::Result::{Err, Ok};

    match gif(gif_data) {
        Ok((_, gif)) => Ok(gif),
        Err(_) => Err(()),
    }
}
