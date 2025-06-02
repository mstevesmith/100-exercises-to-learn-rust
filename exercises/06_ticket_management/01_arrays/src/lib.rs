// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    // TODO
    temps: [i32; 7],
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

fn day_idx(day: &Weekday) -> Result<usize, String> {
    match day {
        Weekday::Monday => Ok(0),
        Weekday::Tuesday => Ok(1),
        Weekday::Wednesday => Ok(2),
        Weekday::Thursday => Ok(3),
        Weekday::Friday => Ok(4),
        Weekday::Saturday => Ok(5),
        Weekday::Sunday => Ok(6),
        _ => Err("Invalid Day".to_string())
    }
}

impl WeekTemperatures {
    pub fn new() -> Self {
        let vals: [i32; 7] = [-1000; 7];

        WeekTemperatures { temps: vals }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
        let x: i32 = self.temps[day_idx(&day).unwrap()];

        if x != -1000 {
            return Some(x);
        } else {
            return None;
        }
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
        self.temps[day_idx(&day).unwrap()] = temperature;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_temperature() {
        let mut week_temperatures = WeekTemperatures::new();

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Tuesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Wednesday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Thursday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Saturday), None);
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), None);

        week_temperatures.set_temperature(Weekday::Monday, 20);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(20));

        week_temperatures.set_temperature(Weekday::Monday, 25);
        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));

        week_temperatures.set_temperature(Weekday::Tuesday, 30);
        week_temperatures.set_temperature(Weekday::Wednesday, 35);
        week_temperatures.set_temperature(Weekday::Thursday, 40);
        week_temperatures.set_temperature(Weekday::Friday, 45);
        week_temperatures.set_temperature(Weekday::Saturday, 50);
        week_temperatures.set_temperature(Weekday::Sunday, 55);

        assert_eq!(week_temperatures.get_temperature(Weekday::Monday), Some(25));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Tuesday),
            Some(30)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Wednesday),
            Some(35)
        );
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Thursday),
            Some(40)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Friday), Some(45));
        assert_eq!(
            week_temperatures.get_temperature(Weekday::Saturday),
            Some(50)
        );
        assert_eq!(week_temperatures.get_temperature(Weekday::Sunday), Some(55));
    }
}
