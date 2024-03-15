/*
    Clay Molitor

*/

use regex::Regex;

fn main() {
    println!("/t/t🔎 Regex 🔎\n");

    // Regex expressions
    let rgx_american_number = r"(?x)
        ^.* # Beginning of line
        \(??\d {3,4}\)??  # Optional area code with or without parens
        [-,\s]?? \d{3}    # Three digits
        [-,\s]?? \d{4}    # Last Four digits";
    // International regex from emery.
    let international_number = r"(?x)^\+
        (?:\d\s?){1,3}
        (\(\d{1,4}\)
        |
        \d{1,4})
        (?:[.\-\s]?\d{1,})+$";
    let rgx_addresses = r"(?xs) 
        ^.*        # Beginning of address
        [,\s]      # Space or comma before state
        [A-Z]{2}   # State
        [,\s]{1,2} # Space after state
        [0-9]{5}   # Zip";
    let rgx_log = r"(?x) 
        v10\.4\.6 .*? # Version number v10.4.6
        ERROR .*?     # Error
        insufficient \s permissions .*? # message contains: insufficient permissions
        ";
    // As per USPS publication 28.2.22.224, States must be abbreviated to two upper case characters.
    // https://pe.usps.com/text/pub28/pub28c2_009.htm

    // Read files
    let phone_numbers_file = std::fs::read_to_string("phone_numbers.txt").ok().unwrap();
    let address_file = std::fs::read_to_string("addresses.txt").ok().unwrap();
    let log_file = std::fs::read_to_string("log_file.txt").ok().unwrap();

    // -------------------------- Print Phone Numbers --------------------------
    println!("    Phone number     ┃ Valid American ┃ Valid International ┃");
    println!(" ━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━━━━━━━┫");

    // Create table of phone numbers and if they are valid
    for phone_number in phone_numbers_file.split("\n") {
        print!("{:21}┃", phone_number);
        // Find if regex hits
        if regex_matches(phone_number, rgx_american_number) {
            print!("    {:12}┃", "X");
        } else {
            print!("    {:12}┃", " ");
        }
        if regex_matches(phone_number, international_number) {
            print!("    {:17}┃", "X");
        } else {
            print!("    {:17}┃", " ");
        }
        println!();
    }

    // -------------------------- Print Addresses --------------------------
    println!("                                                            ┃ Valid Address ┃");
    println!(" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━╋━━━━━━━━━━━━━━━┫");
    for address in address_file.split("\n") {
        let address_lines: Vec<&str> = address.lines().collect();
        for line in address_lines {
            print!("{:60}┃", line);
            if regex_matches(address, rgx_addresses) {
                print!("    {:11}┃", "X");
            } else {
                print!("    {:11}┃", " ");
            }
            println!();
        }
    }

    // -------------------------- Print log --------------------------
    println!(" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    for log in log_file.lines() {
        if regex_matches(log, rgx_log) {
            println!("{log}");
        }
    }
    println!(" ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("\n🐬 Done 🐬");
    
}

// If a regex expression search finds a match in data, return true
fn regex_matches<'a>(data: &'a str, search: &'a str) -> bool {
    if let Ok(regex) = { Regex::new(search) } {
        return regex.is_match(data);
    }
    false
}