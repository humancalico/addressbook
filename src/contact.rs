#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    id: usize,
    first_name: String,
    last_name: String,
    address: String,
    phone_number: String,
}

impl Contact {
    pub fn new(
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

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn get_phone_number(&self) -> String {
        return self.phone_number.to_string();
    }

    pub fn to_tsv_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}",
            self.id, self.first_name, self.last_name, self.address, self.phone_number
        )
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

    #[test]
    fn test_get_id() {
        let contact = Contact::new(
            "Lewis".to_string(),
            "Hamilton".to_string(),
            "Mercedes-AMG Petronas Formula One Team, Brackley, UK".to_string(),
            "1234567890".to_string(),
            0,
        );
        assert_eq!(contact.get_id(), 1);
    }

    #[test]
    fn test_get_full_name() {
        let contact = Contact::new(
            "Max".to_string(),
            "Verstappen".to_string(),
            "Red Bull Racing, Milton Keynes, UK".to_string(),
            "1234567800".to_string(),
            0,
        );
        assert_eq!(contact.get_full_name(), "Max Verstappen");
    }

    #[test]
    fn test_get_phone_number() {
        let contact = Contact::new(
            "Valtteri".to_string(),
            "Bottas".to_string(),
            "Mercedes-AMG Petronas Formula One Team, Brackley, UK".to_string(),
            "1234567810".to_string(),
            0,
        );
        assert_eq!(contact.get_phone_number(), "1234567810");
    }

    #[test]
    fn test_to_tsv_string() {
        let contact = Contact::new(
            "Lewis".to_string(),
            "Hamilton".to_string(),
            "7 World Championships St".to_string(),
            "+44 123 456 7890".to_string(),
            0,
        );
        assert_eq!(
            contact.to_tsv_string(),
            "1\tLewis\tHamilton\t7 World Championships St\t+44 123 456 7890"
        );
    }
}
