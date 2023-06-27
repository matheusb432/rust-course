// Topic: Lifetimes & Structures
//
// Requirements:
// * 1. Display just the names and titles of persons from the mock-data.csv file
// * 2. The names & titles must be stored in a struct separately from the mock
//   data for potential later usage
// * 3. None of the mock data may be duplicated in memory
//
// Notes:
// * The mock data has already been loaded with the include_str! macro, so all functionality
//   must be implemented using references/borrows

use std::collections::HashMap;

const TITLE_KEY: &'static str = "title";
const NAME_KEY: &'static str = "first_name";

const MOCK_DATA: &'static str = include_str!("mock-data.csv");

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    title: &'a str,
}

struct People<'a> {
    inner: Vec<Person<'a>>,
}
impl<'a> People<'a> {
    pub fn new(inner: Vec<Person<'a>>) -> Self {
        Self { inner }
    }
}

// ? This can be much simpler, but I tried to make this as efficient as possible in the case of a really large CSV file
fn read_people_csv(csv_data: &'static str) -> Option<Vec<Person<'_>>> {
    let lines: Vec<&str> = csv_data.split('\n').collect();

    let (Some(name_idx), Some(title_idx)) = get_indexes(&lines) else {
        println!("Could not find title or name keys from csv!");
        return None;
    };
    let (name_order, title_order) = get_orders(&name_idx, &title_idx);

    let people = lines[1..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split(",")
                .enumerate()
                .filter(|&(i, _)| i == name_idx || i == title_idx)
                .map(|(_, s)| s.trim())
                .collect::<Vec<_>>()
        })
        .map(|row_line| Person {
            name: row_line[name_order],
            title: row_line[title_order],
        })
        .collect();

    Some(people)
}

fn get_orders(name_idx: &usize, title_idx: &usize) -> (usize, usize) {
    let mut indexes = vec![name_idx, title_idx];
    indexes.sort();
    let mut map = HashMap::new();
    map.insert(indexes[0], 0 as usize);
    map.insert(indexes[1], 1 as usize);
    let name_order = *map.get(name_idx).unwrap();
    let title_order = *map.get(title_idx).unwrap();

    return (name_order, title_order);
}

fn get_indexes(lines: &Vec<&str>) -> (Option<usize>, Option<usize>) {
    let mut name_idx = None;
    let mut title_idx = None;
    let header_row = lines.first();

    for (i, prop) in header_row.unwrap().split(",").enumerate() {
        match prop.trim() {
            NAME_KEY => name_idx = Some(i),
            TITLE_KEY => title_idx = Some(i),
            _ => {}
        }
    }
    (name_idx, title_idx)
}

fn main() {
    // * 1. 2.
    if let Some(csv_people) = read_people_csv(MOCK_DATA) {
        let people = People::new(csv_people);

        for person in people.inner {
            println!("Name: {:?}\tTitle: {:?}", person.name, person.title);
        }
    } else {
        panic!("Couldn't parse mock-data.csv!");
    }
}
