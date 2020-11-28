fn text(gift: &str) {
    if gift == "onion" { panic!("Iyaaaaaaaaaaaaaa!!!!"); }

    println!("I love {}s!!!!!", gift);
}

fn main() {
    text("ramen");
    text("onion");
}
