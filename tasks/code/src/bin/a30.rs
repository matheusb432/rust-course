// Topic: Generics & Structures
//
// Requirements:
// * 1. Create a Vehicle structure that is generic over traits Body and Color
// * 2. Create structures for vehicle bodies and vehicle colors and implement the
//      Body and Color traits for these structures
// * 3. Implement a 'new' function for Vehicle that allows it to have any body and any color
// * 4. Create at least two different vehicles in the main function and print their info
//
// Notes:
// * Examples of car bodies can be Truck, Car, Scooter
// * Examples of colors could be red, white, black
// * It is not necessary to have data fields or function implementations
//   for the vehicle bodies/colors

#![allow(dead_code)]

// ? 1.
#[derive(Debug)]
struct Vehicle<B, C>
where
    B: Body,
    C: Color,
{
    body: B,
    color: C,
}

impl<B, C> Vehicle<B, C>
where
    B: Body,
    C: Color,
{
    // ? 3.
    pub fn new(body: B, color: C) -> Self {
        Self { body, color }
    }
}

trait Body {}
trait Color {}

// ? 2.
#[derive(Debug)]
struct Truck;
impl Body for Truck {}
#[derive(Debug)]
struct Car;
impl Body for Car {}

#[derive(Debug)]
struct Red;
impl Color for Red {}
#[derive(Debug)]
struct White;
impl Color for White {}

fn main() {
    let white_car = Vehicle::new(Car, White);
    let red_truck = Vehicle::new(Truck, Red);

    // ? 4.
    dbg!(white_car);
    dbg!(red_truck);
}
