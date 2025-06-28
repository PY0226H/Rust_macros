use anyhow::Result;
use macros::my_vec;
fn main() -> Result<()> {
    let v: Vec<i32> = my_vec![
        "1".parse()?,
        "2".parse()?,
        "3".parse()?,
        "4".parse()?,
        "5".parse()?,
        "6".parse()?,
        "7".parse()?,
        "8".parse()?,
        "9".parse()?,
        "10".parse()?,
    ];
    println!("{:?}", v);
    Ok(())
}
