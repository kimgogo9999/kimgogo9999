fn main() {
    let owo = |x: Vec<&str>| x.iter().map(|x| x.to_string() + " ").collect::<String>();
    println!(
        "Name: {}\nAge: {}\nGender: {}\nLanguage: {}\nEditor: {}",
        hello().name,
        hello().age,
        hello().gender,
        owo(hello().language),
        owo(hello().editor)
    )
}

struct SelfIntroduction<'a> {
    name: &'a str,
    age: i32,
    gender: &'a str,
    language: Vec<&'a str>,
    editor: Vec<&'a str>,
}

fn hello<'a>() -> SelfIntroduction<'a> {
    let hello = SelfIntroduction {
        name: "Bread Cat",
        age: 6,
        gender: "Male",
        language: vec!["Rust", "C#", "Python"],
        editor: vec!["IntelliJ", "vscode", "notepad"],
    };
    hello
}
