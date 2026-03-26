struct Con<'a> {
    url: &'a str,
}

#[allow(dead_code)]
const CONFIG: Con = Con {
    url: "1234567890@qq.com",
};
