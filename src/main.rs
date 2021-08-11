use std::io::{stdin, stdout, self, Error, Read, Write};
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
        println!("Amount Owed: {}\n", self.amount_owed);
    }

    fn edit(&mut self, name: Option<String>, amount_owed: Option<i32>) {
        match name {
            Some(s) => self.name = s,
            None => (),
        }

        match amount_owed {
            Some(s) => self.amount_owed = s,
            None => (),
        }
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

fn screen_clear() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"\nPress Enter to go back to the Main Menu.").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
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
    println!("Bills:");
    if bills.is_empty() {
        return Err("There are no available bills".to_owned());
    }

    for (key, value) in bills.iter() {
        println!("Bill id: {}", key);
        value.view();
    }
    Ok(())
}

fn remove_bill(bills: &mut HashMap<i32, Bill>) -> Result<(), Error> {
    input_print("Input bill id: ");
    let id: String;
    match user_input() {
        Ok(input) => id = input,
        Err(e) => return Err(e),
    }
    let id: i32 = id.trim().parse().unwrap();

    input_print("Are you sure you want to delete this bill(yes/no): ");
    let confirm: String;
    match user_input() {
        Ok(input) => confirm = input,
        Err(e) => return Err(e),
    }
    match confirm.to_lowercase().as_str() {
        "yes" =>  {bills.remove(&id);},
        "no" => (),
        _ => (),
    }
    Ok(())
}

fn edit_bill(bills: &mut HashMap<i32, Bill>) -> Result<(), Error>{
    let id: String;
    input_print("Input an id: ");
    match user_input() {
        Ok(input) => id = input,
        Err(e) => return Err(e),
    }
    let id: i32 = id.trim().parse().unwrap();

    println!("Edit: ");
    println!("1. Name");
    println!("2. Owed Amount");
    println!("3. Both");
    input_print(">");
    let choice: String;

    match user_input() {
        Ok(input) => choice = input,
        Err(e) => return Err(e),
    }

    match choice.as_str() {
        "1" => {
            let name: String;
            input_print("Input name: ");
            match user_input() {
                Ok(input) => name = input,
                Err(e) => return Err(e),
            }
            bills.get_mut(&id).unwrap().edit(Some(name), None);
        }

        "2" => {
            let amount: String;
            input_print("Input Owed Amount: ");
            match user_input() {
                Ok(input) => amount = input,
                Err(e) => return Err(e),
            }
            let amount: i32 = amount.trim().parse().unwrap();

            bills.get_mut(&id).unwrap().edit(None, Some(amount));
        }

        "3" => {    
            let name: String;
            input_print("Input name: ");
            match user_input() {
                Ok(input) => name = input,
                Err(e) => return Err(e),
            }
            let amount: String;
            input_print("Input Owed Amount: ");
            match user_input() {
                Ok(input) => amount = input,
                Err(e) => return Err(e),
            }
            let amount: i32 = amount.trim().parse().unwrap();
            bills.get_mut(&id).unwrap().edit(Some(name), Some(amount));
        }
        _ => println!("There is no such option"),
    }
    

    
    Ok(())
}


fn main() {
    let mut bills = HashMap::new();
    bills.insert(1, Bill::add("Example".to_owned(), 100));
    let mut uinput: String;

    loop {
        //clearing screen
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
                screen_clear();
                match add_bill(&mut bills) {
                    Err(e) => println!("Error: {:?}", e),
                    _ => ()
                }
            }

            Some(View) => {
                screen_clear();
                match list_bills(&mut bills) {
                   Err(e) => println!("{:?}", e),
                   _ => (),
                }
                pause();

            }
            Some(Remove) => {
                screen_clear();
                match remove_bill(&mut bills) {
                   Err(e) => println!("Error: {:?}", e),
                   _ => (),
                }
            }
            Some(Edit) => {
                screen_clear();
                match edit_bill(&mut bills) {
                    Err(e) => println!("Error: {:?}", e),
                    _ => (),
                }

            }
            Some(Exit) => {
                break;
            }

            None => println!("There is no such option."),
        }
        screen_clear();
    }
}

