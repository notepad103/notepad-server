struct Con {
    url: &'static str,
}

#[allow(dead_code)]
const CONFIG: Con = Con {
    url: "1234567890@qq.com",
};

// fn main() {
//     let result = 'outer: loop {
//         println!("Outer loop");

//         'inner: loop {
//             println!("Inner loop-1");

//             loop {
//                 println!("Inner loop-2");
//                 break 'outer 20;
//             } //inner-2·loop.ends
//         } //inner-1 loop ends
//     }; //outer loop.ends

//     println!("Exited outer loop with result=.{}", result);
// }
