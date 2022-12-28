fn main() {
    let a = [1,2,3,4,5];
    println!("{}",a.iter().fold(10, |sum,x|sum+x*x));
}
