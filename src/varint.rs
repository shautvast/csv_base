const SLOT_2_0: u64 = 0x001fc07f;
const SLOT_4_2_0: u64 = 0xf01fc07f;

/// varints as implemented in `SQLite`

pub fn write(value: i64) -> Vec<u8> {
    let mut v = value;
    if (v & ((0xff00_0000) << 32)) == 0 {
        if v == 0 {
            return vec![0];
        }
        let mut result = Vec::new();
        while v != 0 {
            result.push(((v & 0x7f) | 0x80) as u8);
            v >>= 7;
        }
        result[0] &= 0x7f;

        result.reverse();
        result
    } else {
        let mut result = vec![0_u8; 9];
        result[8] = v as u8;
        v >>= 8;
        for i in (0..=7).rev() {
            result[i] = ((v & 0x7f) | 0x80) as u8;
            v >>= 7;
        }
        result
    }
}

pub fn read(data: Vec<u8>) -> u64 {
    let mut a = data[0] as u64;
    if (data[0] as i8) >= 0 {
        return a;
    }

    let mut b = data[1] as u64;
    if (b & 0x80) == 0 {
        return ((a & 0x7f) << 7) | b;
    }

    a = (a << 14) | data[2] as u64;
    if (a & 0x80) == 0 {
        a &= SLOT_2_0;
        b = (b & 0x7f) << 7;
        a |= b;
        return a;
    }

    a &= SLOT_2_0;
    b = b << 14;
    b |= data[3] as u64;
    if (b & 0x80) == 0 {
        b &= SLOT_2_0;
        a = (a << 7) | b;
        return a;
    }

    b &= SLOT_2_0;
    let mut s = a;
    a = a << 14;
    let m = data[4] as u64;
    a |= m;
    if (a & 0x80) == 0 {
        b = b << 7;
        a |= b;
        s = s >> 18;
        return (s << 32) | a;
    }

    s = (s << 7) | b;
    b = (b << 14) | data[5] as u64;
    if (b & 0x80) == 0 {
        a &= SLOT_2_0;
        a = (a << 7) | b;
        s = s >> 18;
        return (s << 32) | a;
    }

    a = a << 14;
    a |= data[6] as u64;
    if (a & 0x80) == 0 {
        a &= SLOT_4_2_0;
        b &= SLOT_2_0;
        b = b << 7;
        a |= b;
        s = s >> 11;
        return (s << 32) | a;
    }

    a &= SLOT_2_0;
    b = (b << 14) | data[7] as u64;
    if (b & 0x80) == 0 {
        b &= SLOT_4_2_0;
        a = (a << 7) | b;
        s = s >> 14;
        return (s << 32) | a;
    }

    a = a << 15;
    a |= data[8] as u64;
    b &= SLOT_2_0;
    b = b << 8;
    a |= b;
    s = s << 14;
    b = m;
    b &= 0x7f;
    b = b >> 3;
    s |= b;
    (s << 32) | a
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0() {
        assert_eq!(0, read(write(0)));
    }

    #[test]
    fn test_127() {
        assert_eq!(127, read(write(127)));
    }
    #[test]
    fn test_m127() {
        assert_eq!(398639861, read(write(398639861)));
    }
}
