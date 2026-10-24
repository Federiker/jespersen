/// Abbreviations written with a full stop that are usually followed by a capitalised name or
/// number, so the stop does not end the sentence.
#[rustfmt::skip]
const ABBREVIATIONS: &[&str] = &[
    // Titles.
    "Mr", "Mrs", "Ms", "Mx", "Dr", "Prof", "Rev", "Hon", "Jr", "Sr",
    "Sen", "Rep", "Gov", "Gen", "Col", "Capt", "Lt", "Sgt",
    // Places.
    "St", "Mt", "Ft", "Ave", "Rd", "Blvd",
    // Companies and departments.
    "Dept", "Inc", "Ltd", "Corp",
    // Months before a day.
    "Jan", "Feb", "Mar", "Apr", "Jun", "Jul", "Aug", "Sep", "Sept", "Oct", "Nov", "Dec",
    // References.
    "No", "Nos", "Vol", "vol", "Fig", "fig", "pp", "p", "ch", "Ch", "vs", "cf", "approx",
];

/// Whether `word` (without its full stop) is an abbreviation that does not end a sentence when a
/// capitalised word follows it, as in "Mr. Smith". Lower-case followers need no list: they never
/// start a sentence.
#[must_use]
pub fn is_abbreviation(word: &str) -> bool {
    ABBREVIATIONS.contains(&word)
}
