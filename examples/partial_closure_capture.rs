struct Message {
    subject: String,
    body: String,
}

fn main() {
    let message = Message {
        subject: String::from("Hello"),
        body: String::from("World"),
    };

    let print_subject = move || {
        println!("{}", message.subject);
    };

    println!("{}", message.body);
    print_subject();
}