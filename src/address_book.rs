use crate::contact::Contact;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct AddressBook {
    pub last_assigned_id: usize,
    contacts_map: HashMap<usize, Contact>,
    name_index: HashMap<String, Vec<usize>>,
    phone_index: HashMap<String, Vec<usize>>,
}

impl AddressBook {
    pub fn new() -> Self {
        Self {
            last_assigned_id: 0,
            contacts_map: HashMap::new(),
            name_index: HashMap::new(),
            phone_index: HashMap::new(),
        }
    }

    pub fn list_contacts_stdout(&self) {
        for (_, contact) in self.contacts_map.iter() {
            println!("{}", contact);
        }
    }

    // TODO: check for duplicate contacts here as well
    pub fn add_contact(&mut self, contact: Contact) {
        let contact_id = contact.get_id();
        self.contacts_map.insert(contact_id, contact.clone());
        self.update_all_indexes(
            contact.get_full_name(),
            contact.get_phone_number(),
            contact_id,
        );
        self.update_last_assigned_id(contact_id)
    }

    pub fn update_last_assigned_id(&mut self, last_assigned_id: usize) {
        self.last_assigned_id = last_assigned_id;
    }

    fn update_all_indexes(&mut self, full_name: String, phone_number: String, contact_id: usize) {
        self.update_name_index(full_name, contact_id);
        self.update_phone_index(phone_number, contact_id);
    }

    fn update_name_index(&mut self, full_name: String, contact_id: usize) {
        match self.name_index.get_mut(&full_name) {
            Some(contact_ids) => contact_ids.push(contact_id),
            None => {
                self.name_index.insert(full_name, vec![contact_id]);
            }
        }
    }

    fn update_phone_index(&mut self, phone_number: String, contact_id: usize) {
        match self.phone_index.get_mut(&phone_number) {
            Some(contact_ids) => contact_ids.push(contact_id),
            None => {
                self.phone_index.insert(phone_number, vec![contact_id]);
            }
        }
    }

    pub fn get_contact_by_id(&self, contact_id: &usize) -> Option<&Contact> {
        self.contacts_map.get(contact_id)
    }

    pub fn get_contacts_by_name(&self, name: &str) -> Vec<&Contact> {
        match self.name_index.get(name) {
            Some(contact_ids) => contact_ids
                .iter()
                .map(|contact_id| self.get_contact_by_id(contact_id).unwrap())
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn get_contacts_by_phone_number(&self, phone_number: &str) -> Vec<&Contact> {
        match self.phone_index.get(phone_number) {
            Some(contact_ids) => contact_ids
                .iter()
                .map(|contact_id| self.get_contact_by_id(contact_id).unwrap())
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn load_from_tsv<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let lines = BufReader::new(file).lines();
        let mut address_book = AddressBook::new();
        for (_, line) in lines.enumerate() {
            let line = line?;
            let contact = Contact::from_tsv_string(&line)?;
            address_book.add_contact(contact);
        }
        Ok(address_book)
    }

    pub fn append_contact_to_file<P: AsRef<Path>>(
        &self,
        path: P,
        contact: &Contact,
    ) -> io::Result<()> {
        // FIXME: data is not being appended to end of the file
        let mut file = OpenOptions::new().create(true).append(true).open(path)?;
        writeln!(file, "{}", contact.to_tsv_string())?;
        Ok(())
    }
}
