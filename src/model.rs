use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Day {
    pub date: NaiveDate,
    pub meals: Vec<Meal>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meal {
    pub name: String,
    pub category: String,
    pub prices: Prices,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Prices {
    pub students: Option<f64>,
    pub employees: Option<f64>,
    pub others: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
}
