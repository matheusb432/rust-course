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

use code::hmap;

const TITLE_KEY: &str = "title";
const NAME_KEY: &str = "first_name";

const MOCK_DATA: &str = include_str!("mock-data.csv");

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

// ? Into impl so that Vec<Person> can be converted into People directly
impl<'a> Into<People<'a>> for Vec<Person<'a>> {
    fn into(self) -> People<'a> {
        People::new(self)
    }
}

// ? IntoIterator impl so that People can directly be used in a for..in loop
impl<'a> IntoIterator for People<'a> {
    type Item = Person<'a>;
    type IntoIter = std::vec::IntoIter<Person<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

// ? This can be much simpler, but I tried to make this as efficient as possible in the case of a really large CSV file
fn read_people_csv(csv_data: &'static str) -> Option<Vec<Person<'_>>> {
    let lines: Vec<&str> = csv_data.split('\n').collect();

    //  NOTE .copied() is equivalent to .map(|f| *f) for an Option
    let (Some(name_idx), Some(title_idx)) = get_indexes(lines.first().copied()) else {
        println!("Could not find title or name keys from csv!");
        return None;
    };
    let (name_order, title_order) = get_orders(&name_idx, &title_idx);

    let people = lines[1..lines.len() - 1]
        .iter()
        .map(|line| {
            line.split(',')
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
    let map = hmap! { indexes[0] => 0, indexes[1] => 1, };
    let name_order = *map.get(name_idx).expect("name index not found");
    let title_order = *map.get(title_idx).expect("title index not found");

    (name_order, title_order)
}

fn get_indexes(header_row: Option<&str>) -> (Option<usize>, Option<usize>) {
    let mut name_idx = None;
    let mut title_idx = None;

    for (i, prop) in header_row
        .expect("csv file is empty!")
        .split(',')
        .enumerate()
    {
        match prop.trim() {
            NAME_KEY => name_idx = Some(i),
            TITLE_KEY => title_idx = Some(i),
            _ => (),
        }
    }
    (name_idx, title_idx)
}

fn main() {
    // ? 1. 2.
    let people: People = read_people_csv(MOCK_DATA)
        .expect("Couldn't parse mock-data.csv!")
        // ? Using the Into impl to convert Vec<Person> into People
        .into();

    // ? Using the IntoIterator impl to iterate over People
    for person in people {
        println!("Name: {:?}\tTitle: {:?}", person.name, person.title);
    }
}
