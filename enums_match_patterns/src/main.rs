//演示if let 和 let else

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    California,
    Texas,
}
impl UsState {
    fn exist_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
            UsState::California => year >= 1850,
            UsState::Texas => year >= 1845,
        }
    }
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn describe_state_quarter(coin: &Coin) -> Option<String> {
    let Coin::Quarter(state) = coin else {
        return None;
    };

    if state.exist_in(1900) {
        Some(format!("{state:?} is pretty old, for America!"))
    } else {
        Some(format!("{state:?} is pretty young, for America!"))
    }
}

fn main() {
    println!("==== 1. if let 基础用法 ====");
    // only care some, not care None
    let config_max = Some(3u8);

    // use match
    match config_max {
        Some(max) => println!("[match] max valut is :{max}"),
        _ => (), //多余的模版代码
    }

    //use if let
    if let Some(max) = config_max {
        println!("[match] max valut is :{max}");
    }

    //当值为None时，if let 不会执行
    let empty: Option<u8> = None;
    if let Some(val) = empty {
        println!("this line would't print,{val}");
    }
    println!("None的情况被跳过");

    println!("\n===== 2. if let + else =====");

    let coins = vec![
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alabama),
        Coin::Quarter(UsState::Texas),
    ];

    let mut count = 0;
    for coin in &coins {
        if let Coin::Quarter(state) = coin {
            println!("发现州纪念币，来自{state:?}");
        } else {
            count += 1;
        }
    }

    println!("非Quarter coin 的数量为:{count}");

    println!("\n===== 3. let...else（快乐路径） =====");

    for coin in &coins {
        match describe_state_quarter(coin) {
            Some(msg) => println!("{msg}"),
            None => println!("{coin:?} is not quarter, pass"),
        }
    }

    println!("\n===== 4. 对比总结 =====");

    let my_opinion: Option<&str> = Some("Rust is great!");

    //match:穷尽所有分支
    let msg1 = match my_opinion {
        Some(s) => println!("{s}"),
        None => println!("None"),
    };
    println!("{msg1:?}");

    // if let:只关心一种模式
    if let Some(s) = my_opinion {
        println!("{s}")
    }

    //let...else
    fn extract_or_bail(opt: Option<&str>) -> &str {
        let Some(s) = opt else {
            panic!("没有值");
        };

        s
    }

    println!("let..else {}", extract_or_bail(my_opinion));

    //enum IPAddrKind{
    //V4(u8, u8, u8, u8),
    //V6(String)
    //}

    //let home = ErrorKind::V4(String::from( "127.0.0.1"));
    //let loopback = IPAddrKind::V6(String::from( "::1"));

    // fn values_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => {
    //             println!("good penny");
    //             1
    //         }
    //         Coin::Nickel => {
    //             println!("nice nickel");
    //             2
    //         }
    //         Coin::Dime => {
    //             println!("better dime");
    //             3
    //         }
    //         Coin::Quarter(UsState) => {
    //             println!("pretty quarter");
    //             4
    //         }
    //     }
    // }
    // let coin = Coin::Nickel;
    // values_in_cents(coin);

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         Some(x) => Some(x + 1),
    //         None => None,
    //     }
    // }

    // let x = 1;
    // let y = Some(x);
    // plus_one(y);
}
