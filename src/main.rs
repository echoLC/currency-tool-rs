use currency_tool_rs::{get_default_currency, get_default_rate};
use std::fmt::Display;
use std::convert::TryFrom;

fn main() {
    println!("default currency:{}", get_default_currency());
    println!("default currency rate:{}", get_default_rate());
    
    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display {
        println!("Announcement! {}", ann);

        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("hello");

    {
        let string2 = String::from("world");
        let result = longest_with_an_announcement(&string1, &string2, String::from("hello"));

        println!("result is {}", result);
    }

    let s1: Box<str> = String::from("hello world").into();

    println!("{:?}", s1);
    let mut s = String::new();

    let update_string = |str| s.push_str(str);

    exec(update_string);

    fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
        f("hello")
    }

    enum_to_int_example(MyEnum::C as i32);
    enum_to_int_example(4);

    let str1 = generate_str();
    println!("{}", str1);
}

enum MyEnum {
  A = 1,
  B,
  C
}

impl TryFrom<i32> for MyEnum {
  type Error = ();

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    match value {
      x if x == MyEnum::A as i32 => Ok(MyEnum::A),      
      x if x == MyEnum::B as i32 => Ok(MyEnum::B),      
      x if x == MyEnum::C as i32 => Ok(MyEnum::C),
      _ => Err(())      
    }
  }  
}


fn enum_to_int_example(v: i32) {
  match v.try_into() {
    Ok(MyEnum::A) => println!("got A"),    
    Ok(MyEnum::B) => println!("got B"),    
    Ok(MyEnum::C) => println!("got C"),
    Err(_) => eprintln!("unknown number")    
  }      
}

fn generate_str () -> &'static str {
  let mut s = String::new();
  s.push_str("hello, world");

  Box::leak(s.into_boxed_str())
}
