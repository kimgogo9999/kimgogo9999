fn main() {
    let x = Intro { name: "빵켓".to_string(), age: 6, pvp: -1972, mcname: "BBangCat".to_string(), mic: false, description: "??".to_string() };

    match x.hello() {
        Some(t) => println!("{}", t),
        None => println!("당신은 휴먼입니까?")
    }
}

struct Intro { name: String, age: i32, pvp: i32, mcname: String, mic: bool, description: String }

impl Intro {
    fn hello(&self) -> Option<String> {
        _ => Some(format!("이름: {}\n나이: {}\nPVP 실력: {}/10\n마인크래프트 이름: {}\n마이크사용: {}\n설명: {}",
                          self.name, self.age, self.pvp, self.mcname, self.mic, self.description))
    }
}
