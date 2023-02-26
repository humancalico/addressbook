#[derive(Debug)]
pub struct Contact {
    id: usize,
    first_name: String,
    last_name: String,
    address: String,
    phone_number: String,
}

impl Contact {
    fn new(
        first_name: String,
        last_name: String,
        address: String,
        phone_number: String,
        last_assigned_id: usize,
    ) -> Self {
        Self {
            id: last_assigned_id + 1,
            first_name,
            last_name,
            address,
            phone_number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_contact() {
        let c = Contact::new(
            "Fernando".to_string(),
            "Alonso".to_string(),
            "8 Place de la Concorde, Paris".to_string(),
            "987654321".to_string(),
            0,
        );
        assert_eq!(c.id, 1);
        assert_eq!(c.first_name, "Fernando");
        assert_eq!(c.last_name, "Alonso");
        assert_eq!(c.address, "8 Place de la Concorde, Paris");
        assert_eq!(c.phone_number, "987654321");
    }
}
