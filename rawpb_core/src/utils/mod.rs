/// 序列化变体类型数字
///
/// # Examples
///
/// ```
/// use rawpb_core::utils::serialize_variant;
///
/// assert_eq!(serialize_variant(0x3dd), hex::decode("dd07").unwrap().as_slice());
/// assert_eq!(serialize_variant(0x2e), hex::decode("2e").unwrap().as_slice());
/// assert_eq!(serialize_variant(0), hex::decode("00").unwrap().as_slice());
/// assert_eq!(serialize_variant(1), hex::decode("01").unwrap().as_slice());
/// assert_eq!(serialize_variant(0x213b4024), hex::decode("a480ed8902").unwrap().as_slice());
/// assert_eq!(serialize_variant(0xff), hex::decode("ff01").unwrap().as_slice());
/// ```
#[allow(dead_code)]
pub fn serialize_variant<T>(value: T) -> Vec<u8>
where
    T: std::fmt::Binary + std::marker::Copy,
{
    use std::iter::FromIterator;
    let _cols = format!("{:b}", value)
        .chars()
        .into_iter()
        .collect::<Vec<char>>();
    let mut _str = _cols
        .rchunks(7)
        .into_iter()
        .map(|x| String::from_iter(x.iter()))
        .collect::<Vec<String>>();
    let _end = u8::from_str_radix(&_str.pop().unwrap(), 2).unwrap();
    let mut _ret = _str
        .into_iter()
        .map(|mut x| {
            x.insert(0, '1'); // 在最前面插入符号位1
            u8::from_str_radix(&x, 2).unwrap()
        })
        .collect::<Vec<u8>>();
    _ret.push(_end);

    _ret
}

/// 反序列化变体类型数字
///
/// # Examples
///
/// ```
/// use rawpb_core::utils::deserialize_variant;
///
/// let a: i32 = deserialize_variant(hex::decode("dd07").unwrap());
/// assert_eq!(a, 0x3dd);
/// let a: u64 = deserialize_variant(hex::decode("2e").unwrap());
/// assert_eq!(a, 0x2e);
/// let a: i8 = deserialize_variant(hex::decode("00").unwrap());
/// assert_eq!(a, 0);
/// let a: u8 = deserialize_variant(hex::decode("01").unwrap());
/// assert_eq!(a, 1);
/// let a: u32 = deserialize_variant(hex::decode("a480ed8902").unwrap());
/// assert_eq!(a, 0x213b4024);
/// let a: i32 = deserialize_variant(hex::decode("ff01").unwrap());
/// assert_eq!(a, 0xff);
/// ```
#[allow(dead_code)]
pub fn deserialize_variant<T>(value: Vec<u8>) -> T
where
    T: std::marker::Copy
        + std::ops::Shl<Output = T>
        + std::ops::BitOr<Output = T>
        + std::convert::TryFrom<i32>
        + std::convert::TryFrom<u8>,
{
    let mut val = value;

    if val.len() <= 0 {
        panic!("deserialize error..");
    }
    let _end = val.pop().unwrap();
    let _ret = val
        .into_iter()
        .rev()
        .map(|x| x & 0x7f)
        .fold(T::try_from(_end), |acc, x| {
            if let (Ok(a), Ok(b), Ok(c)) = (acc, T::try_from(7), T::try_from(x)) {
                Ok((a << b) | c)
            } else {
                panic!("deserialize error..");
            }
        });
    if let Ok(ret) = _ret {
        ret
    } else {
        panic!("deserialize error..");
    }
}
