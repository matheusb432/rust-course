// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * 1. Bricks:
//      * Colored bricks should print "The brick color is [color]"
//      * Other bricks should print "[Bricktype] brick"
// * 2. Water:
//      * Pressure levels 10 and over should print "High water pressure!"
//      * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * 3. Grass, Dirt, and Sand should all print "Ground tile"
// * 4. Treasure Chests:
//      * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * 5. Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug, PartialEq, Eq)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn main() {
    let red_brick = Tile::Brick(BrickStyle::Red);
    let gray_brick = Tile::Brick(BrickStyle::Gray);
    let brick = Tile::Brick(BrickStyle::Dungeon);
    let high_pressure = Tile::Water(Pressure(20));
    let low_pressure = Tile::Water(Pressure(5));
    let dirt = Tile::Dirt;
    let grass = Tile::Grass;
    let sand = Tile::Sand;
    let wood = Tile::Wood;
    let gold_chest = Tile::Treasure(TreasureChest {
        content: TreasureItem::Gold,
        amount: 150,
    });
    let power_chest = Tile::Treasure(TreasureChest {
        content: TreasureItem::SuperPower,
        amount: 300,
    });
    let tiles: Vec<Tile> = vec![
        red_brick,
        gray_brick,
        brick,
        high_pressure,
        low_pressure,
        dirt,
        grass,
        sand,
        wood,
        gold_chest,
        power_chest,
    ];

    for tile in tiles {
        match tile {
            // NOTE advanced pattern matching to check for multiple conditions
            Tile::Brick(brick) => match brick {
                b @ BrickStyle::Gray | b @ BrickStyle::Red => println!("The brick color is {b:?}"),
                b => println!("{b:?} brick"),
            },
            Tile::Water(pressure) => match pressure {
                Pressure(p) if p >= 10 => println!("High water pressure!"),
                Pressure(p) => println!("Water pressure level: {p:?}"),
            },
            // NOTE Matching on multiple values at once
            Tile::Grass | Tile::Dirt | Tile::Sand => println!("Ground tile"),
            Tile::Treasure(TreasureChest { content, amount })
                if content == TreasureItem::Gold && amount >= 100 =>
            {
                println!("Lots of gold!")
            }
            _ => (),
        }
    }
}
