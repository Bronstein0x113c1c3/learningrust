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

#[test]
fn testgcd(){
    assert_eq!(gcd(14, 15),1);
}
