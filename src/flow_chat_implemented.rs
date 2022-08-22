pub fn implementation(m: u32) -> u32{
    let mut m = m;
    let mut d = 2;
    let mut n = 0;

    while m != 1 {
        while m % d != 0 {
            d += 1;
        }
        m /= d;
        n += 1;
    }

    if n == 0 {n += 1;}
    n
}
