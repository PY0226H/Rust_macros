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

#[macro_export]
macro_rules! my_try {
    ($expr:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => return Err(err.into()),
        }
    };
}

//my_ready! => Poll::Ready / Poll::Pending
#[macro_export]
macro_rules! my_ready {
    ($expr:expr) => {
        match $expr {
            std::task::Poll::Ready(val) => std::task::Poll::Ready(val),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    };
}
