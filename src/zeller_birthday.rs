pub struct Date {
    pub nday: u8,
    pub month: u8,
    pub year: u32,
}

#[derive(Debug, PartialEq)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

impl Day {
    fn new(dayn: u8) -> Option<Self>{
        match dayn {
            0 => Some(Self::Sunday),
            1 => Some(Self::Monday),
            2 => Some(Self::Tuesday),
            3 => Some(Self::Wednesday),
            4 => Some(Self::Thursday),
            5 => Some(Self::Friday),
            6 => Some(Self::Saturday),
            _ => None,
        }
    }
}

pub fn find_day(date: &Date) -> Day {
    let mut month = date.month;
    let mut year = date.year;
    let nday = date.nday;

    if date.month >= 3 {
        month -= 2;
    } else {
        month += 10;
    }

    if month == 11 || month == 12 {
        year -= 1;
    }

    let century = year / 100;
    let anti_century = year % 100;

    let mut result = 0i32;
    result += nday as i32;
    result += ((13. / 5.) * month as f32 - 0.2).floor() as i32;
    result += anti_century as i32;
    result += (anti_century / 4) as i32;
    result += (century / 4) as i32;
    result -= (2 * century) as i32;
    result %= 7;

    if year >= 1700 && year <= 1751 {
        result -= 3;
    } else {
        if year <= 1699 {
            result -= 4;
        }
    }

    if result < 0 {
        result += 1;
    }

    if let Some(day) = Day::new(result as u8) {
        day
    } else {
        Day::Sunday
    }
}

#[test]
fn find_day_test() {
    assert_eq!(
        find_day(
            &Date {
                nday: 21,
                month: 8,
                year: 2022,
            }
        ), Day::Sunday
    )
}
