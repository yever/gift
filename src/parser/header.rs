
#[derive(Debug, PartialEq)]
pub enum Version {
    GIF89a,
    GIF87a,
}

named!(signature, tag!("GIF"));

fn get_version(bytes: &[u8]) -> Result<Version, ()> {
    match bytes {
        b"87a" => Ok(Version::GIF87a),
        b"89a" => Ok(Version::GIF89a),
        _ => Err(()),
    }
}

named!(version<&[u8], Version>, map_res!(alt!(tag!("87a") | tag!("89a")), get_version));

#[cfg(test)]
mod tests {
    use nom::{Context::Code, Err::Error, ErrorKind::{Alt, Tag}};
    use super::*;

    #[test]
    fn signature_should_parse_gif() {
        assert_eq!(signature(&b"GIF89a"[..]), Ok((&b"89a"[..], &b"GIF"[..])));
    }

    #[test]
    fn signature_should_fail_on_invalid_signature() {
        assert_eq!(signature(&b"blablabla"[..]), Err(Error(Code(&b"blablabla"[..], Tag))));
    }

    #[test]
    fn should_parse_version_89a() {
        assert_eq!(version(&b"89a"[..]), Ok((&b""[..], Version::GIF89a)));
    }

    #[test]
    fn should_parse_version_87a() {
        assert_eq!(version(&b"87a"[..]), Ok((&b""[..], Version::GIF87a)));
    }

    #[test]
    fn should_fail_on_unknown_version() {
        assert_eq!(version(&b"12a"[..]), Err(Error(Code(&b"12a"[..], Alt))));
    }
}
