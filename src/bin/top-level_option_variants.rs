fn main() {
    let item_is_available = is_item_in_stock();

    match item_is_available {
        Some(true) => println!("Yes, the item is in stock and available"),
        Some(false) => println!("No, the item is not in stock"),
        None => println!("We're unable to find the item that you're looking for"),
    }
}

fn is_item_in_stock() -> Option<bool> {
    let item_exist_in_catalog = true;
    let item_is_in_stock = false;

    if item_exist_in_catalog && item_is_in_stock {
        Some(true)
    } else if item_exist_in_catalog && !item_is_in_stock {
        Some(false)
    } else {
        None
    }
}