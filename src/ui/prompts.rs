use crate::{models::{Epic, Story, Status}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("----------------------------");
    println!("Enter epic name: ");
    let name = get_user_input();
    println!("Enter epic description: ");
    let description = get_user_input();
    Epic::new(name, description)
}

fn create_story_prompt() -> Story {
    println!("----------------------------");
    println!("Enter story name: ");
    let name = get_user_input();
    println!("Enter story description: ");
    let description = get_user_input();
    Story::new(name, description)
}

fn delete_epic_prompt() -> bool {
    println!("----------------------------");
    println!(
        "Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:"
    );
    if let Some(input) = get_user_input().chars().next() {
        input.to_ascii_lowercase() == 'y'
    } else {
        false
    }
}

fn delete_story_prompt() -> bool {
    println!("----------------------------");
    println!("Are you sure you want to delete this story? [Y/n]:");
    if let Some(input) = get_user_input().chars().next() {
        input.to_ascii_lowercase() == 'y'
    } else {
        false
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("----------------------------");
    println!("New Status: (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED):");
    if let Some(input) = get_user_input().chars().next() {
        match input {
            '1' => Some(Status::Open),
            '2' => Some(Status::InProgress),
            '3' => Some(Status::Resolved),
            '4' => Some(Status::Closed),
            _ => None
        }
    } else {
        None
    }
}