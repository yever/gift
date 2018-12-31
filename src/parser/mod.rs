mod subblocks;
mod blocks;

use super::model::{
    Block, GraphicControlExtension, ImageData, ImageDescriptor, SubBlocks, GIF,
    GIFVersion
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

named!(gif<&[u8], GIF>,
       do_parse!(
                                    tag!("GIF")          >>
           version:                 version              >>
           width:                   le_u16               >>
           height:                  le_u16               >>
           packed_field:            le_u8                >>
           _background_color_index: le_u8                >>
           _pixel_aspect_ratio:     le_u8                >>
           global_color_table:      cond!(
               packed_field & 0b_1000_0000 != 0,
               take!(3 * (1 << ((packed_field & 0b_0000_0111) + 1)))
                                    )                    >>
           data:                   many0!(blocks::block) >>          
           (GIF {
               version: version,
               width: width,
               height: height,
               global_color_table: global_color_table,
               data: data,
           })
       )
);

pub fn parse_gif(gif_data: &[u8]) -> Result<GIF, ()> {
    match gif(gif_data) {
        Ok((_, gif)) => Ok(gif),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{Context::Code, Err::Error, ErrorKind};

    #[test]
    fn should_parse_version_89a() {
        assert_eq!(version(&b"89a"[..]), Ok((&b""[..], GIFVersion::GIF89a)));
    }

    #[test]
    fn should_parse_version_87a() {
        assert_eq!(version(&b"87a"[..]), Ok((&b""[..], GIFVersion::GIF87a)));
    }

    #[test]
    fn should_fail_on_unknown_version() {
        assert_eq!(
            version(&b"12a"[..]),
            Err(Error(Code(&b"12a"[..], ErrorKind::Alt)))
        );
    }
}
