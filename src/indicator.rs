#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Into<::chrono::Weekday> for Day {
    fn into(self) -> ::chrono::Weekday {
        match self {
            Day::Monday    => ::chrono::Weekday::Mon,
            Day::Tuesday   => ::chrono::Weekday::Tue,
            Day::Wednesday => ::chrono::Weekday::Wed,
            Day::Thursday  => ::chrono::Weekday::Thu,
            Day::Friday    => ::chrono::Weekday::Fri,
            Day::Saturday  => ::chrono::Weekday::Sat,
            Day::Sunday    => ::chrono::Weekday::Sun,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub enum Month {
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

impl Into<u32> for Month {
    fn into(self) -> u32 {
        match self {
            Month::January   =>  1,
            Month::February  =>  2,
            Month::March     =>  3,
            Month::April     =>  4,
            Month::May       =>  5,
            Month::June      =>  6,
            Month::July      =>  7,
            Month::August    =>  8,
            Month::September =>  9,
            Month::October   => 10,
            Month::November  => 11,
            Month::December  => 12,
        }
    }
}