use std::error::Error;

fn decode_digit(digit: u8, counter: &mut usize, chunk: &mut u32, result: &mut Vec<u8>) {
    const TABLE: [u32; 5] = [85 * 85 * 85 * 85, 85 * 85 * 85, 85 * 85, 85, 1];

    let byte = digit - 33;

    *chunk += byte as u32 * TABLE[*counter];

    if *counter == 4 {
        result.extend_from_slice(&chunk.to_be_bytes());
        *chunk = 0;
        *counter = 0;
    } else {
        *counter += 1;
    }
}

pub fn decode(input: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut result = Vec::with_capacity(4 * (input.len() / 5 + 16));

    let mut counter = 0;
    let mut chunk = 0;

    for digit in input
        .split("<~")
        .nth(1)
        .ok_or("No leading <~")?
        .rsplit("~>")
        .nth(1)
        .ok_or("No trailing ~>")?
        .bytes()
        .filter(|c| !c.is_ascii_whitespace())
    {
        if digit == b'z' {
            if counter == 0 {
                result.extend_from_slice(&[0, 0, 0, 0]);
            } else {
                return Err("Missaligned z in input".into());
            }
        }

        if digit < 33 || digit > 117 {
            return Err("Input char is out of range for Ascii85".into());
        }

        decode_digit(digit, &mut counter, &mut chunk, &mut result);
    }

    let mut to_remove = 0;

    while counter != 0 {
        decode_digit(b'u', &mut counter, &mut chunk, &mut result);
        to_remove += 1;
    }

    result.drain((result.len() - to_remove)..result.len());

    Ok(result)
}

#[cfg(test)]
mod tests {

    use super::decode;

    #[test]
    fn decode_test() {
        assert_eq!(
            decode("<~9jqo^F*2M7/c~>").unwrap(),
            [77, 97, 110, 32, 115, 117, 114, 101, 46],
        );

        assert!(
            decode(r#"
                <~9jqo^BlbD-BleB1DJ+*+F(f,q/0JhKF<GL>Cj@.4Gp$d7F!,L7@<6@)/0JDEF<G%<+EV:2F!,
                O<DJ+*.@<*K0@<6L(Df-\0Ec5e;DffZ(EZee.Bl.9pF"AGXBPCsi+DGm>@3BB/F*&OCAfu2/AKY
                i(DIb:@FD,*)+C]U=@3BN#EcYf8ATD3s@q?d$AftVqCh[NqF<G:8+EV:.+Cf>-FD5W8ARlolDIa
                l(DId<j@<?3r@:F%a+D58'ATD4$Bl@l3De:,-DJs`8ARoFb/0JMK@qB4^F!,R<AKZ&-DfTqBG%G
                >uD.RTpAKYo'+CT/5+Cei#DII?(E,9)oF*2M7/c~>"#
            ).unwrap().iter().zip(
                b"Man is distinguished, not only by his reason, but by this singular passion from other animals, which is a lust of the mind, that by a perseverance of delight in the continued and indefatigable generation of knowledge, exceeds the short vehemence of any carnal pleasure."
                    .iter()
            ).all(|(a, b)| a == b)
        );

        assert_eq!(
            decode(std::str::from_utf8(&[b'<', b'~', 47, 99, 117, 117, 117, b'~', b'>']).unwrap()).unwrap(),
            [46, 3, 25, 180]
        );
    }
}