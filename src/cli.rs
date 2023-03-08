use std::io::{self, Write};
use std::path::Path;

use crate::address_book::AddressBook;
use crate::contact::Contact;

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

fn search_by_phone_number(address_book: &AddressBook) -> Vec<&Contact> {
    let phone_number = get_input("Enter phone number: ");
    // TODO: print no contacts found when empty vector
    address_book.get_contacts_by_phone_number(&phone_number)
}

fn search_by_id(address_book: &AddressBook) -> Vec<&Contact> {
    // TODO: Don't use unwrap in this parsing
    let id = get_input("Enter ID: ").parse::<usize>().unwrap();
    // TODO: print no contacts found when empty vector
    address_book.get_contact_by_id(&id).into_iter().collect()
}

fn search_by_name(address_book: &AddressBook) -> Vec<&Contact> {
    let name = get_input("Enter Name: ");
    // TODO: print no contacts found when empty vector
    address_book.get_contacts_by_name(&name)
}

pub fn run() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <address_book_path>", args[0]);
        return;
    }

    let path = Path::new(&args[1]);
    let mut address_book = match AddressBook::load_from_tsv(path) {
        Ok(address_book) => address_book,
        Err(e) => {
            println!("Failed to load address book: {}", e);
            return;
        }
    };

    loop {
        print_prompt();
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim().to_lowercase();
        if command == "exit" {
            break;
        } else if command == "list" {
            list_contacts(&address_book);
        } else if command == "add" {
            add_contact(&mut address_book, path);
        } else if command == "find" {
            find_contact(&address_book);
        } else {
            println!("Invalid command");
        }
    }
}

fn print_prompt() {
    print!("> ");
}

fn list_contacts(address_book: &AddressBook) {
    address_book.list_contacts_stdout()
}

fn get_contact_details(address_book: &AddressBook) -> Contact {
    let mut first_name = String::new();
    let mut last_name = String::new();
    let mut address = String::new();
    let mut phone_number = String::new();
    println!("Enter contact details:");
    print!("First name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut first_name).unwrap();
    print!("Last name: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut last_name).unwrap();
    print!("Address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut address).unwrap();
    print!("Phone number: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut phone_number).unwrap();

    Contact::new(
        first_name.trim().to_string(),
        last_name.trim().to_string(),
        address.trim().to_string(),
        phone_number.trim().to_string(),
        address_book.last_assigned_id,
    )
}

fn add_contact(address_book: &mut AddressBook, path: impl AsRef<Path>) {
    // TODO: add result checking
    let contact = get_contact_details(&address_book);
    // TODO: add result checking
    address_book.add_contact(contact.clone());
    // TODO: add result checking
    address_book.append_contact_to_file(path, &contact);
    println!("Contact added successfully!");
}

fn find_contact(address_book: &AddressBook) {
    println!("Search by:");
    println!("1. Phone number");
    println!("2. Name");
    println!("3. ID");

    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let search_choice = input.trim().parse::<u8>().unwrap();

    let contacts = match search_choice {
        1 => search_by_phone_number(address_book),
        2 => search_by_name(address_book),
        3 => search_by_id(address_book),
        _ => {
            println!("Invalid choice: {}", search_choice);
            Vec::new()
        }
    };

    contacts.iter().for_each(|contact| println!("{}", contact));
}
