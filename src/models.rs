pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed,
}

pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<Story>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open and the stories should be an empty vector
        Epic {
            name,
            description,
            status: Status::Open,
            stories: Vec::new(),
        }
    }
}

pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        // by default the status should be set to open
        Story {
            name,
            description,
            status: Status::Open,
        }
    }
}

pub struct DBState {
    // This struct represents the entire db state which includes the last_item_id, epics, and stories
    pub last_item_id: u32,
    pub epics: Vec<Epic>,
    pub stories: Vec<Story>,
}