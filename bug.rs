fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let v = &vec[0];
    vec.push(3); // This line causes undefined behavior
    println!("{}", v);
}