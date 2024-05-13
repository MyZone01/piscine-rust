// Instructions:
// In this exercise, you will be provided with a json file commits.json(download) with data corresponding to git commits in GitHub (extracted using the GitHub rest API). Your objective is to extract the relevant data and place it in a struct called CommitData.

// Create two functions:

// commits_per_author: which returns a hash map with the number of commits per author. The auditors will be identified by their GitHub login.
// commits_per_week: which returns a hash map with the number of commits per week.
// A week is represented by the year followed by the number of the week. For example, January 1, 2020 is in week 1 of 2020 and will be represented by a String with the form "2020-W1".

// Expected functions
// pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
// }

// pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
// }
// Usage
// Here is a possible test for your function:

// use commits_stats::{commits_per_week, commits_per_author};

// fn main() {
// 	let contents = fs::read_to_string("commits.json").unwrap();
// 	let serialized = json::parse(&contents).unwrap();
// 	println!("{:?}", commits_per_week(&serialized));
// 	println!("{:?}", commits_per_author(&serialized));
// }
// And its output:

// $ cargo run
// {"2020-W44": 5, "2020-W36": 1, "2020-W31": 1, ... ,"2020-W45": 4, "2020-W46": 4}
// {"homembaixinho": 2, "mwenzkowski": 3, ... ,"tamirzb": 1, "paul-ri": 2, "RPigott": 1}
// $

use chrono::*;
use json::JsonValue;
use std::collections::HashMap;

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_week = HashMap::new();
    for commit in data.members() {
        let date = commit["commit"]["author"]["date"].as_str().unwrap();
        let date = date.split('T').collect::<Vec<&str>>()[0];
        let date = date.split('-').collect::<Vec<&str>>();
        let year = date[0];
        let month = date[1];
        let day = date[2];
        let week = chrono::NaiveDate::from_ymd_opt(
            year.parse().unwrap(),
            month.parse().unwrap(),
            day.parse().unwrap(),
        )
        .unwrap()
        .iso_week()
        .week();
        let week = format!("{}-W{}", year, week);
        let count = commits_per_week.entry(week).or_insert(0);
        *count += 1;
    }
    commits_per_week
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author = HashMap::new();
    for commit in data.members() {
        let author = commit["author"]["login"].as_str().unwrap();
        let count = commits_per_author.entry(author.to_string()).or_insert(0);
        *count += 1;
    }
    commits_per_author
}
