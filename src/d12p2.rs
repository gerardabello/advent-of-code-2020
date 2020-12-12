use crate::d12p1::{parse_movement, Direction, Movement, MovementKind};

#[derive(Debug)]
struct Waypoint {
    pos_x: i64,
    pos_y: i64,
}

#[derive(Debug)]
struct Ship {
    pos_x: i64,
    pos_y: i64,
    waypoint: Waypoint,
}

impl Waypoint {
    fn turn_left(&mut self) {
        let px = self.pos_x;
        let py = self.pos_y;
        self.pos_x = -py;
        self.pos_y = px;
    }

    fn turn_rigth(&mut self) {
        let px = self.pos_x;
        let py = self.pos_y;
        self.pos_x = py;
        self.pos_y = -px;
    }

    fn move_direction(&mut self, direction: Direction, ammount: u32) {
        match direction {
            Direction::East => {
                self.pos_x += ammount as i64;
            }
            Direction::West => {
                self.pos_x -= ammount as i64;
            }
            Direction::North => {
                self.pos_y += ammount as i64;
            }
            Direction::South => {
                self.pos_y -= ammount as i64;
            }
        };
    }
}

impl Ship {
    fn move_to_waypoint(&mut self, ammount: u32) {
        self.pos_x += self.waypoint.pos_x * ammount as i64;
        self.pos_y += self.waypoint.pos_y * ammount as i64;
    }

    fn apply_movement(&mut self, movement: &Movement) {
        let ammount = movement.ammount;
        match movement.kind {
            MovementKind::East => self.waypoint.move_direction(Direction::East, ammount),
            MovementKind::West => self.waypoint.move_direction(Direction::West, ammount),
            MovementKind::North => self.waypoint.move_direction(Direction::North, ammount),
            MovementKind::South => self.waypoint.move_direction(Direction::South, ammount),

            MovementKind::TurnLeft => {
                assert!(ammount % 90 == 0);
                let turns = ammount / 90;
                for _ in 0..turns {
                    self.waypoint.turn_left();
                }
            }

            MovementKind::TurnRight => {
                assert!(ammount % 90 == 0);
                let turns = ammount / 90;
                for _ in 0..turns {
                    self.waypoint.turn_rigth();
                }
            }

            MovementKind::Forward => self.move_to_waypoint(ammount),
        };
    }
}

pub fn solve(input: &str) -> String {
    let movements: Vec<Movement> = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| parse_movement(l).unwrap())
        .map(|(_, instruction)| instruction)
        .collect();

    let mut ship = Ship {
        pos_x: 0,
        pos_y: 0,
        waypoint: Waypoint {
            pos_x: 10,
            pos_y: 1,
        },
    };

    for mov in movements {
        ship.apply_movement(&mov);
    }

    let distance = i64::abs(ship.pos_x) + i64::abs(ship.pos_y);

    distance.to_string()
}
