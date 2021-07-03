# Hello, World!


![Language](https://github-readme-stats.vercel.app/api/top-langs/?username=fn79&&layout=compact&langs_count=10&theme=nightowl&custom_title=Language)

<center><img src="/rs_icon.png"></center>

```rs
  ____   ____                             _____        _   
 |  _ \ |  _ \                           / ____|      | |  
 | |_) || |_) |  __ _  _ __    __ _     | |      __ _ | |_ 
 |  _ < |  _ <  / _` || '_ \  / _` |    | |     / _` || __|
 | |_) || |_) || (_| || | | || (_| |    | |____| (_| || |_ 
 |____/ |____/  \__,_||_| |_| \__, |     \_____|\__,_| \__|
                               __/ |                       
                              |___/                        
```

```rust
fn main() {
    println!("이름: {}\n나이: {}\n성별: {}\n인성: {}\n할수있는 언어: {}개\n", hello().name, hello().age, hello().gender, hello().insung, hello().language.len())
}

struct SelfIntroduction<'a> { name: String, age: i32, gender: String, insung: i32, language: [&'a str; 4] }

fn hello() -> SelfIntroduction<'static> {
    let hello = SelfIntroduction {
        name: String::from("! 빵켓#0001"),
        age: 6,
        gender: String::from("남"),
        insung: 1972,
        language: ["Rust", "C#", "Python", "헤으응 언어 (?)"]
    };
    hello
}
```
