fn main() {
    let sus = regexTest::find(String::from("lorem ipsum dolor sit amet."), String::from("lor"), Vec::new());
    println!("{:?}", sus);
}