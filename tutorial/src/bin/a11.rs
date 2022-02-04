struct GroceryItem {
    id: i32,
    quantity: i32,
}

fn print_id(grocery_item: &GroceryItem) {
    println!("id: {:?}", grocery_item.id);
}

fn print_quantity(grocery_item: &GroceryItem) {
    println!("quantity: {:?}", grocery_item.quantity);
}

fn main() {
    let grocery_item = GroceryItem {
        id: 25,
        quantity: 12,
    };
    print_id(&grocery_item);
    print_quantity(&grocery_item);
}
