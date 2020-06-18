mod decode;
mod encode;

pub use decode::decode;
pub use encode::encode;

const TABLE: [u32; 5] = [85 * 85 * 85 * 85, 85 * 85 * 85, 85 * 85, 85, 1];

#[cfg(test)]
mod tests {

    use super::{decode, encode};

    #[test]
    fn test() {
        let cases: &[&[u8]] = &[
            &[],
            &[1],
            &[2],
            &[1, 2, 3],
            &[0, 0, 0, 0],
            &[0, 0, 0, 0, 0],
            &[0, 0, 0, 0, 0],
            &[1, 2, 3, 4, 5],
        ];

        for case in cases {
            assert_eq!(
                decode(dbg!(&encode(dbg!(*case)))).unwrap().as_slice(),
                *case
            );
        }
    }
}
