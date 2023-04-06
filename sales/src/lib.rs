
#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart { 
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some(item_price) = s.products.iter().find(|(name, _)| *name == ele).map(|(_, price)| *price) {
            self.items.push((ele, item_price));
        }
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        // self.items.sort_by(|(_,a), (_,b)| a.partial_cmp(b).unwrap());
        // let receipt:Vec<f32> = self.items.iter().map(|(_,a)|*a).collect();
        // let sum:f32 = receipt.iter().sum();
        // receipt
        // pub fn generate_receipt(&mut self) -> Vec<f32> {
            let mut discount_items: Vec<f32> = Vec::new();
            self.items
                .sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());
            self.receipt = self.items.iter().map(|(_, t)| *t).collect();
            let sets_of_3 = self.receipt.clone().len() / 3;
            let mut cheapest_items: f32 = 0.0;
            for i in 0..sets_of_3 {
                cheapest_items += self.receipt.clone()[i]
            }
            let total: f32 = self.receipt.clone().iter().sum();
            let multiplier: f32 = (total - cheapest_items) / total;
            discount_items = self
                .receipt
                .clone()
                .iter()
                .map(|t| ((*t * multiplier) * 100.0).round() / 100.0)
                .collect();
            discount_items.sort_by(|a, b| a.partial_cmp(b).unwrap());
                self.receipt = discount_items.clone();
                self.receipt.clone()
            // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let store = Store::new(vec![
            (String::from("product A"), 1.23),
            (String::from("product B"), 23.1),
            (String::from("product C"), 3.12),
            (String::from("product D"), 9.75),
            (String::from("product E"), 1.75),
            (String::from("product F"), 23.75),
            (String::from("product G"), 2.75),
            (String::from("product H"), 1.64),
            (String::from("product I"), 15.23),
            (String::from("product J"), 2.10),
            (String::from("product K"), 54.91),
            (String::from("product L"), 43.99),
            ]);
    
        println!("{:?}", store);
    
        let mut cart = Cart::new();
        cart.insert_item(&store, String::from("product A"));
        cart.insert_item(&store, String::from("product B"));
        cart.insert_item(&store, String::from("product C"));
        cart.insert_item(&store, String::from("product D"));
        cart.insert_item(&store, String::from("product E"));
        cart.insert_item(&store, String::from("product F"));
    
        println!("{:?}", cart.generate_receipt());
    
        println!("{:?}", cart);
    }
}
