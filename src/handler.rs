use std::collections::HashMap;

use poise::serenity_prelude::UserId;

// there is a static codes list with 10k codes in it.

// user struct
// has an id, a vec of completed codes positions, and a vec of accepted codes positions
struct User {
    id: UserId,
    completed_codes: Vec<usize>,
    accepted_codes: Vec<usize>,
    rate: i8,
}


// Take in a pointer to a Vec<String> upon initialization and store it in a variable called codes_list.
pub struct CodeRaidHandler {
    // owner_id: UserId,
    codes_list: Vec<String>,
    available_codes: Vec<usize>,
    users: HashMap<UserId, User>,
}

impl CodeRaidHandler {
    pub fn new(
        // owner_id: UserId,
        codes_list: Vec<String>,
    ) -> Self {
        let mut available_codes: Vec<usize> = Vec::new();
        for i in 0..codes_list.len() {
            available_codes.insert(0, i);
            // available_codes.push(i as i32);
        }
        // available_codes.reverse();
        Self {
            // owner_id,
            codes_list,
            available_codes,
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, user_id: UserId, rate: i8) {
        // insert a new user into the users HashMap
        // if the user already exists, do nothing
        if !self.users.contains_key(&user_id) {
            let user = User {
                id: user_id,
                completed_codes: Vec::new(),
                accepted_codes: Vec::new(),
                rate,
            };
            self.users.insert(user_id, user);
        }
    }

    // deopt user
    // change their rate
    // opt user back
    // return new codes for user
    pub fn change_user_rate(&mut self, user_id: UserId, rate: i8) -> Vec<String> {
        self.deopt_user(user_id);
        let user = self.users.get_mut(&user_id).unwrap();
        user.rate = rate;
        self.opt_user(user_id, rate);
        self.get_user_codes(user_id)
    }

    // puts all accepted codes back into the available codes list
    pub fn deopt_user(&mut self, user_id: UserId) {
        if let Some(user) = self.users.get_mut(&user_id) {
            user.accepted_codes.reverse();
            for code in user.accepted_codes.iter() {
                self.available_codes.push(*code);
            }
            user.accepted_codes.clear();
        }
    }

    pub fn opt_user(&mut self, user_id: UserId, codes: i8) {
        if let Some(user) = self.users.get_mut(&user_id) {
            for _ in 0..codes {
                // get the first available code
                if let Some(code) = self.available_codes.pop() {
                    user.accepted_codes.push(code);
                }
            }
        }
    }

    pub fn submit_codes(&mut self, user_id: UserId) -> Vec<String> {
        if let Some(user) = self.users.get_mut(&user_id) {
            // add all accepted codes to the completed codes list
            for code in user.accepted_codes.iter() {
                user.completed_codes.push(*code);
            }
            // remove all codes from the user's accepted codes list
            user.accepted_codes.clear();

            // add more codes to the user's accepted codes list based on their rate
            for _ in 0..user.rate {
                // get the first available code
                if let Some(code) = self.available_codes.pop() {
                    user.accepted_codes.push(code);
                }
            }
            // user.accepted_codes.reverse();
        }
        self.get_user_codes(user_id)
    }


    pub fn get_user_codes(&self, user_id: UserId) -> Vec<String> {
        // make a new vec to store the codes
        let mut codes = Vec::new();
        if let Some(user) = self.users.get(&user_id) {
            // add all accepted codes to the completed codes list
            for code in user.accepted_codes.iter() {
                codes.push(self.codes_list[*code as usize].clone());
            }
        }
        codes
    }

    pub fn get_user_completed_codes(&self, user_id: UserId) -> Vec<usize> {
        if let Some(user) = self.users.get(&user_id) {
            return user.completed_codes.clone();
        }
        Vec::new()
    }

    pub fn get_uncompleted_codes(&self) -> Vec<String> {
        let mut codes = Vec::new();
        let mut completed_codes = Vec::new();
        for user in self.users.values() {
            for code in user.completed_codes.iter() {
                completed_codes.push(*code);
            }
        }
        // go through self.codes_list and if it is not in the completed codes list, add it to the codes list
        for i in 0..self.codes_list.len() {
            if !completed_codes.contains(&i) {
                codes.push(self.codes_list[i].clone());
            }
        }
        codes
    }
}
