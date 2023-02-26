use crate::contact::Contact;
use std::collections::HashMap;

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
}
