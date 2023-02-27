use crate::contact::Contact;
use std::collections::HashMap;

#[derive(Debug)]
struct AddressBook {
    last_assigned_id: usize,
    contacts_map: HashMap<usize, Contact>,
    name_index: HashMap<String, usize>,
    phone_index: HashMap<String, usize>,
}

impl AddressBook {
    fn new() -> Self {
        Self {
            last_assigned_id: 0,
            contacts_map: HashMap::new(),
            name_index: HashMap::new(),
            phone_index: HashMap::new(),
        }
    }

    fn add_contact(&mut self, contact: Contact) {
        let contact_id = contact.get_id();
        self.contacts_map.insert(contact_id, contact.clone());
        self.update_all_indexes(
            contact.get_full_name(),
            contact.get_phone_number(),
            contact_id,
        );
        self.update_last_assigned_id(contact_id)
    }

    fn update_last_assigned_id(&mut self, last_assigned_id: usize) {
        self.last_assigned_id = last_assigned_id;
    }

    fn update_all_indexes(&mut self, full_name: String, phone_number: String, contact_id: usize) {
        self.update_name_index(full_name, contact_id);
        self.update_phone_index(phone_number, contact_id);
    }

    fn update_name_index(&mut self, full_name: String, contact_id: usize) {
        self.name_index.insert(full_name, contact_id);
    }

    fn update_phone_index(&mut self, phone_number: String, contact_id: usize) {
        self.phone_index.insert(phone_number, contact_id);
    }

    fn get_contact_by_id(&self, contact_id: usize) -> Option<&Contact> {
        self.contacts_map.get(&contact_id)
    }

    fn get_contact_by_name(&self, name: &str) -> Option<&Contact> {
        if let Some(contact_id) = self.name_index.get(name) {
            return self.contacts_map.get(contact_id);
        }
        None
    }

    fn get_contact_by_phone_number(&self, phone_number: &str) -> Option<&Contact> {
        if let Some(contact_id) = self.phone_index.get(phone_number) {
            return self.contacts_map.get(contact_id);
        }
        None
    }

