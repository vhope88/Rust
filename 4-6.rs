fn area<T: std::ops::Mul<Output = T> + Copy>(r: T) -> T {
    r * r
}
fn main() {
    println!("{}", area(3));
    println!("{}", area(3.2));
}