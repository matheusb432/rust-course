struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 90 },
        Test { score: 100 },
        Test { score: 77 },
        Test { score: 97 },
    ];

    for test in my_scores {
        println!("score = {:?}", test.score);
    }
}
