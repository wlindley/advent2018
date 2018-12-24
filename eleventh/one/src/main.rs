fn main() {
    let serial = 5093;
    let grid = Grid::new(serial);
    let (x, y) = grid.highest();
    println!("Highest cell: {},{}", x, y);
}

fn power_level(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let power = rack_id * y;
    let power = power + serial;
    let power = power * rack_id;
    let hundreds = (power / 100) % 10;
    hundreds - 5
}

const CELL_SIZE: i32 = 3;

struct Grid {
    serial: i32,
}

impl Grid {
    fn new(serial: i32) -> Self {
        Grid{serial}
    }

    fn get(&self, x: i32, y: i32) -> i32 {
        power_level(x, y, self.serial)
    }

    fn cell(&self, x: i32, y: i32) -> i32 {
        let mut power = 0;
        for x in x..x+CELL_SIZE {
            for y in y..y+CELL_SIZE {
                power += self.get(x, y);
            }
        }
        power
    }

    fn highest(&self) -> (i32, i32) {
        let mut highest = 0;
        let mut high_x = std::i32::MIN;
        let mut high_y = std::i32::MIN;
        for x in 1..301 - CELL_SIZE {
            for y in 1..301 - CELL_SIZE {
                let power = self.cell(x, y);
                if power > highest {
                    highest = power;
                    high_x = x;
                    high_y = y;
                }
            }
        }
        (high_x, high_y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_level() {
        assert_eq!(4, power_level(3, 5, 8));
        assert_eq!(-5, power_level(122, 79, 57));
        assert_eq!(0, power_level(217, 196, 39));
        assert_eq!(4, power_level(101, 153, 71));
    }

    #[test]
    fn test_grid_get() {
        let grid = Grid::new(18);
        assert_eq!(4, grid.get(33, 45));
        assert_eq!(4, grid.get(34, 45));
        assert_eq!(4, grid.get(35, 45));
        assert_eq!(3, grid.get(33, 46));
        assert_eq!(3, grid.get(34, 46));
        assert_eq!(4, grid.get(35, 46));
        assert_eq!(1, grid.get(33, 47));
        assert_eq!(2, grid.get(34, 47));
        assert_eq!(4, grid.get(35, 47));
    }

    #[test]
    fn test_grid_cell() {
        let grid = Grid::new(18);
        assert_eq!(29, grid.cell(33, 45));
        let grid = Grid::new(42);
        assert_eq!(30, grid.cell(21, 61));
    }

    #[test]
    fn test_grid_highest() {
        let grid = Grid::new(18);
        assert_eq!((33, 45), grid.highest());
        let grid = Grid::new(42);
        assert_eq!((21, 61), grid.highest());
    }
}
