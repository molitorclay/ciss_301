/*  Lab 1 - Truth Tables and Logic Expressions - Clay Molitor

My program takes a logical expression and outputs a truth table.
It also takes a truth table and outputs a sum of products formula. 
See the submission document for further details.

*/
use std::io;



fn main() {
    let mut user_input: String = String::new();

    println!("🦀 Hello! 🦀\n");

    // Prompt user to enter a logical expression.
    println!("\nEnter a logical expression.");
    println!("Ex: ((//B+A+(A*C))*/(/C+///(A*/B)))");

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = user_input.trim().to_string();
    if user_input.is_empty() {
        user_input = "((//B+A+(A*C))*/(/C+///(A*/B)))".to_string();
        println!("Using default expression: (//B+A+(A*C))*/(/C+///(A*/B))");
    }
    let ec_result = std::panic::catch_unwind( || {
        run_extra_credit(user_input);
    });
    if ec_result.is_ok() == false {
        println!("\n\n❗❗ Enter a valid expression next time ❗❗");
        println!("⚠️  If you leave the input empty the a default expression will be run ⚠️");

    }
    
    
    // Clean string
    let mut user_input: String = String::new();

    println!("\nEnter groups of to four 0s or 1s, separated by commas ");
    println!("Ex: 00110,111,00001,10101,10100");

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    user_input = user_input.trim().to_string();
    if user_input.is_empty() {
        user_input = "00110,111,00001,10101,10100".to_string();
        println!("Using default sum of products: 00110,111,00001,10101,10100");
    }


    // Remove all characters besides '1' '0' and ','
    user_input.retain(|c| c == '1' ||  c == '0' || c == ',');
    
    let sp_result = std::panic::catch_unwind( || {
        sum_of_products(user_input);
    });
    if sp_result.is_ok() == false {
        println!("\n\n❗❗ Enter a valid truth next time ❗❗");
        println!("⚠️  If you leave the input empty a default truth table will be used ⚠️");

    }
    

    println!("\n\n🦀 Bye! 🦀\n\n");
}

fn sum_of_products(entire_input: String)
{
    for (i, chunk) in entire_input.trim().split(',').enumerate() { // for each comma separated chunk
        if chunk.len() > 5 { 
            panic!("You entered more than 4 variables");
        }
        if i > 0 { // not first run
            print!(" + ");
        }
        for (j, c) in chunk.chars().enumerate() { // compare each character to the last char in the string
            if j < chunk.len() -1 { // ignore last char
                if chunk.ends_with(c) == false {
                    print!("/");
                }
                // Print j'th letter in alphabet 
                print!("{}", (j as u8 + 'A' as u8) as char);
            }
        }
    }
}

fn run_extra_credit(expression: String)
{
    // Steps:
    // For each combination in BOOL_COMBOS:
    //  Substitute variables with 0 or 1
    //  Apply negations
    //  loop
    //       remove parenthesis around single variables
    //       calculate deepest leftmost boolean operation
    //       Substitute the operation for its result
    //       Apply negations again
    //       break loop if only one variable is left
    //  Repeat for ever possible combination of values for A,B,C,D
    const BOOL_COMBOS: [[bool; 4]; 17] = [
        [false, false, false, false],
        [true , false, false, false],
        [false, true , false, false],
        [true , true , false, false],

        [false, false, true , false],
        [true , false, true , false],
        [false, true , true , false],
        [true , true , true , false],

        [false, false, false, true ],
        [true , false, false, true ],
        [false, true , false, true ],
        [true , true , false, true ],

        [false, false, true , true ],
        [true , false, true , true ],
        [false, true , true , true ],
        [true , true , true , true ],
        [true , true , true , true ],
    ];
    // Or: +, And: *
    //let expression = String::from("((//B+A+(A*C))*/(/C+///(A*/B)))");
    let var_count = number_of_variables_in_expression(expression.clone());
    //println!("{}", var_count);
    println!("{}", expression.clone());

    // Print header
    for c in ('A' as u8)..('A' as u8 + var_count as u8) {
        print!("{} ", c as char );
    }   println!("| F");
    
    for _ in 0..var_count {
        print!("--")
    }   println!("+--");
    

    // calculate and print truth table
    // 0 to var_count^2, in order to iterate only unique combinations when var_count < 4
    for row in &BOOL_COMBOS[..(var_count * var_count)] {
        // Print variable value
        for c in 0..(var_count) {
            print!("{} ", row[c] as u8);
        }   print!("| ");
        
        let mut expression = expression.clone(); // connects function user_inputs to inputs.
        expression = substitute_variables(expression, row, var_count);
        //println!("after subbing: {}", expression);

        expression = apply_nots('/', expression);
        //println!("after not-ing: {}", expression);
        
        while expression.len() >= 3{
            expression = remove_single_parenthesis(perform_operation(expression));
            //println!("\t{}", beautify_logic(expression.clone()));
    
        }
        println!("{}", expression);
    }
    
}
// Returns the number of unique variables used in the expression
// Variable used can not skip letters alphabetical, ex A B C may be used, but A C D may not as B is skipped
fn number_of_variables_in_expression(expression: String) -> usize
{
    let mut n: usize = 0;
    for c in ('A' as u8)..('Z' as u8)
    {
        if !expression.contains(c as char) {
            return n
        }
        n += 1;
    }
    0
}
// Pass the expression and an array of char-bool tuples
// returns an expression with letters included in variables replaced with 1 or 0, depending on their paired bool
fn substitute_variables(expression: String, variables: &[bool], var_count: usize) -> String
{
    let mut result: String = expression.clone();
    let mut letter = 'A' as u8;
    for bool_state in &variables[0..var_count]{
        result = result.replace(letter as char, if *bool_state {"1"} else {"0"});
        letter += 1;
    }
    result
}


