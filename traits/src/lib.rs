pub use std::fmt;

#[derive(Debug)]
pub struct Player {
	pub name: String,
	pub strength: f64,
	pub score: i32,
	pub money: i32,
	pub weapons: Vec<String>,
}
impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{} {} {}\n{:?}", self.name, self.strength, self.score, self.money, self.weapons)

    }
}

pub struct Fruit {
	pub weight_in_kg: f64,
}

pub struct Meat {
	pub weight_in_kg: f64,
	pub fat_content: f64,
}

impl Player {
	fn eat<T: Food>(&mut self, food: T) {
		self.strength += food.gives();
	}
}

pub trait Food {
	fn gives(&self) -> f64;
}

impl Food for Fruit {
    fn gives(&self) -> f64 {
        self.weight_in_kg * 4.
    }
}

impl Food for Meat {
    fn gives(&self) -> f64 {
        let protein = (1. - self.fat_content) * self.weight_in_kg * 4.;
        let fat = self.fat_content * self.weight_in_kg * 9.;
        protein + fat
    }
}


#[test]
fn test_gives() {
    let apple = Fruit { weight_in_kg: 1.0 };
    assert_eq!(apple.gives(), 4.0);
    let steak = Meat {
        weight_in_kg: 1.0,
        fat_content: 1.0,
    };
    assert_eq!(steak.gives(), 9.0);

    let steak = Meat {
        weight_in_kg: 1.0,
        fat_content: 0.0,
    };
    assert_eq!(steak.gives(), 4.0);

    let steak = Meat {
        weight_in_kg: 1.5,
        fat_content: 0.3,
    };
    assert_eq!(steak.gives(), 8.25);
}

#[test]
fn test_eat() {
    let apple = Fruit { weight_in_kg: 1.0 };
    assert_eq!(apple.gives(), 4.0);
    let steak = Meat {
        weight_in_kg: 1.0,
        fat_content: 1.0,
    };

    let mut player1 = Player {
        name: String::from("player1"),
        strength: 1.0,
        score: 0,
        money: 0,
        weapons: vec![String::from("knife")],
    };
    player1.eat(apple);
    assert_eq!(player1.strength, 5.0);
    player1.eat(steak);
    assert_eq!(player1.strength, 14.0);
}

#[test]
fn test_display() {
    let player1 = Player {
        name: String::from("player1"),
        strength: 1.0,
        score: 0,
        money: 0,
        weapons: vec![String::from("knife"), String::from("shotgun")],
    };
    println!("{}", player1);
    assert_eq!(
        player1.to_string(),
        "player1\nStrength: 1, Score: 0, Money: 0\nWeapons: [\"knife\", \"shotgun\"]"
    )
}