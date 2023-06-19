// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * 1. Use a struct to encapsulate the box characteristics
// * 2. Use an enum for the box color
// * 3. Implement functionality on the box struct to create a new box
// * 4. Implement functionality on the box struct to print the characteristics

// * 1.
struct ShippingBox {
    dimensions: Dimensions,
    weight_kg: u32,
    color: Color,
}

struct Dimensions {
    width_cm: u32,
    height_cm: u32,
}

impl Dimensions {
    fn square(size: u32) -> Self {
        Dimensions {
            width_cm: size,
            height_cm: size,
        }
    }

    fn as_meters_tuple(&self) -> (f64, f64) {
        let width_m = float_division(self.width_cm, 100);
        let height_m = float_division(self.height_cm, 100);

        (width_m, height_m)
    }

    fn get_info(&self) -> String {
        let (width_m, height_m) = self.as_meters_tuple();

        format!("\tWidth: {width_m} meters\n\tHeight: {height_m} meters")
    }
}

fn float_division(x: u32, y: u32) -> f64 {
    return x as f64 / y as f64;
}

impl ShippingBox {
    fn new(weight_kg: u32, color: Color, dimensions: Dimensions) -> Self {
        // NOTE implementations can also call their functions via Self
        Self {
            dimensions,
            weight_kg,
            color,
        }
    }

    // * 3.
    fn new_square(size_cm: u32, weight_kg: u32, color: Color) -> Self {
        Self {
            dimensions: Dimensions::square(size_cm),
            weight_kg,
            color,
        }
    }

    fn new_black_square(size_cm: u32, weight_kg: u32) -> Self {
        Self::new_square(size_cm, weight_kg, Color::Black)
    }

    fn new_orange_square(size_cm: u32, weight_kg: u32) -> Self {
        Self::new_square(size_cm, weight_kg, Color::Orange)
    }

    // * 4.
    fn print(&self) {
        let color_text = self.color.to_str();
        let dimensions_info = self.dimensions.get_info();

        println!(
            "Dimensions: \n{}\nWeight: {} kilograms\nColor: {}",
            dimensions_info, self.weight_kg, color_text
        )
    }
}

// * 2.
enum Color {
    Orange,
    Black,
}

impl Color {
    fn to_str(&self) -> &str {
        match self {
            Color::Orange => "Orange",
            Color::Black => "Black",
        }
    }
}

fn main() {
    let new_box = ShippingBox::new(
        100,
        Color::Black,
        Dimensions {
            width_cm: 100,
            height_cm: 20,
        },
    );
    let black_square_box = ShippingBox::new_black_square(150, 90);
    let orange_square_box = ShippingBox::new_orange_square(275, 150);

    new_box.print();
    black_square_box.print();
    black_square_box.print();
    orange_square_box.print();
}