// Replaces a not signs followed by a 1 or 0 with properly inverted booleans.  
// Multiple contiguous not signs are accounted for.
fn apply_nots(not_char: char, expression: String) -> String
{
    let mut result: String = String::new();
    let mut prev_char: Option<char> = None;
    let mut contiguous_nots = 0;
    // Rebuild the string removing not's and inverting booleans
    for c in expression.chars() {
        if c == not_char { // on logical not
            contiguous_nots += 1;
            // Don't push to results
        }
        else { // anything but a not
            
            if prev_char == Some(not_char) { // character after nots, throw error if not a variable or left parenthesis

                
                if matches!(c, '0' | '1') { // Flip boolean if odd number of nots
                    result.push(
                        if contiguous_nots % 2 == 1 { // Is odd number of nots
                            if c == '0' {'1'} else {'0'} // Inverse bool
                        }   else { c }
                    );
                }
                else if c == '(' {
                    for _ in 0..contiguous_nots{ // Add removed nots back if they where before a '(' 
                        result.push('/'); 
                    }
                    result.push(c);
                }
                else { // throw error if not a variable or left parenthesis
                    println!("{result}:::{expression}|||{c}");
                    panic!("Not's must be placed before a variable");
                }
            }
            else {
                result.push(c);
            }
            contiguous_nots = 0;
        }
        
        prev_char = Some(c);
    }

    result
}

fn perform_operation(expression: String) -> String
{
    // 1) remove useless parenthesis pair ex: (0) -> 0
    // 2) calculate inner most value 
    // 3) loop to top until one val is left

    let depth: i32 = op_max_depth(expression.as_str());
    let mut n: i32 = 0; // Track depth
    for i in 0..expression.len() { 
        match expression.chars().nth(i).unwrap() {
            '(' => n = n + 1,
            ')' => n = n - 1,
            _ => {}
        }
        if n == depth { // found leftmost deepest parenthesis
            let op_start = i +1;
            let op_end = op_start + 3;

            // replace operation with it's result
            return [
                &expression[..op_start], //before operation
                if Operation::new(&expression[op_start..op_end]).calculate() {"1"} else {"0"}, // operation results
                &expression[op_end..]   // after operation
                ].join("");
            //return Operation::new(&expression[i+1..i+4]);
        }

    }
    panic!("operation with depth {} not found in {}", n, expression);
}
// Returns the depth of deepest nested parenthesis.
fn op_max_depth(expression: &str) -> i32
{
    let mut d: i32 = 0;
    let mut max: i32 = 0;
    for c in expression.chars() {
        d = d + match c { // Count parenthesis to track current depth
            '(' => 1,
            ')' => -1,
            _ => 0,
        };
        max = std::cmp::max(d, max);
    }

    max
}
// Removes parenthesis that surround a single variable, like: 
// Example: '(n)' becomes just 'n'
// Also apply's nots, EXample '/(1)' becomes '0'
fn remove_single_parenthesis(expression: String) -> String
{
    let mut two_ago: char = '\0';
    let mut one_ago: char = '\0';

    let mut i = 0;

    for ch in expression.chars()
    {
        if ch == ')' && two_ago == '('
        {
            let result: String = [ // Trim out both parenthesis
                &expression[0..i-2], // Left
                one_ago.to_string().as_str(), // variable
                &expression[i+1..], // Right
            ].join(""); 
            //print!("{}", result);
            return remove_single_parenthesis(result);
        }
        two_ago = one_ago;
        one_ago = ch;
        i = i + 1;
    }
    // Return and calculate nots
    return apply_nots('/', expression);
}


/*
OR:     +
AND:    *
NOT:    /
*/

// An operation is a bundle of two booleans and a binary function
// The functions must take the two bools and return a third bool
struct Operation{
    a: bool,
    b: bool,
    function: BinaryFunction,
}
impl Operation { 
    pub fn new(builder: &str) -> Operation {
        Operation {
            // convert 1/0 to true/false
            a:         match builder.chars().nth(0).unwrap(){
                '0' => false,
                '1' => true,
                _ => {panic!("Invalid operation: {}", builder)}, 
            },
            // Convert */+ to And/Or
            function:  match builder.chars().nth(1).unwrap(){
                '*' => logic_and,
                '+' => logic_or,
                _ => {panic!("Invalid operation: {}", builder)}, 
            },
            b:         match builder.chars().nth(2).unwrap(){
                '0' => false,
                '1' => true,
                _ => {panic!("Invalid operation: {}", builder)}, 
            },
        }
    }
}
impl Operation {
    pub fn calculate(&self) -> bool{
        (self.function)(self.a, self.b)
    }
}
// binary operators: and, or
type BinaryFunction = fn(bool, bool) -> bool;
// AND char: *
fn logic_and(a: bool, b: bool) -> bool{
    a && b
}
// OR char: +
fn logic_or(a: bool, b: bool) -> bool{
    a || b
}


