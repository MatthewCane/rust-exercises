fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn age(age: i32) {
    println!("You are {}", age)
}

fn main() {
    hello("Matt");
    age(28);
}
