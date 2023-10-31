// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Bill {
    name: String,
    owe: f64,
}

impl Bill {
    fn new(name: String, owe: f64) -> Self {  //create new bill
        Self {name, owe}
    }
}
#[derive(Debug, Clone)]
pub struct Bills {
    inner: HashMap<String, Bill>
}

impl Bills {  //creating new instance of a Bills Hashmap
    fn new() -> Self {
        Self{inner: HashMap::new()}
    }

    fn add_bill(&mut self, bill: Bill) { 
        //let bill_name = bill.name.clone();
        self.inner.insert(bill.name.to_owned(), bill);
    }

    fn view_bill(&mut self) {
        for (id, bill) in &self.inner {
            println!("Bill: ID: {} || Name: {} || Amount Owed: {}", id, bill.name, bill.owe);
        }
    }

    fn remove_bill(&mut self, id: &str) {  //prompts for the ID of the bill and then removes the selected bill
        let id = id.to_string();
        self.inner.remove(&id);
    
    }

    fn edit_bill(&mut self, id_input: &str, edited_bill: Bill) {
        
        self.inner.insert(id_input.to_string(), edited_bill);
    }
}
enum Menu {
    AddBill,
    ViewBill,
    RemoveBill,
    EditBill
}

impl Menu {
    fn user_input_menu(input: &str) -> Option<Self> {

        match input {
            "1" => Some(Menu::AddBill),
            "2" => Some(Menu::ViewBill),
            "3" => Some(Menu::RemoveBill),
            "4" => Some(Menu::EditBill),
            _ => None,
        }
    }
}

//user input function
fn user_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}

mod menu {
    use crate::{Bills, Bill, user_input};

    pub fn main_menu() {  //main menu function for the initial main menu
        println!("Please select a number which correlates with the menu item");
        println!("1 = Add a bill");
        println!("2 = View Bill(s)");
        println!("3 = Remove a bill");
        println!("4 = Edit a bill");
    }

    pub fn add_bill_prompt(bills: &mut Bills) { //it has the Bills struct here (the hashmap) so we can access it and input the data

        println!("Please insert the name: ");
        let bill_name = match user_input() {
            Ok(input) => input.to_lowercase(),
            Err(_) => return,
        };
    
        println!("Please insert the amount owing: ");
        let bill_amount = match user_input() {
            Ok(input) => input,
            Err(_) => return,
        };
     
        let bill_amount: f64 = match bill_amount.parse() { //converts string to float for the amount
            Ok(data) => data,
            Err(_) => return,
        };
    
        let new_bill = Bill::new(bill_name, bill_amount); //creates a new instsance of a bill and inputs the data collected
        
        //let new_bill_id = (bills.len() + 1) as u64;

        bills.add_bill(new_bill); //pushes the newly created instance of the bill and pushes it into the add_bill function
                                    // add_bill function pushes the Bill struct into the Bills vector
    
        println!("Bill has been added");
    
    }

    pub fn remove_bill_prompt(bills: &mut Bills) {

        println!("Please type in the name for the bill you would like to remove");  //prompts user to type in the ID
        
        bills.view_bill(); //shows the current list of bills inside the bill hashmap

        let id_input = match user_input() {
            Ok(data) => data,
            Err(_) => return,
        };
    
        if let Some(input) = bills.inner.get(&id_input) {
            println!("You have removed Bill: {}", input.name);
            bills.remove_bill(id_input.as_str());
        }
        else {
            println!("Bill doesnt exist");
            return;
        }
    }

    pub fn edit_bill_prompt(bills: &mut Bills) {

        println!("Please type the ID of the bill in which you would like to edit");

        bills.view_bill(); //shows the current list of bills inside the bill hashmap
    
        let id_input = match user_input() {
            Ok(data) => data,
            Err(_) => return,
        };
    
        if let Some(bill) = bills.inner.get(&id_input) {
            println!("Beginning the process of editing the below bill, please follow the prompts: ");
            println!("Bill: Name: {} || Value: {}",bill.name, bill.owe);
            
            println!("Please insert the name: ");
            let bill_name = match user_input() {
            Ok(input) => input.to_lowercase(),
            Err(_) => return,
            };
    
            println!("Please insert the amount owing: ");
            let bill_amount = match user_input() {
            Ok(input) => input,
            Err(_) => return,
            };
     
            let bill_amount: f64 = match bill_amount.parse() { //converts string to float for the amount
            Ok(data) => data,
            Err(_) => return,
            };
    
            let edited_bill = Bill::new(bill_name, bill_amount);

            bills.edit_bill(id_input.as_str(), edited_bill)
            }
        else {println!("Bill does not exist");}
        
    }
}



fn main() {

    
    let mut bill_list = Bills::new(); //Creating a new instance of Bills to store the bills

    loop {
        menu::main_menu();

        let input =  match user_input(){
            Ok(data) => data,
            Err(_) => return
        };

        match Menu::user_input_menu(input.as_str()) {
            Some(Menu::AddBill) => menu::add_bill_prompt(&mut bill_list),
            Some(Menu::ViewBill) => Bills::view_bill(&mut bill_list),
            Some(Menu::RemoveBill) => menu::remove_bill_prompt(&mut bill_list),
            Some(Menu::EditBill) => menu::edit_bill_prompt(&mut bill_list),
            None => return,
        }
        /* 
        match input.as_str() {
            "1" => {
                println!("You selected: Add a bill option");
                let bill = Bill::add_bill();
                let new_bill_id = (bill_list.len() + 1) as u64;
                bill_list.insert(new_bill_id, bill );
                println!("Bill successfully added!");
            }
            "2" => {
                for (id, bill) in &bill_list {
                    println!("Bill: ID: {} || Name: {} || Amount Owed: {}", id, bill.name, bill.owe);
                }
            }
            "3" => {
                Bill::remove_bill(&mut bill_list);
                println!("Selected bill has been removed");
            }
            "4" => {
                Bill::edit_bill(&mut bill_list);
            }
            &_ => todo!()
        }
        */
    }
}
