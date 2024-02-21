/*
    Clay Molitor
    2/18/2024

    From me, this lab has mainly been an exercise in familiarizing myself with Rust's limitations.
    I'm used to OOP so Rust puts limits on the programer that I am just learning.

    For a detailed writeup on my program's functionality and how I addressed the problems I encountered please see the attached writeup.
*/
use std::collections::HashSet;
use std::fmt;

fn main() {
    // Three sets loaded from file. 
    let mut a: HashSet<(ReadCountString, String)> = HashSet::new();
    let mut b: HashSet<(ReadCountString, String)> = HashSet::new();
    let mut c: HashSet<(ReadCountString, String)> = HashSet::new();
    set_from_file("A", &mut a);
    set_from_file("B", &mut b);
    set_from_file("C", &mut c);
    
    println!("🦀 Running Program 🦀\n");

    // Print A ∪ B
    println!("    A ∪ B: ");
    for (s, v) in a.union(&b) {
        println!("{:5} : {}", s._value, v)
    }
    // Print A ∩ B
    println!("    A ∩ B: ");
    for (s, v) in a.intersection(&b) {
        println!("{:5} : {}", s._value, v)
    }
    // Print A \\ C
    println!("    A \\ C: ");
    for (s, v) in a.difference(&c) {
        println!("{:5} : {}", s._value, v)
    }
    // Print B △ C:
    println!("    B △ C: ");
    for (s, v) in b.symmetric_difference(&c) {
        println!("{:5} : {}", s._value, v)
    }

    println!("\n 🎬 𝓕𝓲𝓷 🎬\n");

}

// Loads a HashSet array from a file
// File must be formatted as line of "KEY, VALUE", 
//  ex:
//  45234, Bob
//  34523, Fred
// The first column is returned as ReadCountString and the second column as Strings.
fn set_from_file(file_name: &str, output: &mut HashSet<(ReadCountString, String)>) -> Option<i32> {
    // Read passed file as string
    let content = std::fs::read_to_string(file_name).ok()?;

    for line in content.split('\n') {
        if let Some(pos) = line.find(',') {
            // Remove comma from string a
            let (a, b) = line.split_at(pos + 1);
            let a = (&a[0..pos]).trim();
            let b = b.trim();
            // Adds line to 
            output.insert((ReadCountString::new(a.to_string()), b.to_string()));
        }
    }
    Some(0)
}


// The purpose of this "class" is to have a string that keeps track of how often it is read
// However, Rust does not allow mutation during the eq or display call, so the read_count value can not be updated.
// The read_count variable is not used, so reads aren't actually tracked.
#[derive(Hash, Eq)]
struct ReadCountString {
    _value: String,  // Private, only access through .read()
    read_count: u64, // Increment each time _value is read. WIP
}

impl ReadCountString {
    // Constructor
    fn new(content: String) -> ReadCountString{
        ReadCountString{
            _value: content,
            read_count: 0
        }
    }
    // Used to read _value while incrementing read_count
    // Not used.
    fn _read(&mut self) -> String{
        self.read_count += 1;
        return self._value.clone();
    }
}
// Override eq and implements the PartialEq interface. 
// Should increment read_count
impl PartialEq for ReadCountString {
    fn eq(&self, other: &ReadCountString) -> bool {
        // self.read_count += 1;
        self._value == other._value
    }
}
// To string
// Should increment read_count
impl fmt::Display for ReadCountString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // self.read_count += 1;
        write!(f, "{}", self._value)
    }
}