struct MyStruct {
    val: i32,
    action: Box<dyn Fn(i32) -> i32>,
}