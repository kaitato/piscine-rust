pub struct One {
    pub first_layer: Option<Two>,
}
pub struct Two {
    pub second_layer: Option<Three>,
}
pub struct Three {
    pub third_layer: Option<Four>
}
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(&self) -> Option<u16> {
        match &self.first_layer {
            Some(first) => match &first.second_layer {
                Some(second) => match &second.third_layer {
                    Some(third) => match &third.fourth_layer {
                        Some(fourth) => Some(*fourth),
                        None => None,
                    },
                    None => None,
                },
                None => None,
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let a = One {
        first_layer : Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four {
                    fourth_layer: Some(1000)
                })
            })
        })
    };

    // output: 1000
    println!("{:?}", match a.get_fourth_layer() {
        Some(e) => e,
        None => 0
    })
    }
}
