# Hello, World!


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
    println!("이름: {}\n나이: {}\n성별: {}\n사용하는 언어: {}개 ({:?})\n", hello().name, hello().age, hello().gender, hello().language.len(), hello().language)
}

struct SelfIntroduction<'a> { name: &'a str, age: i32, gender: &'a str, language: [&'a str; 3] }

fn hello<'a>() -> SelfIntroduction<'a> {
    let hello = SelfIntroduction {
        name: "! 빵켓#0001",
        age: 6,
        gender: "남 (아마도)",
        language: ["Rust", "C#", "Python"]
    };
    hello
}
```
