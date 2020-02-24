fn main() {
    let refers_nothing = dangle();

    println!("{}", refers_nothing);
}

fn dangle() -> $String { // invalid reference pass
    let s = String::from("hello");

    $s // invalid return 
}