    fn delete_contact_by_id(&mut self, contact_id: usize) -> Option<(usize, Contact)> {
        if let Some((_, removed_contact)) = self.contacts_map.remove_entry(&contact_id) {
            self.name_index.remove(&removed_contact.get_full_name());
            self.phone_index.remove(&removed_contact.get_phone_number());
            return Some((contact_id, removed_contact));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_address_book() {
        let ab = AddressBook::new();
        assert_eq!(ab.last_assigned_id, 0);
        assert!(ab.contacts_map.is_empty());
        assert!(ab.name_index.is_empty());
        assert!(ab.phone_index.is_empty());
    }

    #[test]
    fn test_update_last_assigned_id() {
        let mut address_book = AddressBook::new();
        address_book.update_last_assigned_id(42);
        assert_eq!(address_book.last_assigned_id, 42);
    }

    #[test]
    fn test_update_name_index() {
        let mut address_book = AddressBook::new();
        let contact = Contact::new(
            "Sebastian".to_string(),
            "Vettel".to_string(),
            "Aston Martin Cognizant Formula One Team, Silverstone, UK".to_string(),
            "1234567890".to_string(),
            0,
        );
        address_book.update_name_index(contact.get_full_name(), contact.get_id());
        assert_eq!(address_book.name_index.len(), 1);
        assert_eq!(address_book.name_index.get("Sebastian Vettel"), Some(&1));
    }

    #[test]
    fn test_update_phone_index() {
        let mut address_book = AddressBook::new();
        let contact = Contact::new(
            "Charles".to_string(),
            "Leclerc".to_string(),
            "Scuderia Ferrari Mission Winnow, Maranello, Italy".to_string(),
            "1234567890".to_string(),
            0,
        );
        address_book.update_phone_index(contact.get_phone_number(), contact.get_id());
        assert_eq!(address_book.phone_index.len(), 1);
        assert_eq!(address_book.phone_index.get("1234567890"), Some(&1));
    }

    #[test]
    fn test_update_all_indexes() {
        let mut address_book = AddressBook::new();
        let contact = Contact::new(
            "Daniel".to_string(),
            "Ricciardo".to_string(),
            "McLaren Racing Limited, Woking, UK".to_string(),
            "1234567890".to_string(),
            0,
        );
        address_book.update_all_indexes(
            contact.get_full_name(),
            contact.get_phone_number(),
            contact.get_id(),
        );
        assert_eq!(address_book.name_index.len(), 1);
        assert_eq!(address_book.name_index.get("Daniel Ricciardo"), Some(&1));
        assert_eq!(address_book.phone_index.len(), 1);
        assert_eq!(address_book.phone_index.get("1234567890"), Some(&1));
    }

    #[test]
    fn test_add_contact() {
        let mut address_book = AddressBook::new();

        let contact1 = Contact::new(
            "Fernando".to_string(),
            "Alonso".to_string(),
            "8 Place de la Concorde, Paris".to_string(),
            "987654321".to_string(),
            0,
        );
        address_book.add_contact(contact1.clone());
        assert_eq!(address_book.contacts_map.len(), 1);
        assert_eq!(address_book.name_index.len(), 1);
        assert_eq!(address_book.phone_index.len(), 1);
        assert_eq!(
            address_book.contacts_map.get(&contact1.get_id()),
            Some(&contact1)
        );
        assert_eq!(
            address_book.name_index.get(&contact1.get_full_name()),
            Some(&contact1.get_id())
        );
        assert_eq!(
            address_book.phone_index.get(&contact1.get_phone_number()),
            Some(&contact1.get_id())
        );
    }

    #[test]
    fn test_get_contact_by_id() {
        let mut address_book = AddressBook::new();
        let contact1 = Contact::new(
            "Lewis".to_string(),
            "Hamilton".to_string(),
            "Some address".to_string(),
            "123456".to_string(),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact1.clone());

        let contact2 = Contact::new(
            "Max".to_string(),
            "Verstappen".to_string(),
            "Some other address".to_string(),
            "789012".to_string(),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact2.clone());

        assert_eq!(
            address_book.get_contact_by_id(contact1.get_id()),
            Some(&contact1)
        );
        assert_eq!(
            address_book.get_contact_by_id(contact2.get_id()),
            Some(&contact2)
        );
        assert_eq!(address_book.get_contact_by_id(999), None);
    }

    #[test]
    fn test_get_contact_by_name() {
        let mut address_book = AddressBook::new();
        let contact1 = Contact::new(
            "Lewis".to_string(),
            "Hamilton".to_string(),
            "Some address".to_string(),
            "123456789".to_string(),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact1.clone());
        let contact2 = Contact::new(
            "Max".to_string(),
            "Verstappen".to_string(),
            "Some address".to_string(),
            "987654321".to_string(),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact2.clone());
        let contact3 = Contact::new(
            "Daniel".to_string(),
            "Ricciardo".to_string(),
            "Some address".to_string(),
            "555555555".to_string(),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact3.clone());

        let result = address_book.get_contact_by_name("Lewis Hamilton");
        assert_eq!(result, Some(&contact1));

        let result = address_book.get_contact_by_name("Max Verstappen");
        assert_eq!(result, Some(&contact2));

        let result = address_book.get_contact_by_name("Daniel Ricciardo");
        assert_eq!(result, Some(&contact3));

        let result = address_book.get_contact_by_name("Sebastian Vettel");
        assert_eq!(result, None);
    }

    #[test]
    fn test_get_contact_by_phone_number() {
        let mut address_book = AddressBook::new();
        let contact = Contact::new(
            "Fernando".to_string(),
            "Alonso".to_string(),
            "8 Place de la Concorde, Paris".to_string(),
            "987654321".to_string(),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact.clone());

        let found_contact = address_book.get_contact_by_phone_number("987654321");
        assert_eq!(found_contact.unwrap(), &contact);

        let not_found_contact = address_book.get_contact_by_phone_number("123456789");
        assert!(not_found_contact.is_none());
    }

    #[test]
    fn test_delete_contact_by_id() {
        let mut address_book = AddressBook::new();

        // Add some contacts
        let contact1 = Contact::new(
            String::from("Lewis"),
            String::from("Hamilton"),
            String::from("123 Main St"),
            String::from("555-1234"),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact1.clone());

        let contact2 = Contact::new(
            String::from("Max"),
            String::from("Verstappen"),
            String::from("456 Oak St"),
            String::from("555-5678"),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact2.clone());

        let contact3 = Contact::new(
            String::from("Sergio"),
            String::from("Perez"),
            String::from("789 Maple St"),
            String::from("555-9101"),
            address_book.last_assigned_id,
        );
        address_book.add_contact(contact3.clone());

        // Test deleting a contact by ID that exists
        let result = address_book.delete_contact_by_id(contact1.get_id());
        assert_eq!(result, Some((contact1.get_id(), contact1.clone())));
        assert_eq!(address_book.get_contact_by_id(contact1.get_id()), None);

        // Test deleting a contact by ID that doesn't exist
        let result = address_book.delete_contact_by_id(100);
        assert_eq!(result, None);

        // Test deleting the last remaining contact
        let result = address_book.delete_contact_by_id(contact3.get_id());
        assert_eq!(result, Some((contact3.get_id(), contact3.clone())));
        assert_eq!(address_book.get_contact_by_id(contact3.get_id()), None);

        // Test deleting a contact by ID that was already deleted
        let result = address_book.delete_contact_by_id(contact1.get_id());
        assert_eq!(result, None);
    }
}
