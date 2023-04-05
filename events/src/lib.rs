use chrono::Duration;
use colored::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Position {
    Top,
    Bottom,
    Center,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Notification {
    pub size: u32,
    pub color: (u8, u8, u8),
    pub position: Position,
    pub content: String,
}

impl std::fmt::Display for Notification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (r, g, b) = self.color;
        let colored_content = match (r, g, b) {
            (255, 2, 22) => self.content.red(),
            (200, 200, 3) => self.content.yellow(),
            (0, 255, 0) => self.content.green(),
            _ => self.content.normal(),
        };
        let position_str = match self.position {
            Position::Top => "top",
            Position::Bottom => "bottom",
            Position::Center => "center",
        };
        write!(
            f,
            "{}",
            format!(
                "{} notification (size {}, position {}): {}",
                match (r, g, b) {
                    (255, 2, 22) => "Error".red(),
                    (200, 200, 3) => "Warning".yellow(),
                    (0, 255, 0) => "Success".green(),
                    _ => "Info".normal(),
                },
                self.size,
                position_str,
                colored_content
            )
        )
    }
}

#[derive(Debug)]
pub enum Event<'a> {
    Remainder(&'a str),
    Registration(Duration),
    Appointment(&'a str),
    Holiday,
}

impl<'a> Event<'a> {
    pub fn notify(&self) -> Notification {
        match self {
            Self::Remainder(text) => Notification {
                size: 50,
                color: (50, 50, 50),
                position: Position::Bottom,
                content: text.to_string(),
            },
            Self::Registration(duration) => Notification {
                size: 30,
                color: (255, 2, 22),
                position: Position::Top,
                content: format!(
                    "You have {} left before the registration ends",
                    duration_to_hms_string(duration)
                ),
            },
            Self::Appointment(text) => Notification {
                size: 100,
                color: (200, 200, 3),
                position: Position::Center,
                content: text.to_string(),
            },
            Self::Holiday => Notification {
                size: 25,
                color: (0, 255, 0),
                position: Position::Top,
                content: "Enjoy your holiday".to_string(),
            },
        }
    }
}

fn duration_to_hms_string(duration: &Duration) -> String {
    let hours = duration.num_hours();
    let minutes = duration.num_minutes() % 60;
    let seconds = duration.num_seconds() % 60;
    format!("{:02}H:{:02}M:{:02}S", hours, minutes, seconds)
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let remainder = Remainder("Go to the doctor");
//         println!("{}", remainder.notify());
//         let registration = Registration(Duration::seconds(49094));
//         println!("{}", registration.notify());
//         let appointment = Appointment("Go to the doctor");
//         println!("{}", appointment.notify());
//         let holiday = Holiday;
//         println!("{}", holiday.notify());
//     }
// }
