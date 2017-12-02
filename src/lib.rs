pub fn inverse_captcha_part_one(captcha: &[u8]) -> Result<u64, &'static str> {
    let mut captcha_shift = Vec::with_capacity(captcha.len());
    unsafe { captcha_shift.set_len(captcha.len()) }
    captcha_shift[0] = captcha[captcha.len() - 1];
    captcha_shift[1..captcha.len()].copy_from_slice(&captcha[0..captcha.len() - 1]);
    let mut sum: u64 = 0;
    for (x, _) in captcha.iter().zip(captcha_shift.iter()).filter(|&(x, y)| x == y) {
        sum += ascii_to_digit(*x)?;
    }
    Ok(sum)
}

pub fn inverse_captcha_part_two(captcha: &[u8]) -> Result<u64, &'static str> {
    let mut captcha_shift = Vec::with_capacity(captcha.len());
    unsafe { captcha_shift.set_len(captcha.len()) }
    captcha_shift[0..captcha.len()/2].copy_from_slice(&captcha[captcha.len()/2..captcha.len()]);
    captcha_shift[captcha.len()/2..captcha.len()].copy_from_slice(&captcha[0..captcha.len()/2]);
    let mut sum: u64 = 0;
    for (x, _) in captcha.iter().zip(captcha_shift.iter()).filter(|&(x, y)| x == y) {
        sum += ascii_to_digit(*x)?;
    }
    Ok(sum)
}

fn ascii_to_digit(ascii: u8) -> Result<u64, &'static str> {
    match ascii {
        b'0' => Ok(0),
        b'1' => Ok(1),
        b'2' => Ok(2),
        b'3' => Ok(3),
        b'4' => Ok(4),
        b'5' => Ok(5),
        b'6' => Ok(6),
        b'7' => Ok(7),
        b'8' => Ok(8),
        b'9' => Ok(9),
        _ => Err("Not a valid digit"),
    }
}

#[cfg(test)]
mod tests {
    use ::*;

    #[test]
    fn test_inverse_captcha_part_one() {
        assert_eq!(inverse_captcha_part_one("1122".as_bytes()), Ok(3));
        assert_eq!(inverse_captcha_part_one("1111".as_bytes()), Ok(4));
        assert_eq!(inverse_captcha_part_one("1234".as_bytes()), Ok(0));
        assert_eq!(inverse_captcha_part_one("91212129".as_bytes()), Ok(9));
    }

    #[test]
    fn test_inverse_captcha_part_two() {
        assert_eq!(inverse_captcha_part_two("1212".as_bytes()), Ok(6));
        assert_eq!(inverse_captcha_part_two("1221".as_bytes()), Ok(0));
        assert_eq!(inverse_captcha_part_two("123425".as_bytes()), Ok(4));
        assert_eq!(inverse_captcha_part_two("123123".as_bytes()), Ok(12));
        assert_eq!(inverse_captcha_part_two("12131415".as_bytes()), Ok(4));
    }
}
