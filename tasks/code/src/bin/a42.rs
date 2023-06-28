// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * 1. Write a program that uses an iterator to generate a score multiplier
// * 2. The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity

struct Stage;

struct Multiplier {
    powerup: usize,
    amount: usize,
    per_iter_base: usize,
}

impl Multiplier {
    pub fn new() -> Self {
        Self {
            powerup: 0,
            amount: 0,
            per_iter_base: 1,
        }
    }
}

impl Iterator for Multiplier {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.amount += self.per_iter_base + self.powerup;
        Some(self.amount)
    }
}

fn main() {
    let mut game = Multiplier::new();

    // ? Iterating via next()
    print!("{:?}\t", game.next());
    print!("{:?}\t", game.next());
    print!("{:?}\t", game.next());

    println!("\nPowering up!");
    game.powerup += 1;

    print!("{:?}\t", game.next());
    print!("{:?}\t", game.next());
    print!("{:?}\t", game.next());

    println!("\nPowering up!");
    game.powerup += 1;

    print!("{:?}\t", game.next());
}
