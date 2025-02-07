fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let v = vec[0]; // Clone the value
    vec.push(3);
    println!("{}", v);
}