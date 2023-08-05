use std::{borrow::Cow, cell::RefCell, rc::Rc, sync::Arc};

#[derive(Debug)]
enum MenuItem {
    Drink,
    Salad,
}

#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: u32,
}

#[derive(Debug)]
struct TableOrder {
    items: Vec<ItemOrder>,
}

fn new_table_order() -> TableOrder {
    TableOrder {
        items: vec![ItemOrder {
            item: MenuItem::Drink,
            quantity: 1,
        }],
    }
}

// ? Type alias for more concise code
type Order = Rc<RefCell<Vec<TableOrder>>>;

#[derive(Debug)]
struct Chef(Order);
#[derive(Debug)]
struct WaitStaff(Order);
#[derive(Debug)]
struct Accounting(Order);

fn main() {
    let orders: Order = Rc::new(RefCell::new(vec![]));

    // NOTE Rc::clone creates a new reference to the same data
    let chef = Chef(Rc::clone(&orders));
    let wait_staff = WaitStaff(Rc::clone(&orders));
    let accounting = Accounting(Rc::clone(&orders));

    let order = new_table_order();

    {
        // ? Will move order to `chef`, `wait_staff` and `accounting`
        orders.borrow_mut().push(order);
    }
    dbg!(chef.0.borrow());
    drop(chef);

    dbg!(wait_staff.0.borrow());
    drop(wait_staff);

    // ? Will still be able to access the data since the smart pointer is still alive (has at least one reference)
    dbg!(accounting.0.borrow());

    // NOTE Memory in bytes of some smart pointers constructs compared to a string slice
    dbg!(std::mem::size_of::<&str>());
    dbg!(std::mem::size_of::<String>());
    dbg!(std::mem::size_of::<Box<str>>());
    dbg!(std::mem::size_of::<Arc<str>>());
    dbg!(std::mem::size_of::<Cow<'_, str>>());
}
