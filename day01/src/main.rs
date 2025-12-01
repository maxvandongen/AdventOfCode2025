#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    direction: Direction,
    amount: usize,
}

#[derive(Debug)]
struct Dial(usize);

impl Dial {
    fn new(start: usize) -> Self {
        Dial(start)
    }

    fn new_default() -> Self {
        Self::new(50)
    }

    fn rotate(&mut self, rotation: &Rotation) -> usize {
        let rotation_amount = rotation.amount % 100;
        let mut count_zeros = rotation.amount / 100;
        self.0 = match rotation.direction {
            Direction::Left => match self.0.checked_sub(rotation_amount) {
                Some(new_value) => new_value,
                None => {
                    let val = 100 - (rotation_amount - self.0);
                    // Only count a zero if we didn't start at zero or land on zero
                    if val != 0 && self.0 != 0 {
                        count_zeros = count_zeros + 1;
                    }
                    val
                },
            },
            Direction::Right => match self.0 + rotation_amount {
                new_value if new_value < 100 => new_value,
                new_value => {
                    let val = new_value - 100;
                    // Only count a zero if we don't land on zero
                    if val != 0 {
                        count_zeros = count_zeros + 1;
                    }
                    val
                },
            },
        };
        return count_zeros + self.is_at(0) as usize;
    }

    fn is_at(&self, position: usize) -> bool {
        self.0 == position
    }
}

fn main() {
    let file_name = "input.txt";
    let content = std::fs::read_to_string(file_name).expect("Can open file");
    let mut part_1: usize = 0;
    let mut part_2: usize = 0;
    let mut dial = Dial::new_default();
    let rotations: Vec<Rotation> = content
        .lines()
        .map(|line| {
            let chars = line.chars().collect::<Vec<char>>();
            let (left, right) = (chars[0], chars[1..].iter().collect::<String>());


            let direction = match left {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let amount: usize = right.parse().expect("Right is number");

            Rotation { direction, amount }
        })
        .collect();

    for rotation in &rotations {
        part_2 += dial.rotate(rotation);
        if dial.is_at(0) {
            part_1 += 1;
        }
    }

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotates_from_description() {
        let mut dial = Dial::new(11);
        let rotation1 = Rotation {
            direction: Direction::Right,
            amount: 8,
        };
        dial.rotate(&rotation1);
        assert_eq!(dial.0, 19);

        let mut dial = Dial::new(99);
        let rotation2 = Rotation {
            direction: Direction::Right,
            amount: 1,
        };
        dial.rotate(&rotation2);
        assert_eq!(dial.0, 0);

        let mut dial = Dial::new(5);

        let rotation3 = Rotation {
            direction: Direction::Left,
            amount: 10,
        };
        dial.rotate(&rotation3);
        assert_eq!(dial.0, 95);
    }

    #[test]
    fn test_rotate_left_within_bounds() {
        let mut dial = Dial::new_default();
        let rotation = Rotation {
            direction: Direction::Left,
            amount: 20,
        };
        dial.rotate(&rotation);
        assert_eq!(dial.0, 30);
    }
    #[test]
    fn test_rotate_left_wrap_around() {
        let mut dial = Dial::new_default();
        let rotation = Rotation {
            direction: Direction::Left,
            amount: 60,
        };
        dial.rotate(&rotation);
        assert_eq!(dial.0, 90);
    }
    #[test]
    fn test_rotate_right_within_bounds() {
        let mut dial = Dial::new_default();
        let rotation = Rotation {
            direction: Direction::Right,
            amount: 30,
        };
        dial.rotate(&rotation);
        assert_eq!(dial.0, 80);
    }
    #[test]
    fn test_rotate_right_wrap_around() {
        let mut dial = Dial::new_default();
        let rotation = Rotation {
            direction: Direction::Right,
            amount: 60,
        };
        dial.rotate(&rotation);
        assert_eq!(dial.0, 10);
    }

    #[test]
    fn test_rotation_count_right() {
        let mut dial = Dial::new_default();
        let rotation = Rotation {
            direction: Direction::Right,
            amount: 250,
        };
        let count = dial.rotate(&rotation);
        assert_eq!(count, 3);
    }

    #[test]
    fn test_rotation_count_right_1000() {
        let mut dial = Dial::new_default();
        let rotation = Rotation {
            direction: Direction::Right,
            amount: 1000,
        };
        let count = dial.rotate(&rotation);
        assert_eq!(count, 10);
    }

        #[test]
    fn test_rotation_count_right_1000_from_zero() {
        let mut dial = Dial::new(0);
        let rotation = Rotation {
            direction: Direction::Right,
            amount: 1000,
        };
        let count = dial.rotate(&rotation);
        assert_eq!(count, 10);
    }

    #[test]
    fn test_rotation_count_left() {
        let mut dial = Dial::new_default();
        let rotation = Rotation {
            direction: Direction::Left,
            amount: 250,
        };
        let count = dial.rotate(&rotation);
        assert_eq!(count, 3);
    }
}
