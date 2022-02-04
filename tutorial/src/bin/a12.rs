enum Color {
    White,
    Black,
    Brown,
    Blue,
}

struct Dimension {
    height: f32,
    width: f32,
    depth: f32,
}

struct ShippingBox {
    weight: f32,
    color: Color,
    dimensions: Dimension,
}

impl Dimension {
    fn print_dimensions(&self) {
        println!(
            "dimensions: {{ height: {:?}, width: {:?}, depth: {:?} }}",
            self.height, self.width, self.depth
        );
    }
}

impl ShippingBox {
    fn create_new_box(weight: f32, color: Color, dimensions: Dimension) -> Self {
        return Self {
            weight: weight,
            color: color,
            dimensions: dimensions,
        };
    }

    fn display_box(&self) {
        println!("weight: {:?}", self.weight);
        match self.color {
            Color::Black => println!("color: black"),
            Color::White => println!("color: white"),
            Color::Blue => println!("color: blue"),
            Color::Brown => println!("color: brown"),
        };
        self.dimensions.print_dimensions();
    }
}

fn main() {
    let dimensions = Dimension {
        height: 10.3,
        width: 5.7,
        depth: 11.9,
    };
    let shipping_box = ShippingBox::create_new_box(50.005, Color::Brown, dimensions);
    shipping_box.display_box();
}
