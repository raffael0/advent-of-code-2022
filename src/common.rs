
pub fn to_num(str : &str) -> usize{
    usize::from_str_radix(str,10).unwrap()
}