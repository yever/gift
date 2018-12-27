use super::{
    subblocks::data_subblocks, Block, GraphicControlExtension, ImageData, ImageDescriptor,
    SubBlocks,
};
use nom::{le_u16, le_u8};

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

named!(pub block<&[u8], Block>, alt!(graphic_block | plain_text_block | application_extension | comment_extension));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_graphic_control_extension() {
        assert_eq!(
            graphic_control_extension(&[0x21, 0xf9, 0x04, 0x01, 0x64, 0x00, 0x02, 0x00][..]),
            Ok((
                &b""[..],
                GraphicControlExtension {
                    byte_size: 4,
                    packed_field: 0x01,
                    delay_time: 100,
                    transparent_color_index: 2,
                }
            ))
        );
    }

    #[test]
    fn should_read_image_descriptor() {
        assert_eq!(
            image_descriptor(&[0x2c, 0x01, 0x00, 0x02, 0x00, 0x05, 0x00, 0x06, 0x00, 0x81][..]),
            Ok((
                &b""[..],
                ImageDescriptor {
                    left: 1,
                    top: 2,
                    width: 5,
                    height: 6,
                    packed_field: 0x81,
                }
            ))
        );
    }

    #[test]
    fn should_parse_image_data() {
        let data = [1, 2, 255, 255, 3, 255, 255, 255, 0];
        assert_eq!(
            image_data(&data[..]),
            Ok((
                &b""[..],
                ImageData {
                    lzw_minimum_code_size: 1,
                    data: SubBlocks(&data[1..])
                }
            ))
        );

        let data = [3, 2, 255, 255, 3, 255, 255, 255, 0, 1, 2, 3];
        assert_eq!(
            image_data(&data[..]),
            Ok((
                &[1, 2, 3][..],
                ImageData {
                    lzw_minimum_code_size: 3,
                    data: SubBlocks(&data[1..9])
                }
            ))
        );
    }

    #[test]
    fn should_parse_graphic_block() {
        let data = [
            // graphic control extension
            0x21, 0xf9, 0x04, // byte size
            0x00, // packed field
            0x00, 0x00, // delay time
            0x00, // transparent color index
            0x00, // block terminator
            // image descriptor
            0x2c, 0x00, 0x00, // left
            0x00, 0x00, // top
            0x0a, 0x00, // width
            0x0a, 0x00, // height
            0x00, // packed field
            // no local color table
            // image data
            0x02, // LZW minimum code size
            0x16, 0x8c, 0x2d, 0x99, 0x87, 0x2a, 0x1c, 0xdc, 0x33, 0xa0, 0x02, 0x75, 0xec, 0x95,
            0xfa, 0xa8, 0xde, 0x60, 0x8c, 0x04, 0x91, 0x4c, 0x01, 0x00,
        ];
        assert_eq!(
            graphic_block(&data[..]),
            Ok((
                &[][..],
                Block::GraphicBlock {
                    graphic_control_extension: Some(GraphicControlExtension {
                        byte_size: 4,
                        packed_field: 0,
                        delay_time: 0,
                        transparent_color_index: 0,
                    }),
                    image_descriptor: ImageDescriptor {
                        left: 0,
                        top: 0,
                        width: 10,
                        height: 10,
                        packed_field: 0,
                    },
                    local_color_table: None,
                    image_data: ImageData {
                        lzw_minimum_code_size: 2,
                        data: SubBlocks(
                            &[
                                0x16, 0x8c, 0x2d, 0x99, 0x87, 0x2a, 0x1c, 0xdc, 0x33, 0xa0, 0x02,
                                0x75, 0xec, 0x95, 0xfa, 0xa8, 0xde, 0x60, 0x8c, 0x04, 0x91, 0x4c,
                                0x01, 0x00,
                            ][..]
                        )
                    }
                }
            ))
        );
    }

    #[test]
    fn should_parse_plain_text_block() {
        let data = [
            0x21, 0x01, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x64, 0x00, 0x64, 0x00, 0x14, 0x14, 0x01,
            0x00, 0x0B, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x00,
        ];
        assert_eq!(
            plain_text_block(&data[..]),
            Ok((
                &[][..],
                Block::TextBlock {
                    graphic_control_extension: None,
                    text: SubBlocks(&data[2..])
                }
            ))
        );
    }

    #[test]
    fn should_parse_application_extension() {
        let data = [
            0x21, 0xFF, 0x0B, 0x4E, 0x45, 0x54, 0x53, 0x43, 0x41, 0x50, 0x45, 0x32, 0x2E, 0x30,
            0x03, 0x01, 0x05, 0x00, 0x00,
        ];
        assert_eq!(
            application_extension(&data[..]),
            Ok((&[][..], Block::ApplicationExtension(SubBlocks(&data[2..]))))
        );
    }

    #[test]
    fn should_parse_comment_extension() {
        let data = [
            0x21, 0xFE, 0x09, 0x62, 0x6C, 0x75, 0x65, 0x62, 0x65, 0x72, 0x72, 0x79, 0x00,
        ];
        assert_eq!(
            comment_extension(&data[..]),
            Ok((&[][..], Block::CommentExtension(SubBlocks(&data[2..]))))
        );
    }
}
