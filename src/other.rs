enum Color {
    Red { color: i32 },
    Green(i32),
    Blue(i32),
}

fn main() {
    let a = Color::Red { color: 10 };
    match a {
        Color::Red { color: c @ 1..1000 } => println!("Red: {}", c),
        Color::Red { color: c @ 1..100 } => println!("Red: {}", c),
        _ => println!("Other"),
    };
}
