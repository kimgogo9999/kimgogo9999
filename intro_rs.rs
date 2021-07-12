fn main() {
    println!(
        "이름: {}\n나이: {}\n성별: {}\n사용하는 언어: {}개 ({:?})\n",
        hello().name,
        hello().age,
        hello().gender,
        hello().language.len(),
        hello().language
    )
}

struct SelfIntroduction<'a> {
    name: &'a str,
    age: i32,
    gender: &'a str,
    language: [&'a str; 3],
}

fn hello<'a>() -> SelfIntroduction<'a> {
    let hello = SelfIntroduction {
        name: "! 빵켓#0001",
        age: 6,
        gender: "남 (아마도)",
        language: ["Rust", "C#", "Python"],
    };
    hello
}
