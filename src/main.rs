use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use models::Action;
use navigator::*;

fn main() {
    // Initialize: create database and navigator
    let mut navigator = Navigator::new(
        Rc::new(
            JiraDatabase::new("./data/db.json".to_owned())
        )
    );
    
    loop {
        clearscreen::clear().unwrap();

        // 1. get current page from navigator. If there is no current page exit the loop.
        if let Some(current_page) = navigator.get_current_page() {
            // 2. render page
            if let Err(error) = current_page.draw_page() {
                println!("Error rendering page: {}\nPress any key to continue...", error);
                wait_for_key_press();
            } else {
                // 3. get user input
                let input = get_user_input().trim().to_owned();
                // 4. pass input to page's input handler
                let action = current_page.handle_input(&input);
                // 5. if the page's input handler returns an action let the navigator process the action
                match action {
                    Ok(Some(action)) => {
                        if let Err(error) = navigator.handle_action(action) {
                            println!("Error handling action: {}\nPress any key to continue...", error);
                            wait_for_key_press();
                        }
                    },
                    Ok(None) => {
                        println!("Invalid input: {}\nPress any key to continue...", input);
                        wait_for_key_press();
                    },
                    Err(error) => {
                        println!(
                            "Invalid input: {} with error: {}\nPress any key to continue...", 
                            input, error
                        );
                        wait_for_key_press();
                    }
                }
            }
        } else {
            break;
        }
    }
    
}