// enum CalcError {
//     DividedByZero,
//     DetectedNegative(i32),
// }

// fn division(dividened: i32, divisor: i32) -> Result<i32, CalcError> {
//     if divisor == 0 {
//         return Err(CalcError::DividedByZero);
//     }
//     if dividened < 0 {
//         return Err(CalcError::DetectedNegative(dividened));
//     }
//     if divisor < 0 {
//         return Err(CalcError::DetectedNegative(divisor));
//     }
//     Ok(dividened / divisor)
// }

// fn main() {
//     let answer = division(4, 2);
//     match answer {
//         Ok(value) => println!("answer is {}", value),
//         Err(err) => match err {
//             CalcError::DividedByZero => eprintln!("ゼロ除算です"),
//             CalcError::DetectedNegative(num) => eprintln!("{} は負の数です 負の数は使用できません", num),
//         },
//     }
// }

// fn main() {
//     let path = "./src/main.rs";
//     match std::fs::read_to_string(path) {
//         Ok(content) => print!("{}", content),
//         Err(why) => println!("{}", why),
//     }
// }

// fn run(path: String) {
//     match std::fs::read_to_string(path) {
//         Ok(content) => print!("{}", content),
//         Err(why) => println!("{}", why),
//     }
// }

// fn main() {
//     let mut args = std::env::args();
//     match args.nth(1) {
//         Some(path) => run(path),
//         None => println!("パスを指定してください"),
//     }
// }

// struct User {
//     tag: i32,
// }

// impl User {
//     fn new(num: i32) -> User {
//         User { tag: num }
//     }

//     fn print_tag(&self) -> i32 {
//         self.tag
//     }
// }

// fn main() {
//     let mut user = User::new(1);
//     assert_eq!(user.print_tag(), 1);
//     user.tag = 2;
//     assert_eq!(user.print_tag(), 2);
// }

// fn add_elem(target: &mut Vec<i32>, elem: i32) {
//     target.push(elem);
// }

// fn main() {
//     let mut list = vec![];
//     add_elem(&mut list, 1);
//     add_elem(&mut list, 2);
//     add_elem(&mut list, 3);
//     assert_eq!(list, vec![1, 2, 3]);
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result =longest(string1.as_str(), string2.as_str());
//         println!("logest string is {}", result);
//     }
// }

struct User<'a> {
    id: UserId,
    user_name: UserName<'a>,
}

impl<'a> User<'a> {
    fn new(user_name: &'a str) -> Self {
        User {
            id: UserId(1),
            user_name: UserName(user_name),
        }
    }
}

struct UserId(i32);
struct UserName<'a>(&'a str);

fn main() {
    let user = User::new("name");
    assert_eq!(user.user_name.0, "name");
    assert_eq!(user.id.0, 1);
}
