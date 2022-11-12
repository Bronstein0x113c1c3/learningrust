// use core::num::dec2flt::number;
use std::io::Write;
use std::str::FromStr;
fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    return n;
}

fn main(){
    //that creates an vector of "numbers" (dynamic array).
    let mut numbers= Vec::new();



    //read arguments from command line, and skip the 1st element (maybe)
    for arg in std::env::args().skip(1){
        //push the numbers from this set of arguments to numbers vector 
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));    
    }
    // for x in numbers{
    //     print!("{} ",x);
    // }

    // println!();
    if numbers.len()==0{
        writeln!(std::io::stderr(),"Usage: gcd NUMBER ... ").unwrap();
        std::process::exit(1);
    }
    //create a mutable variable called d
    //because numbers is an array, so to iterate through pointer and change value of elements
    // we need to use "mut"
    let mut d = numbers[0];
    

    //iterate though elements of vector "numbers"
    for m in &numbers[1..]{
        d = gcd(d, *m);
    }

    println!("the gcd of {:?} is {} ",numbers,d)
}