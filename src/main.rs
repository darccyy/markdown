fn main() {
    let file = include_str!("../../recipes/recipes/pancakes.md");
    let md = markdown::parse(file).unwrap();
    println!("{:#?}", md);
}
