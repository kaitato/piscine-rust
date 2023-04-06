#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThrowObject {
    pub init_position: Object,
    pub init_velocity: Object,
    pub actual_position: Object,
    pub actual_velocity: Object,
    pub time: f32,
}

impl ThrowObject {
    pub fn new(init_position: Object, init_velocity: Object) -> ThrowObject {
        ThrowObject {
            init_position: init_position.clone(),
            init_velocity: init_velocity.clone(),
            actual_position: init_position,
            actual_velocity: init_velocity,
            time: 0.0,
        }
    }
}

impl Iterator for ThrowObject {
    type Item = ThrowObject;

    fn next(&mut self) -> Option<Self::Item> {
        let g = -9.81; // acceleration due to gravity in m/s^2
        let t = 1.0; // time step in seconds

        // Calculate the new position and velocity of the object
        let new_x = self.actual_position.x + self.actual_velocity.x * t;
        let new_y = self.actual_position.y + self.actual_velocity.y * t + 0.5 * g * t * t;
        let new_vx = self.actual_velocity.x;
        let new_vy = self.actual_velocity.y + g * t;

        // Update the ThrowObject with the new values
        self.actual_position = Object { x: new_x, y: new_y };
        self.actual_velocity = Object { x: new_vx, y: new_vy };
        self.time += t;

        // Check if the object has hit the ground
        if self.actual_position.y <= 0.0 {
            None
        } else {
            Some(self.clone())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
        println!("{:?}", obj.next());
    }
}
