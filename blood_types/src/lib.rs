#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            other => Err(format!("`{}` is not a valid antigen", other)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(rhf: &str) -> Result<Self, Self::Err> {
        match rhf {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            o => Err(format!("`{}` is not a valid Rh Factor", o)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        // if self.rh_factor == other.rh_factor {
        //     return self.antigen.cmp(&other.antigen);
        // }
        self.antigen.cmp(&other.antigen)
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(blood_type: &str) -> Result<Self, Self::Err> {
        if blood_type.len() > 3 || blood_type.len() < 2 {
            return Err(format!(
                "Invalid antigen: `{}` invalid length: {}",
                blood_type,
                blood_type.len()
            ));
        }

        let rh_factor_string = blood_type.get(blood_type.len() - 1..);

        if let None = rh_factor_string {
            return Err(format!("Invalid suffix {:?}", rh_factor_string));
        }

        let rh_factor = rh_factor_string.unwrap().parse()?;
        let antigen = blood_type.get(..blood_type.len() - 1).unwrap().parse()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.antigen)?;

        if self.rh_factor == RhFactor::Positive {
            return write!(f, "+");
        }

        write!(f, "-")
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if self.rh_factor != other.rh_factor && self.rh_factor == RhFactor::Negative {
            return false;
        }
        if other.antigen == Antigen::O {
            return true;
        }
        self.antigen == Antigen::AB || other.antigen == self.antigen
    }

pub fn donors(&self) -> Vec<Self> {
        let mut blood_types = Vec::new();
        let mut antigens = if self.antigen == Antigen::O {
            vec![Antigen::O]
        } else {
            vec![Antigen::O, self.antigen.clone()]
        };

        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Negative]
        } else {
            vec![RhFactor::Positive, RhFactor::Negative]
        };

        if self.antigen == Antigen::AB {
            antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                })
            }
        }

        blood_types
    }

     pub fn recipients(&self) -> Vec<Self> {
        let antigens = if self.antigen != Antigen::AB {
            vec![Antigen::AB, self.antigen.clone()]
        } else {
            vec![Antigen::AB]
        };
        
        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Positive, RhFactor::Negative]
        } else {
            vec![RhFactor::Positive]
        };
        
        let blood_types = rh_factors.into_iter().flat_map(|rh_factor| {
            antigens.iter().map(move |antigen| Self { rh_factor: rh_factor.clone(), antigen: antigen.clone() })
        }).collect();
        
        blood_types
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let blood_type: BloodType = "O+".parse().unwrap();
        println!("recipients of O+ {:?}", blood_type.recipients());
        println!("donors of O+ {:?}", blood_type.donors());
        let another_blood_type: BloodType = "A-".parse().unwrap();
        println!(
		"donors of O+ can receive from {:?} {:?}",
		&another_blood_type,
		blood_type.can_receive_from(&another_blood_type)
	);
    }
}