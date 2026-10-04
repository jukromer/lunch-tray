use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Day {
    pub date: NaiveDate,
    pub meals: Vec<Meal>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Meal {
    pub name: String,
    pub category: String,
    pub prices: Prices,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Prices {
    pub students: Option<f64>,
    pub employees: Option<f64>,
    pub others: Option<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Canteen {
    pub id: u32,
    pub name: String,
    pub city: String,
}

impl Canteen {
    pub fn matches(&self, query: &str) -> bool {
        let query = query.to_lowercase();
        self.name.to_lowercase().contains(&query) || self.city.to_lowercase().contains(&query)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PriceGroup {
    Students,
    Employees,
    Guests,
}

impl PriceGroup {
    pub fn price(self, prices: &Prices) -> Option<f64> {
        match self {
            PriceGroup::Students => prices.students,
            PriceGroup::Employees => prices.employees,
            PriceGroup::Guests => prices.others,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            PriceGroup::Students => "students",
            PriceGroup::Employees => "employees",
            PriceGroup::Guests => "guests",
        }
    }

    pub fn from_name(name: &str) -> Option<PriceGroup> {
        match name {
            "students" => Some(PriceGroup::Students),
            "employees" => Some(PriceGroup::Employees),
            "guests" => Some(PriceGroup::Guests),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn prices() -> Prices {
        Prices {
            students: Some(3.1),
            employees: Some(4.25),
            others: None,
        }
    }

    fn tha() -> Canteen {
        Canteen {
            id: 803,
            name: String::from("Mensa TH Augsburg"),
            city: String::from("Augsburg"),
        }
    }

    #[test]
    fn students_pay_student_price() {
        assert_eq!(PriceGroup::Students.price(&prices()), Some(3.1));
    }

    #[test]
    fn employees_pay_employee_price() {
        assert_eq!(PriceGroup::Employees.price(&prices()), Some(4.25));
    }

    #[test]
    fn guests_use_others_field() {
        assert_eq!(PriceGroup::Guests.price(&prices()), None);
    }

    #[test]
    fn empty_query_matches_everything() {
        assert!(tha().matches(""));
    }

    #[test]
    fn matches_name_ignoring_case() {
        assert!(tha().matches("mensa th"));
    }

    #[test]
    fn matches_city_ignoring_case() {
        assert!(tha().matches("AUGSBURG"));
    }

    #[test]
    fn rejects_other_cities() {
        assert!(!tha().matches("berlin"));
    }
}
