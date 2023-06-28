// Topic: Smart Pointers & RefCell
//
// Summary:
//   A vehicle rental company wants to access the rentals available
//   at storefront locations. Create a program that provides access
//   to storefront rentals from the corporate headquarters.
//
// Requirements:
// * 1. Corporate must be able to access the rentals at a storefront
// * 2. Storefronts must be able to rent out vehicles
// * 3. Rentals have the following attributes:
//   - Type of vehicle
//   - Vehicle Identification Number (VIN)
//   - Vehicle status:
//     * Available, Unavailable, Maintenance, Rented
//
// Notes:
// * Use Rc and RefCell to create shared mutable data structures
// * Create at least two rentals and ensure that Corporate and StoreFront
//   can both access the rental information
// * Test your program by changing the vehicle status from both a storefront
//   and from corporate

// ? Disabling warnings for the entire file with `#!`
#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

struct Corporate {
    rentals: Rentals,
}

struct StoreFront {
    rentals: Rentals,
}

type Rentals = Rc<RefCell<Vec<VehicleRental>>>;

#[derive(Debug)]
enum VehicleType {
    Car,
    Bike,
}
#[derive(Debug, PartialEq, Eq)]
enum VehicleStatus {
    Available,
    Unavailable,
    Maintenance,
    Rented,
}

#[derive(Debug)]
struct VehicleRental {
    pub status: VehicleStatus,
    vehicle_type: VehicleType,
    vin: String,
}

impl VehicleRental {
    pub fn new_car(vin: String) -> Self {
        Self {
            vehicle_type: VehicleType::Car,
            vin,
            status: VehicleStatus::Available,
        }
    }

    pub fn new_bike(vin: String) -> Self {
        Self {
            vehicle_type: VehicleType::Bike,
            vin,
            status: VehicleStatus::Available,
        }
    }
}

fn create_rentals() -> Vec<VehicleRental> {
    vec![
        VehicleRental::new_car("100".to_owned()),
        VehicleRental::new_bike("200".to_owned()),
        VehicleRental::new_car("101".to_owned()),
    ]
}

fn main() {
    let rentals: Rentals = Rc::new(RefCell::new(create_rentals()));

    let store = StoreFront {
        // ? Desugared equivalent
        // rentals: Rc::clone(&rentals),
        // NOTE Rc::clone is a shallow copy, meaning it only clones the pointer itself
        rentals: rentals.clone(),
    };
    let mifflin = Corporate {
        rentals: rentals.clone(),
    };
    {
        let rentals = mifflin.rentals.borrow();
        println!("Corporate: {:?}", rentals);
    }
    // NOTE it's necessary to limit the scope of the mutable borrow so that corporate_rentals can be borrowed immutably
    {
        let mut rentals = store.rentals.borrow_mut();
        rentals.remove(0);

        if let Some(rental) = rentals.first_mut() {
            rental.status = VehicleStatus::Rented;
        }
    }

    let rentals = mifflin.rentals.borrow();
    println!("Corporate: {:?}", rentals);
}

mod test {

    use super::*;
    use std::cell;

    #[test]
    fn update_status() {
        let vehicles = vec![
            VehicleRental {
                status: VehicleStatus::Available,
                vehicle_type: VehicleType::Car,
                vin: "100".to_owned(),
            },
            VehicleRental {
                status: VehicleStatus::Maintenance,
                vehicle_type: VehicleType::Bike,
                vin: "abc".to_owned(),
            },
        ];
        let vehicles: Rentals = Rc::new(RefCell::new(vehicles));

        let corp = Corporate {
            rentals: vehicles.clone(),
        };
        let store = StoreFront {
            rentals: vehicles.clone(),
        };

        {
            let mut rentals = store.rentals.borrow_mut();

            // ? .get_mut(0) gets a mutable reference to the 0 indexed in the vector
            if let Some(rental) = rentals.get_mut(0) {
                assert_eq!(rental.status, VehicleStatus::Available);
                rental.status = VehicleStatus::Rented;
                assert_eq!(rental.status, VehicleStatus::Rented);
            }
        }
        {
            let mut rentals = corp.rentals.borrow_mut();

            // ? .get_mut(0) gets a mutable reference to the 0 indexed in the vector
            if let Some(rental) = rentals.get_mut(0) {
                assert_eq!(rental.status, VehicleStatus::Rented);
                rental.status = VehicleStatus::Available;
            }
        }

        {
            let rentals: cell::Ref<'_, Vec<VehicleRental>> = store.rentals.borrow();
            if let Some(rental) = rentals.get(0) {
                assert_eq!(rental.status, VehicleStatus::Available);
            }
        }
    }
}
