/*
    Clay Molitor

    Call chatgpt to solve the entered deduction. 
    Show steps an
    See lab report for details and explanation.
*/
use chatgpt::{prelude::*, types::CompletionResponse};
use std::io;
use std::fs::File;
use std::io::prelude::*;


#[tokio::main]
async fn main() -> Result<()>{

    println!("🏁 Starting 🏁");

    // Read chatgpt key from file
    let mut key = String::new();
    let mut file = File::open("key")?;
    file.read_to_string(&mut key)?;



    let mut user_deduction = String::new();

    println!("Enter a deduction.");
    println!("Example: \n(m ∧ ¬b) → j\n(f ∨ s) → m\nb → t\nf → ¬t\nf\n∴ j\n");

    while !user_deduction.contains("∴") {

        io::stdin().read_line(&mut user_deduction).expect("Could not read line");
        user_deduction = fancify_logic(user_deduction);

        //user_deduction.clear();
    }
    println!("\nYou entered");
    println!("{user_deduction} ");

    
    // Ask chatGPT for solving steps in a table
    let client = ChatGPT::new_with_config(
        key,
        ModelConfigurationBuilder::default()
            .temperature(0.1)
            .engine(ChatGPTEngine::Gpt4)
            .build()
            .unwrap(),
    )?;
    let response: CompletionResponse = client
        .send_message("Give a proof for deductions staring at the end and going to the top, telling me which rules are applied for each step. Don't print an introduction or conclusion. 
        Print the output in a table.".to_owned()
     + user_deduction.as_str())
        .await?;

    // format and print the table chatgpt returns
    println!("{}", print_table(response.message().content.clone()));
    // println!("{}", response.message().content);
    
    // Ask chatgpt if deduction is valid
    // TODO: use a smarter AI
    let response: CompletionResponse = client
        .send_message("Is the deduction valid? answer valid, invalid, or unknown".to_owned()
        + user_deduction.as_str())
        .await?;
    
    println!("Your deduction is: {}", response.message().content);
    
    Ok(())
}


// Takes the jagged chatgtp table and justifies the columns
fn print_table(raw_table: String) -> String {

    let mut result = String::new();

    let mut max_column_width = [0,0,0,0,0];

    // Remove paragraphs chatgpt likes to add
    let mut table = remove_paragraphs(raw_table);
    table = remove_paragraphs(table.chars().rev().collect::<String>() ).chars().rev().collect::<String>();

    // Determin the max width of each column
    for line in table.replace("-", "").trim().lines() {


        for (i, chunk) in line.split("|").enumerate() {
            max_column_width[i] = std::cmp::max(max_column_width[i], chunk.trim().len());
        }
    }

    // Save the new table to result
    for (line_index, line) in table.trim().lines().enumerate() {


        for (i, chunk) in line.split("|").enumerate() {
            if line_index == 1{ // fill out dashes bellow title
                result.push_str(&format!("{} | ", "-".repeat(max_column_width[i]) ));
            }
            else { // print column to width of longest element
                result.push_str(&format!("{:width$} | ", chunk.trim(), width = max_column_width[i]));

            }
        }
        result.push('\n');
    }

    result
}
// Removes the characters before the first '|' in a string
fn remove_paragraphs(input: String) -> String {
    if let Some(index) = input.find('|') {
        input[index..].to_string()
    } else {
        input
    }
}
// turns ascii into unicode logic symbols
fn fancify_logic(expression: String) -> String {
    expression
        .replace(">", "→")
        .replace("~", "¬")
        .replace("*", "∧")
        .replace("+", "∨")
        .replace("A", "∀")
        .replace("E", "∃")
        .replace("R", "∴")
}

// turns unicode into ascii logic symbols
#[allow(dead_code)]
fn defancify_logic(expression: String) -> String {
    expression
        .replace("→", ">")
        .replace("¬", "~")
        .replace("∧", "*")
        .replace("∨", "+")
        .replace("∀", "A")
        .replace("∃", "E")
        .replace("∴", "R")
}
