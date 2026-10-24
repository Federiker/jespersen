//! Abbreviations read by the word after them (jespersen's own list, CC0): Dr. Jones is Doctor Jones,
//! Mulholland Dr. is Drive; Jan. 5 is January fifth.

/// The words of abbreviation `text` before `next` (its text, and whether it is a number); `None`
/// when `text` is no such abbreviation or `next` does not make it one.
pub fn title(text: &str, next: Option<(&str, bool)>) -> Option<&'static str> {
    let before_name = next.is_some_and(|(next, _)| next.chars().next().is_some_and(char::is_uppercase));
    let before_number = next.is_some_and(|(_, number)| number);
    Some(match text {
        "Dr" if before_name => "doctor",
        "Dr" => "drive",
        "St" if before_name => "saint",
        "St" => "street",
        "Prof" => "professor",
        "Gen" if before_name => "general",
        "Col" if before_name => "colonel",
        "Capt" => "captain",
        "Lt" if before_name => "lieutenant",
        "Sgt" => "sergeant",
        "Rev" if before_name => "reverend",
        "Hon" if before_name => "honourable",
        "Mt" if before_name => "mount",
        "Jr" => "junior",
        "Sr" => "senior",
        "Ave" => "avenue",
        "Rd" => "road",
        "Blvd" => "boulevard",
        "Dept" => "department",
        "Inc" => "incorporated",
        "Ltd" => "limited",
        "Corp" => "corporation",
        "pp" if before_number => "pages",
        "p" if before_number => "page",
        "ch" | "Ch" if before_number => "chapter",
        "vol" | "Vol" if before_number => "volume",
        "fig" | "Fig" if before_number => "figure",
        month if before_number => match month {
            "Jan" => "January",
            "Feb" => "February",
            "Mar" => "March",
            "Apr" => "April",
            "Jun" => "June",
            "Jul" => "July",
            "Aug" => "August",
            "Sep" | "Sept" => "September",
            "Oct" => "October",
            "Nov" => "November",
            "Dec" => "December",
            _ => return None,
        },
        _ => return None,
    })
}
