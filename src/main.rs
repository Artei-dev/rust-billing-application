use std::io::{self, Error, Write};
use std::collections::HashMap;

enum MenuOption {
    Add,
    View,
    Remove,
    Edit,
    Exit,
}

struct Bill {
    name: String,
    amount_owed: i32,
}

impl Bill {
    fn add(name: String, amount_owed: i32) -> Self {
        Self {
            name,
            amount_owed,
        }
    }

    fn view(&self) {
        println!("Name: {}", self.name);
        println!("Amount Owed: {}", self.amount_owed);
    }
}

fn user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

fn print_menu() {
    println!("---Billing Application---");
    println!("1. Add Bill");
    println!("2. View all bills");
    println!("3. Remove Bill");
    println!("4. Edit a bill");
    println!("5. Exit Application");
    print!(">");
    io::stdout().flush().unwrap();
}

fn input_print(input: &str) {
    print!("{}", input);
    io::stdout().flush().unwrap();
}

fn choose_menu_option(input: &str) -> Option<MenuOption> {
    use MenuOption::*;
    match input {
        "1" => Some(Add),
        "2" => Some(View),
        "3" => Some(Remove),
        "4" => Some(Edit),
        "5" => Some(Exit),
        _ => None,
    }
}

fn last_key(bills: &HashMap<i32, Bill>) -> i32 {
    if bills.is_empty() {
        return 0;
    }

    let mut last_item_key: i32 = 0;
    for key in bills.keys() {
        if key > &last_item_key {
            last_item_key = key.to_owned();
        }
    }

    last_item_key
}

fn add_bill(bills: &mut HashMap<i32, Bill>) -> Result<(), Error> {
    let key = last_key(&bills) + 1;
    let name: String;
    let amount_owed: String;


    input_print("Input a name: ");
    match user_input() {
        Ok(value) => name = value,
        Err(e) => {
            return Err(e);
        } 
    }

    input_print("Input an owed amount: ");
    match user_input() {
        Ok(value) => amount_owed = value,
        Err(e) => {
            return Err(e);
        } 
    }

    let amount_owed: i32 = amount_owed.trim().parse().unwrap();
    
    bills.insert(key, Bill::add(name, amount_owed));
    Ok(())
}

fn list_bills(bills: &mut HashMap<i32, Bill>) -> Result<(), String> {
    if bills.is_empty() {
        return Err("There are no available bills".to_owned());
    }

    for (key, value) in bills.iter() {
        println!("\nid: {}", key);
        value.view();
    }
    Ok(())
}


fn main() {
    let mut bills = HashMap::new();
    bills.insert(1, Bill::add("Example".to_owned(), 100));
    let mut uinput: String;

    loop {
        print_menu();
        match user_input() {
            Ok(input) => uinput = input,
            Err(e) => {
                println!("Error: {:?}", e);
                continue;
            }
        }

        use MenuOption::*;
        match choose_menu_option(&uinput) {
            Some(Add) => {
                match add_bill(&mut bills) {
                    Err(e) => println!("Error: {:?}", e),
                    _ => ()
                }
            }

            Some(View) => {
                match list_bills(&mut bills) {
                   Err(e) => println!("{:?}", e),
                   _ => (),
                }

            }
            Some(Remove) => {

            }
            Some(Edit) => {

            }
            Some(Exit) => {
                break;
            }

            None => println!("There is no such option."),
        }
    }
}
