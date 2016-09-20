fn main() {
    let argv: String = std::env::args().skip(1)
        .fold("".to_string(), |all, item| all + " '" + &item + "'");
    println!("{}", &argv);
}
