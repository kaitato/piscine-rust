fn main() {
    let car_rental = RentalBusiness {
        car: RefCell::new(Car {
            color: "red".to_string(),
            plate: "AAA".to_string(),
        }),
    };

    println!("{:?}", car_rental.rent_car());
    println!("{:?}", car_rental.repair_car());

    {
        let mut car = car_rental.repair_car();
        car.color = "blue".to_string();
    }

    println!("{:?}", car_rental.rent_car());

    car_rental.change_car(Car {
        color: "pink".to_string(),
        plate: "WWW".to_string(),
    });

    println!("{:?}", car_rental.rent_car());

    println!("{:?}", car_rental.sell_car());
    println!("{:?}", car_rental.sell_car());
}
// level 6: car_rental (karyun.cheung)

use std::cell::Ref;
use std::cell::RefCell;
use std::cell::RefMut;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Car {
    pub color: String,
    pub plate: String,
}

#[derive(Debug)]
pub struct RentalBusiness {
    pub car: RefCell<Car>,
}

impl RentalBusiness {
    pub fn rent_car(&self) -> Ref<Car> {
		self.car.borrow()
	}
    pub fn sell_car(&self) -> Car {
		self.car.replace(Car::default())
	}
    pub fn repair_car(&self) -> RefMut<Car> {
		self.car.borrow_mut()
	}
    pub fn change_car(&self, new_car: Car) {
		*self.car.borrow_mut() = new_car
	}
}