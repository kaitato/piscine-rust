use json::{object};
pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub proteins: f64,
    pub fats: f64,
    pub carbs: f64,
    pub nbr_of_portions: f64,
}
fn round_to_decimal(num: f64, decimal_places: i32) -> f64 {
    let multiplier = 10_f64.powi(decimal_places);
    (num * multiplier).round() / multiplier
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let calories = foods.iter()
        .fold((0.0,0.0,0.0,0.0),| mut acc, food: &Food| {
            acc.0 += food.calories[1]
                .replace("kcal", "")
                .parse::<f64>().unwrap() * food.nbr_of_portions;
            acc.1 += food.carbs * food.nbr_of_portions;
            acc.2 += food.proteins * food.nbr_of_portions;
            acc.3 += food.fats *food.nbr_of_portions;
            println!("{:?}", acc);
            acc
            });
    let result = object!{
        cals: round_to_decimal(calories.0, 2),
       carbs: round_to_decimal(calories.1, 2),
        proteins: round_to_decimal(calories.2, 2),
        fats: round_to_decimal(calories.3, 2),
    };
    result
    
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let a = vec![
        Food {
            name: String::from("big mac"),
            calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
            proteins: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            name: "pizza margherita".to_string(),
            calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

    println!("{:#}", calculate_macros(a));
    }
}