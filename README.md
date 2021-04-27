# Hello, World!

![언어](https://github-readme-stats.vercel.app/api/top-langs/?username=fn79&theme=onedark)


# 안내

- 1일 1 레포를 도전하고 있지만, 조금 힘들거같네요.
- 할수있는 언어는 다음과 같습니다.

```rust
fn main() {
    println!("이름: {}\n나이: {}\n성별: {}\n인성: {}\n할수있는 언어: {}개\n", hello().name, hello().age, hello().gender, hello().insung, hello().language.len())
}

struct SelfIntroduction {
    name: String,
    age: i32,
    gender: String,
    insung: i32,
    language: [&'static str; 14]
}

fn hello() -> SelfIntroduction {
    let hello = SelfIntroduction {
        name: String::from("빵켓"),
        age: 6,
        gender: String::from("???"),
        insung: 1972,
        language: ["C#", "C++", "Java", "Rust", "JavaScript", "GoLang", "F#", "ASM", "Fortran", "Python", "한국어", "영어", "야옹이마을 언어", "BBangLang"]
    };
    hello
}
```

