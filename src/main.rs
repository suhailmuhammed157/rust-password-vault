mod passwords;

use std::process::exit;

use passwords::Service;

use crate::passwords::get_passwords_from_file;
use crate::passwords::prompt;

fn clear() {
    print!("{}[2J", 27 as char);
}

fn main() {
    clear();
    let ascii_art = r#"
   '##::::'##:'##:::'##::::::::::'##::::'##::::'###::::'##::::'##:'##:::::::'########:
    ###::'###:. ##:'##::::::::::: ##:::: ##:::'## ##::: ##:::: ##: ##:::::::... ##..::
    ####'####::. ####:::::::::::: ##:::: ##::'##:. ##:: ##:::: ##: ##:::::::::: ##::::
    ## ### ##:::. ##::::'#######: ##:::: ##:'##:::. ##: ##:::: ##: ##:::::::::: ##::::
    ##. #: ##:::: ##::::........:. ##:: ##:: #########: ##:::: ##: ##:::::::::: ##::::
    ##:.:: ##:::: ##::::::::::::::. ## ##::: ##.... ##: ##:::: ##: ##:::::::::: ##::::
    ##:::: ##:::: ##:::::::::::::::. ###:::: ##:::: ##:. #######:: ########:::: ##::::
    ..:::::..:::::..:::::::::::::::::...:::::..:::::..:::.......:::........:::::..:::::
    "#;

    println!("{ascii_art}");
    loop {
        println!("Password manager menu: ");
        println!("1. Add new entry ");
        println!("2. List all entries ");
        println!("3. Search entry ");
        println!("4. Quit ");

        let mut user_choice = String::new();

        std::io::stdin().read_line(&mut user_choice).unwrap();

        match user_choice.trim() {
            "1" => {
                clear();
                let new_service_entry = Service::create_service(
                    prompt("Enter service name: "),
                    prompt("Enter username: "),
                    prompt("Enter password"),
                );
                new_service_entry.write_to_file();
                println!("Service with username and password added successfully!");
            }
            "2" => {
                clear();
                let services = get_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error while getting all services: {}", err);
                    let dummy_service = vec![Service::new(); 1];
                    dummy_service
                });

                for item in &services {
                    println!(
                        "Service: {} \n username: {} \n password: {}",
                        item.service, item.username, item.password
                    );
                }
            }
            "3" => {
                clear();
                let services = get_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error while getting passwords: {}", err);
                    let dummy_service = vec![Service::new(); 1];
                    dummy_service
                });

                let search_text = prompt("Search: ");
                for item in &services {
                    if search_text == item.service {
                        println!(
                            "Service: {} \n username: {} \n password: {}",
                            item.service, item.username, item.password
                        );
                    }
                }
            }
            "4" => {
                clear();
                println!("BYE!");
                exit(0);
            }
            _ => {
                eprintln!("Invalid Entry");
            }
        }
    }
}
