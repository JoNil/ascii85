use crate::TABLE;
use std::borrow::Cow;
use std::convert::TryInto;

pub fn encode(input: &[u8]) -> String {
    let mut result = String::with_capacity(5 * (input.len() / 4 + 16));

    result.push_str("<~");

    for chunk in input.chunks(4) {
        let (chunk, count) = if chunk.len() == 4 {
            (Cow::from(chunk), 5)
        } else {
            let mut new_chunk = Vec::new();
            new_chunk.resize_with(4, || 0);
            new_chunk[..chunk.len()].copy_from_slice(chunk);
            (Cow::from(new_chunk), 5 - (4 - chunk.len()))
        };

        let number = u32::from_be_bytes(chunk.as_ref().try_into().expect("Internal Error"));

        for i in 0..count {
            let digit = (((number / TABLE[i]) % 85) + 33) as u8;
            result.push(digit as char);
        }
    }

    result.push_str("~>");

    result
}

#[cfg(test)]
mod tests {

    use super::encode;

    #[test]
    fn encode_test() {
        assert_eq!(
            "<~9jqo^F*2M7/c~>",
            encode(&[77, 97, 110, 32, 115, 117, 114, 101, 46]),
        );

        assert_eq!(
            r#"<~9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,O<DJ+*.@<*K0@<6L(Df-\0Ec5e;DffZ(EZee.Bl.9pF"AGXBPCsi+DGm>@3BB/F*&OCAfu2/AKYi(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolDIal(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG%G>uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c~>"#,
            encode(b"Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure."),
        );

        assert_eq!(
            std::str::from_utf8(&[b'<', b'~', 47, 99, 117, 117, 117, b'~', b'>']).unwrap(),
            encode(&[46, 3, 25, 180]),
        );
    }
}
