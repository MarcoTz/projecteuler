#[derive(PartialEq, Eq)]
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    fn num_days(&self) -> usize {
        if matches!(self, Month::February) {
            return 28;
        }

        if matches!(
            self,
            Month::September | Month::April | Month::June | Month::November
        ) {
            return 30;
        }

        31
    }

    fn next(self) -> Month {
        match self {
            Month::January => Month::February,
            Month::February => Month::March,
            Month::March => Month::April,
            Month::April => Month::May,
            Month::May => Month::June,
            Month::June => Month::July,
            Month::July => Month::August,
            Month::August => Month::September,
            Month::September => Month::October,
            Month::October => Month::November,
            Month::November => Month::December,
            Month::December => Month::January,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Year(usize);

impl Year {
    fn is_leap(&self) -> bool {
        (self.0 % 4 == 0 && self.0 % 100 != 0) || self.0 % 400 == 0
    }
}

#[derive(PartialEq, Eq)]
enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Weekday {
    fn next(self) -> Weekday {
        match self {
            Weekday::Monday => Weekday::Tuesday,
            Weekday::Tuesday => Weekday::Wednesday,
            Weekday::Wednesday => Weekday::Thursday,
            Weekday::Thursday => Weekday::Friday,
            Weekday::Friday => Weekday::Saturday,
            Weekday::Saturday => Weekday::Sunday,
            Weekday::Sunday => Weekday::Monday,
        }
    }
}

struct Day {
    day: usize,
    weekday: Weekday,
    month: Month,
    year: Year,
}

impl Default for Day {
    fn default() -> Day {
        Day {
            day: 1,
            weekday: Weekday::Monday,
            month: Month::January,
            year: Year(1901),
        }
    }
}

impl Day {
    fn next(self) -> Day {
        let mut month_days = self.month.num_days();
        if self.month == Month::February && self.year.is_leap() {
            month_days += 1;
        }
        let day = (self.day + 1) % month_days + 1;
        let weekday = self.weekday.next();
        let mut month = self.month;
        let mut year = self.year;
        if day == 1 {
            month = month.next();
            if month == Month::January {
                year.0 += 1;
            }
        }
        Day {
            day,
            weekday,
            month,
            year,
        }
    }
}

fn main() {
    let mut day = Day::default();
    let mut mondays = 0;
    while day.year < Year(2000) {
        day = day.next();
        if day.weekday == Weekday::Monday && day.day == 1 {
            mondays += 1;
        }
    }
    println!("Number of mondays: {mondays}");
}
