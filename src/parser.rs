use super::{GIFVersion, GIF};
use nom::le_u16;

named!(signature, tag!("GIF"));

fn get_version(bytes: &[u8]) -> Result<GIFVersion, ()> {
    match bytes {
        b"87a" => Ok(GIFVersion::GIF87a),
        b"89a" => Ok(GIFVersion::GIF89a),
        _ => Err(()),
    }
}

named!(version<&[u8], GIFVersion>, map_res!(alt!(tag!("87a") | tag!("89a")), get_version));

named!(
    gif<&[u8], GIF>,
    do_parse!(
                             signature >>
    version:                 version   >>
    width:                   le_u16    >>
    height:                  le_u16    >>
    packed_field:            take!(1)  >>
    background_color_index:  take!(1)  >>
    pixel_aspect_ratio:      take!(1)  >>
    global_color_table:      cond!(
        packed_field[0] & 0x80 != 0,
        take!(3 * (1 << ((packed_field[0] & 0b_0000_0111) + 1)))
                             )         >>
    (GIF {
        version: version,
        width: width,
        height: height,
        global_color_table: global_color_table
    }))
);

pub fn parse_gif(gif_data: &[u8]) -> std::result::Result<GIF, ()> {
    use std::result::Result::{Err, Ok};

    match gif(gif_data) {
        Ok((_, gif)) => Ok(gif),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        Context::Code,
        Err::Error,
        ErrorKind::{Alt, Tag},
    };

    #[test]
    fn signature_should_parse_gif() {
        assert_eq!(signature(&b"GIF89a"[..]), Ok((&b"89a"[..], &b"GIF"[..])));
    }

    #[test]
    fn signature_should_fail_on_invalid_signature() {
        assert_eq!(
            signature(&b"blablabla"[..]),
            Err(Error(Code(&b"blablabla"[..], Tag)))
        );
    }

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
        assert_eq!(version(&b"12a"[..]), Err(Error(Code(&b"12a"[..], Alt))));
    }

    #[test]
    fn should_parse_gif() {
        let gif_data = include_bytes!("../fixtures/sample_1.gif");
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

}
