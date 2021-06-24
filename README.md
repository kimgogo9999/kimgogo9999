# Hello, World!

![언어](https://github-readme-stats.vercel.app/api/top-langs/?username=fn79&theme=tokyonight)

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
        language: ["Rust", "C#", "C++", "Java", "JavaScript", "GoLang", "F#", "Python", "한국어", "영어", "야옹이마을 언어", "BBangLang"]
    };
    hello
}
```

```rust
fn main() {
    hello::say::self_intro();
}

mod hello {
    pub mod say {
        use crate::Info;
        use std::cmp::Ordering;

        pub fn self_intro() {
            let x = Info {
                name: String::from("안녕"),
                age: 6,
                des: String::from("나는 빵켓이얌~!")
            };
            println!(
                "{} {} {} / {}",
                x.get_name(),
                x.get_age(),
                x.get_des(),
                age_(x.get_age())
            );
        }

        pub fn age_(age: i8) -> String {
            match age.cmp(&6) { // 값 확인
                Ordering::Less => String::from("저보다 작네요..?"),
                Ordering::Greater => String::from("저보다 크네요..?"),
                Ordering::Equal => { String::from("저랑 나이가 같네요!") },
            }
        }
    }
}

impl Info {
    fn get_name(&self) -> &String { &self.name }
    fn get_age(&self) -> i8 { self.age as i8 }
    fn get_des(&self) -> &String { &self.des }
}

struct Info { name: String, age: i8, des: String }
```

**러스트!**
