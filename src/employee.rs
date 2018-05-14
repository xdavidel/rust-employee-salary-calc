pub struct Employee {
    name: String,
    age: u8,
    salary: f32,
    raise_factor: f32,
}

impl ToString for Employee {
    fn to_string(&self) -> String {
        return format!(
            "{}, {} years old, is earning {} per month",
            self.name, self.age, self.salary
        );
    }
}

impl Employee {
    pub fn future_salary(&self, years: u8) -> f32 {
        let mut salary: f32 = self.salary;

        for _ in 0..years {
            salary *= self.raise_factor;
        }

        return salary;
    }

    pub fn new(name: String, age: u8, salary: f32, raise_factor: f32) -> Employee {
        Employee {
            name: name,
            age: age,
            salary: salary,
            raise_factor: raise_factor,
        }
    }
}
