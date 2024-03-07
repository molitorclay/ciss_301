/*
    Clay Molitor: Lab 5 - Relations

    A requirement of this assignment is to use a functional programming style. 
        No mutability or global variables are allowed. 
        I've decided to take things further by using as few variables 
        and branches as possible, focusing instead on long chains of function calls. 
        Parts of my code poorly written to meet this arbitrary goal.
        Please remember that if you hire me I'll write good code, not what's below.
*/

use Relation;
use regex::Regex;

type LPair<'a> = Vec<(&'a str, &'a str)>; // {(a, b), ...}
type SSet<'a> = Vec<&'a str>; // Not used for calculations, but still loaded from file.
type FullSet<'a> = (SSet<'a>, LPair<'a>); // (S, L)

type RelationOperation<T> = fn(&Relation<T>) -> bool;

fn main() {

    std::fs::read_to_string("sets.txt")
        .unwrap()
        .replace(" ", "") // remove spaces
        .lines() // for each line
        .map(|line: &str| {
                println!("Line: {}", line); // Print relation set before relations table
                sets_from_line(line).unwrap() // Load sets from line 
            })
        .map(|set| format_results(set.1)) // set.1 is the LPair
        .for_each(|x| println!("{}", x)) // Print each result
        ;
}

// Remove the unused function format_results
fn format_results(set: LPair) -> String {

    let operations: Vec<(RelationOperation<&str>, &str)> = vec![
        (Relation::is_reflexive,     "reflexive"),
        (Relation::is_symmetric,     "symmetric"),
        (Relation::is_transitive,    "transitive"),
        (Relation::is_antisymmetric, "antisymmetric"),
        (Relation::is_equivalence,   "equivalence")
    ];

    operations.iter()
        .map(|(op, name)| format!("{:14}: {}", // Print to string,
            name, // The name of the function.
            op(&Relation::from_iter(set.clone())))) // Function result (true/false)
        .collect::<Vec<String>>()
        .join("\n") // Each function goes on a new line.
}
// Pass a line loaded from sets.txt
// Returns a FullSet populated with line data
fn sets_from_line<'a>(line: &'a str) -> Option<FullSet<'a>> {
    Regex::new(r"(?x)
        \[
            (?P<S> .*  ) # Capture first set, S
        \] , \[\[        # Match delimitating commas and brackets
            (?P<L> .*)   # Capture second set, L
        \]\] ")
    .unwrap()
    .captures(line)? // Apply regex to passed arg, line
    .iter()
    .map(|x| x.unwrap().as_str()) // convert S and L to strings
    .collect::<Vec<&'a str>>()
    .get(0..3) // Grab all matches
    .map(|slice| // Match 0 dropped, because it's the entered string.
        (slice[1] // S
            .split(",")
            .collect::<Vec<&'a str>>() 
        ,
        (slice[2] // L
            .split("],[") // Break into pairs
            .map(|x| x // Turn pairs into tuples
                .split(",")
                .collect::<Vec<&'a str>>()) 
            .map(|x| (x[0], x[1]))
            .collect::<Vec<(&'a str, &'a str)>>()
        ))
    )
}
