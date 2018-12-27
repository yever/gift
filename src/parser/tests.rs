use super::{
    application_extension, comment_extension, graphic_block, graphic_control_extension, image_data,
    image_descriptor, parse_gif, plain_text_block, version, Block, GIFVersion,
    GraphicControlExtension, ImageData, ImageDescriptor, SubBlocks, GIF,
};
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
        0x16, 0x8c, 0x2d, 0x99, 0x87, 0x2a, 0x1c, 0xdc, 0x33, 0xa0, 0x02, 0x75, 0xec, 0x95, 0xfa,
        0xa8, 0xde, 0x60, 0x8c, 0x04, 0x91, 0x4c, 0x01, 0x00,
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
                            0x16, 0x8c, 0x2d, 0x99, 0x87, 0x2a, 0x1c, 0xdc, 0x33, 0xa0, 0x02, 0x75,
                            0xec, 0x95, 0xfa, 0xa8, 0xde, 0x60, 0x8c, 0x04, 0x91, 0x4c, 0x01, 0x00,
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
        0x21, 0x01, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x64, 0x00, 0x64, 0x00, 0x14, 0x14, 0x01, 0x00,
        0x0B, 0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20, 0x77, 0x6F, 0x72, 0x6C, 0x64, 0x00,
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
        0x21, 0xFF, 0x0B, 0x4E, 0x45, 0x54, 0x53, 0x43, 0x41, 0x50, 0x45, 0x32, 0x2E, 0x30, 0x03,
        0x01, 0x05, 0x00, 0x00,
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

#[test]
fn should_parse_gif() {
    let gif_data = include_bytes!("../../fixtures/sample_1.gif");
    assert_eq!(
        parse_gif(gif_data),
        Ok(GIF {
            version: GIFVersion::GIF89a,
            width: 10,
            height: 10,
            global_color_table: Some(
                &[0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00][..]
            )
        })
    );
}
