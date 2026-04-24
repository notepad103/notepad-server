fn consume<F>(mut f: F, x: i32) -> i32
where
    F: FnMut(i32) -> i32,
{
    f(x)
}

fn main() {
    let mut y = 10;
    let add_y = |x: i32| {
        y += x;
        y
    };
    let b = consume(add_y, 1);
    println!("{}", b);
}
