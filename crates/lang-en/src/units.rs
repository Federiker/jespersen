//! Units after a number (jespersen's own list, CC0), named in British spelling; the lexicon finds
//! them under their American spellings (kilometer).

/// A unit's singular and plural names, for its symbol.
pub fn unit(symbol: &str) -> Option<(&'static str, &'static str)> {
    Some(match symbol {
        "km" => ("kilometre", "kilometres"),
        "m" => ("metre", "metres"),
        "cm" => ("centimetre", "centimetres"),
        "mm" => ("millimetre", "millimetres"),
        "kg" => ("kilogram", "kilograms"),
        "g" => ("gram", "grams"),
        "mg" => ("milligram", "milligrams"),
        "lb" | "lbs" => ("pound", "pounds"),
        "oz" => ("ounce", "ounces"),
        "ft" => ("foot", "feet"),
        "mi" => ("mile", "miles"),
        "mph" => ("mile per hour", "miles per hour"),
        "kph" => ("kilometre per hour", "kilometres per hour"),
        "ml" => ("millilitre", "millilitres"),
        "KB" | "kB" => ("kilobyte", "kilobytes"),
        "MB" => ("megabyte", "megabytes"),
        "GB" => ("gigabyte", "gigabytes"),
        "TB" => ("terabyte", "terabytes"),
        "Hz" => ("hertz", "hertz"),
        "kHz" => ("kilohertz", "kilohertz"),
        "MHz" => ("megahertz", "megahertz"),
        "GHz" => ("gigahertz", "gigahertz"),
        "kW" => ("kilowatt", "kilowatts"),
        "kWh" => ("kilowatt hour", "kilowatt hours"),
        _ => return None,
    })
}

/// The unit a symbol after "/" stands for (km/h: per hour).
pub fn per(symbol: &str) -> Option<&'static str> {
    Some(match symbol {
        "h" => "hour",
        "s" => "second",
        "min" => "minute",
        _ => return None,
    })
}
