fn call_twice(greet: impl Fn(&str)) {
    greet("Alice");
    greet("Bob");
}

fn main() {
    let prefix = String::from("Hello");

    let greet = move |name: &str| {
        println!("{prefix}, {name}");
    };

    call_twice(greet);
}