use super::SubBlocks;
use nom::{
    le_u8,
    Context::Code,
    Err::{Error, Incomplete},
    ErrorKind, IResult, Needed,
};

fn non_empty_subblock(input: &[u8]) -> IResult<&[u8], &[u8]> {
    match le_u8(input) {
        Ok((_, 0)) => Err(Error(Code(input, ErrorKind::Custom(0)))),
        Ok((_, n)) => take!(input, n + 1),
        Err(err) => Err(err),
    }
}

pub fn data_subblocks(input: &[u8]) -> IResult<&[u8], SubBlocks> {
    let mut i = 0;

    while let Ok((_, subblock)) = non_empty_subblock(&input[i..]) {
        i += subblock.len();
    }

    // the index should point to the terminating 0
    match input[i] {
        0 => map!(input, take!(i + 1), |data: &[u8]| SubBlocks(data)),
        _ => Err(Error(Code(input, ErrorKind::Custom(0)))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_non_empty_subblock() {
        let data = &[0][..];
        assert_eq!(
            non_empty_subblock(data),
            Err(Error(Code(data, ErrorKind::Custom(0))))
        );

        let data = &[1, 2, 3][..];
        assert_eq!(non_empty_subblock(data), Ok((&[3][..], &[1, 2][..])));

        let data = &[2, 3, 4, 5][..];
        assert_eq!(non_empty_subblock(data), Ok((&[5][..], &[2, 3, 4][..])));

        let data = &[5][..];
        assert_eq!(non_empty_subblock(data), Err(Incomplete(Needed::Size(6))));
    }

    #[test]
    fn should_parse_data_subblocks() {
        let data = &[0][..];
        assert_eq!(data_subblocks(data), Ok((&[][..], SubBlocks(&[0][..]))));

        let data = &[0, 1, 2, 3][..];
        assert_eq!(
            data_subblocks(data),
            Ok((&[1, 2, 3][..], SubBlocks(&[0][..])))
        );

        let data = &[1, 255, 0][..];
        assert_eq!(
            data_subblocks(data),
            Ok((&[][..], SubBlocks(&[1, 255, 0][..])))
        );

        let data = &[1, 255, 2, 255, 255, 0][..];
        assert_eq!(
            data_subblocks(data),
            Ok((&[][..], SubBlocks(&[1, 255, 2, 255, 255, 0][..])))
        );

        let data = &[2, 255, 255, 0, 1, 2, 3][..];
        assert_eq!(
            data_subblocks(data),
            Ok((&[1, 2, 3][..], SubBlocks(&[2, 255, 255, 0][..])))
        );

        let data = &[5, 6][..];
        assert_eq!(
            data_subblocks(data),
            Err(Error(Code(data, ErrorKind::Custom(0))))
        );
    }
}
