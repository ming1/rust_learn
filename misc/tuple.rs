fn main() {
    let mut a = 1;
    let mut b = 1;

    {
        let c = (&mut a, &b);

        *c.0 = 2;
        *c.1 = 3;

        println!("{:?}", c);
    }
    println!("a {} b {}", a, b);
}
