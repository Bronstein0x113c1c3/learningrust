// fn fibonacci(i:i32)->Result<i64,i64>{
//     if i<=0{
//         return Err(0);
//     }
//     else if i<=2{
//         return Ok(1);
//     }
//     else{
//         match fibonacci(i-2) {
//             Ok(x)=>{
//                 match fibonacci(i-1) {
//                     Ok(y)=>{
//                         return Ok(x+y);
//                     },
//                     Err(k )=>{
//                         return Err(k);
//                     }
                    
//                 }
//             },
//             Err(x)=>{
//                 return Err(x);
//             }
//         }
//     }
// }
fn fibonacci(i:i32)->i64{
    if i<=0{
        return 0;
    }
    else if i<=2{
        return 1;
    }
    else{
        return fibonacci(i-1)+fibonacci(i-2);
    }
    
}
fn main() {
    // let fib = fibonacci(10);
    // match fib {
    //     Ok(x) =>{
    //         println!("{}", x);
    //     }
    //     Err(x) =>{
    //         println!("{}", x);
    //     }
    // }
    for i in 0..11{
        println!("{} ",fibonacci(i));
    }
}
