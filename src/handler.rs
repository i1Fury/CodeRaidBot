use poise::serenity_prelude::UserId;

// there is a static codes list with 10k codes in it.

// user struct
// has an id, a vec of completed codes positions, and a vec of accepted codes positions
struct User {
    id: UserId,
    completed_codes: Vec<usize>,
    accepted_codes: Vec<usize>,
}


// Take in a pointer to a Vec<String> upon initialization and store it in a variable called codes_list.
pub struct CodeRaidHandler {
    codes_list: &'static Vec<String>,
}

impl CodeRaidHandler {
    pub fn new(codes_list: &'static Vec<String>) -> Self {
        Self { codes_list }
    }
}