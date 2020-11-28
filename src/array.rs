fn main() {
    let a: [isize; 4] = [1, 2, 3, 4];
    println!("{:?}", a); // => [1, 2, 3]
    for elm in &a {
        println!("{}", elm);
    }
}