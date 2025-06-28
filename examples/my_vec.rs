use anyhow::Result;

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

// my_vec! = my_vec!{1,2,3}; //Vec<i32>
#[macro_export]
macro_rules! my_vec {
    () =>{
        Vec::new()
    };
    ($ele:expr; $n:expr) => {
        std::vec::from_elem($ele, $n)
    };
    ($($x:expr),+ $(,)?) => {
        // {
        //     let mut temp_vec = Vec::new();
        //     $(
        //         temp_vec.push($x);
        //     )*
        //     temp_vec
        // }
        <[_]>::into_vec(Box::new([$($x),*]))
    };
}
