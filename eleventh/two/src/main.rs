use std::cmp;

fn main() {
    let serial = 5093;
    let mut grid = Grid::new(serial);
    let (x, y, size) = grid.highest();
    println!("Highest cell: {},{},{}", x, y, size);
}

fn power_level(x: i32, y: i32, serial: i32) -> i64 {
    let rack_id = x as i64 + 10i64;
    let power = rack_id * y as i64;
    let power = power + serial as i64;
    let power = power * rack_id;
    let hundreds = (power / 100i64) % 10i64;
    hundreds - 5i64
}

struct Grid {
    serial: i32,
    grid: Vec<Vec<Option<i64>>>,
}

impl Grid {
    fn new(serial: i32) -> Self {
        let mut grid = Vec::with_capacity(301);
        for _ in 0..301 {
            grid.push(vec![None; 301]);
        }
        Grid{serial, grid}
    }

    fn get(&mut self, x: i32, y: i32) -> i64 {
        let ix = x as usize;
        let iy = y as usize;
        match self.grid[ix][iy] {
            None => {
                let power = power_level(x, y, self.serial);
                self.grid[ix][iy] = Some(power);
                power
            },
            Some(power) => power,
        }
    }

    fn cell(&mut self, x: i32, y: i32, size: i32) -> i64 {
        let mut power = 0;
        for cur_x in 0..size {
            for cur_y in 0..size {
                power += self.get(x + cur_x, y + cur_y);
            }
        }
        power
    }

    fn highest(&mut self) -> (i32, i32, i32) {
        let mut highest = 0;
        let mut high_x = std::i32::MIN;
        let mut high_y = std::i32::MIN;
        let mut high_size = 0;
        for x in 1..301 {
            for y in 1..301 {
                let max = cmp::max(x, y);
                for size in 1..301-max {
                    let power = self.cell(x, y, size);
                    if power > highest {
                        highest = power;
                        high_x = x;
                        high_y = y;
                        high_size = size;
                    }
                }
            }
        }
        (high_x, high_y, high_size)
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
        assert_eq!(3, power_level(90, 269, 18));
    }

    #[test]
    fn test_grid_get() {
        let mut grid = Grid::new(18);
        assert_eq!(4, grid.get(33, 45));
        assert_eq!(4, grid.get(34, 45));
        assert_eq!(4, grid.get(35, 45));
        assert_eq!(3, grid.get(33, 46));
        assert_eq!(3, grid.get(34, 46));
        assert_eq!(4, grid.get(35, 46));
        assert_eq!(1, grid.get(33, 47));
        assert_eq!(2, grid.get(34, 47));
        assert_eq!(4, grid.get(35, 47));
        assert_eq!(4, grid.get(35, 47));
    }

    #[test]
    fn test_grid_cell() {
        let mut grid = Grid::new(18);
        assert_eq!(29, grid.cell(33, 45, 3));
        assert_eq!(113, grid.cell(90, 269, 16));
        let mut grid = Grid::new(42);
        assert_eq!(30, grid.cell(21, 61, 3));
        assert_eq!(119, grid.cell(232, 251, 12));
    }

    #[test]
    fn test_grid_highest() {
        let mut grid = Grid::new(18);
        assert_eq!((90, 269, 16), grid.highest());
        let mut grid = Grid::new(42);
        assert_eq!((232, 251, 12), grid.highest());
    }
}
