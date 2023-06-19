// NOTE derive is a special macro for enums or structs
// NOTE Applying the Clone and Copy derives informs the compiler that this should automatically
// * create a copy when stored in a struct or function, meaning ownership would not be transfered.
#[derive(Debug, Clone, Copy)]
enum Position {
    Manager,
    Supervisor,
    Worker,
}

// NOTE Only small structs should have the clone and copy derives, as the operation
// * of copying a big struct can be expensive so it should always be explicit
#[derive(Debug, Clone, Copy)]
struct Employee {
    position: Position,
    work_hours: i64,
}

// NOTE This fn will take ownership of the emp parameter
fn print_employee(emp: Employee) {
    println!("{:?}", emp);
}

fn main() {
    let me = Employee {
        position: Position::Manager,
        work_hours: 40,
    };

    // NOTE With [derive(Debug)], it's possible to print the enum key here
    println!("{:?}", me.position);

    print_employee(me);
    print_employee(me);
}
