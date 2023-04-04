fn main() {
    println!("My outfit will be: {:?}", choose_outfit(Some(0), Ok("Dear friend, ...")));
}

// level 2: dress_code (karyun.cheung)

#[derive(Debug, PartialEq, Eq)]
pub enum Jacket {
    Black,
    White,
    Flowers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Hat {
    Snapback,
    Baseball,
    Fedora,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Outfit {
    pub jacket: Jacket,
    pub hat: Hat,
}

pub fn choose_outfit(formality_level: Option<u32>, invitation_message: Result<&str, &str>) -> Outfit {
    match (formality_level, invitation_message) {
        (None, Err(_)) => Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        },
        (None, Ok(_)) => Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Fedora,
        },
        (Some(level), Err(_)) if level > 0 => Outfit {
            jacket: Jacket::White,
            hat: Hat::Snapback,
        },
		(Some(level), Ok(_)) if level > 0 => Outfit {
            jacket: Jacket::White,
            hat: Hat::Fedora,
        },
        (Some(level), Ok(_)) if level < 1 => Outfit {
            jacket: Jacket::Black,
            hat: Hat::Fedora,
        },
        _ => Outfit {
            jacket: Jacket::Black,
            hat: Hat::Snapback,
        },
    }
}
