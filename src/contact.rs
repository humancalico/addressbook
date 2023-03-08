use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Default)]

pub struct Contact {
    id: usize,
    first_name: String,
    last_name: String,
    address: String,
    phone_number: String,
}

// Display trait for Contact struct
impl Display for Contact {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_tsv_string())
    }
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

    pub fn from_tsv_string(tsv_string: &str) -> Result<Self, ParseError> {
        let fields: Vec<&str> = tsv_string.trim().split('\t').collect();

        if fields.len() != 5 {
            return Err(ParseError::InvalidData(
                "Invalid number of fields".to_string(),
            ));
        }

        let id = match fields[0].parse::<usize>() {
            Ok(id) => id,
            Err(_) => return Err(ParseError::InvalidId),
        };

        let first_name = fields[1].to_string();
        let last_name = fields[2].to_string();
        let address = fields[3].to_string();
        let phone_number = fields[4].to_string();

        Ok(Self {
            id,
            first_name,
            last_name,
            address,
            phone_number,
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidData(String),
    InvalidId,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParseError::InvalidData(msg) => write!(f, "Invalid data: {}", msg),
            ParseError::InvalidId => write!(f, "Invalid ID"),
        }
    }
}

impl Error for ParseError {}
