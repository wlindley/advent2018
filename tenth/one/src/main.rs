use std::cmp;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let points: Vec<Point> = read_input().into_iter().map(parse_line).collect();
    let mut simulation = Simulation::with_points(points);
    println!("{}\n", simulation);
    for _ in 0..1000000 {
        simulation.update();
        println!("{}\n", simulation);
    }
}

fn read_input() -> Vec<String> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r.lines().map(|l| l.unwrap()).collect();
}

fn parse_line(mut line: String) -> Point {
    line = line.replace("position=", "").replace("velocity=", "");
    line = line.replace("<", "").replace(">", "").replace(",", "");
    let mut iter = line.trim().split_whitespace().map(|t| t.parse().unwrap());
    return Point::new(
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
        iter.next().unwrap(),
    );
}

type Scalar = i64;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord {
    pub x: Scalar,
    pub y: Scalar,
}

impl Coord {
    pub fn new(x: Scalar, y: Scalar) -> Self {
        Coord { x, y }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    pub pos: Coord,
    pub vel: Coord,
}

impl Point {
    pub fn new(x: Scalar, y: Scalar, vel_x: Scalar, vel_y: Scalar) -> Self {
        Point {
            pos: Coord::new(x, y),
            vel: Coord::new(vel_x, vel_y),
        }
    }

    pub fn update(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
    }
}

struct Simulation {
    points: Vec<Point>
}

impl Simulation {
    pub fn with_points(points: Vec<Point>) -> Self {
        Simulation{points}
    }

    pub fn update(&mut self) {
        for point in &mut self.points {
            point.update();
        }
    }

    pub fn coords(&self) -> Vec<&Coord> {
        let mut result = Vec::with_capacity(self.points.len());
        for point in &self.points {
            result.push(&point.pos);
        }
        return result;
    }

    pub fn output_size(&self) -> usize {
        let point_coords = self.coords();
        let rect = Rect::enclosing(&point_coords);
        let width = rect.bottom_right.x - rect.top_left.x;
        let height = rect.bottom_right.y - rect.top_left.y;
        ((width + 1) * height) as usize
    }
}

impl fmt::Display for Simulation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let point_coords = self.coords();
        let rect = Rect::enclosing(&point_coords);
        let width = rect.bottom_right.x - rect.top_left.x;
        let height = rect.bottom_right.y - rect.top_left.y;
        let size = ((width + 1) * height) as usize;
        if size > 50000 {
            return write!(f, "");
        }

        let mut buffer = String::with_capacity(size);

        let mut prev_y = point_coords[0].y;
        for coord in rect.coords() {
            if coord.y != prev_y {
                buffer.push('\n');
                prev_y = coord.y;
            }
            if point_coords.contains(&&coord) {
                buffer.push('#');
            } else {
                buffer.push('.');
            }
        }
        write!(f, "{}", buffer)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rect {
    top_left: Coord,
    bottom_right: Coord,
}

impl Rect {
    fn new(x: Scalar, y: Scalar, x2: Scalar, y2: Scalar) -> Rect {
        return Rect {
            top_left: Coord::new(x, y),
            bottom_right: Coord::new(x2, y2),
        };
    }

    fn enclosing(coords: &Vec<&Coord>) -> Rect {
        return coords.iter().fold(
            Rect::new(std::i64::MAX, std::i64::MAX, std::i64::MIN, std::i64::MIN),
            |rect, c| {
                let min_x = cmp::min(rect.top_left.x, c.x);
                let min_y = cmp::min(rect.top_left.y, c.y);
                let max_x = cmp::max(rect.bottom_right.x, c.x);
                let max_y = cmp::max(rect.bottom_right.y, c.y);
                return Rect {
                    top_left: Coord::new(min_x, min_y),
                    bottom_right: Coord::new(max_x, max_y),
                };
            },
        );
    }

    fn coords(&self) -> CoordIterator {
        return CoordIterator {
            rect: self.clone(),
            cur: self.top_left.clone(),
        };
    }
}

struct CoordIterator {
    rect: Rect,
    cur: Coord,
}

impl Iterator for CoordIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.y > self.rect.bottom_right.y {
            return Option::None;
        }
        let coord = self.cur.clone();
        self.cur.x += 1;
        if self.cur.x > self.rect.bottom_right.x {
            self.cur.x = self.rect.top_left.x;
            self.cur.y += 1;
        }
        return Option::Some(coord);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        assert_eq!(
            Point::new(0, 0, 0, 0),
            parse_line(String::from("position=<     0,      0> velocity=< 0,  0>"))
        );
        assert_eq!(
            Point::new(0, 0, 1, 1),
            parse_line(String::from("position=<     0,      0> velocity=< 1,  1>"))
        );
        assert_eq!(
            Point::new(1, 1, 1, 1),
            parse_line(String::from("position=<     1,      1> velocity=< 1,  1>"))
        );
        assert_eq!(
            Point::new(-1, -1, -1, -1),
            parse_line(String::from("position=<    -1,     -1> velocity=<-1, -1>"))
        );
    }

    #[test]
    fn test_point_update() {
        let mut point = Point::new(0, 0, 1, 0);
        point.update();
        assert_eq!(Point::new(1, 0, 1, 0), point);
        point.update();
        assert_eq!(Point::new(2, 0, 1, 0), point);

        let mut point = Point::new(0, 0, 0, 1);
        point.update();
        assert_eq!(Point::new(0, 1, 0, 1), point);
        point.update();
        assert_eq!(Point::new(0, 2, 0, 1), point);
    }

    #[test]
    fn test_simulation_update() {
        let mut sim = Simulation::with_points(vec![Point::new(0, 0, 1, 0), Point::new(0, 0, 0, 1)]);
        sim.update();
        assert_eq!(Point::new(1, 0, 1, 0), sim.points[0]);
        assert_eq!(Point::new(0, 1, 0, 1), sim.points[1]);
    }

    #[test]
    fn test_simulation_display() {
        let sim = Simulation::with_points(vec![Point::new(0, 0, 0, 0), Point::new(1, 1, 0, 0)]);
        let expected = String::from("#.\n.#");
        assert_eq!(expected, sim.to_string());

        let sim = Simulation::with_points(vec![Point::new(1, 0, 0, 0), Point::new(0, 2, 0, 0), Point::new(2, 1, 0, 0)]);
        let expected = String::from(".#.\n..#\n#..");
        assert_eq!(expected, sim.to_string());

        let sim = Simulation::with_points(vec![Point::new(1, 0, 0, 0), Point::new(1, 2, 0, 0), Point::new(3, 1, 0, 0)]);
        let expected = String::from("#..\n..#\n#..");
        assert_eq!(expected, sim.to_string());
    }

    #[test]
    fn test_simulation_coords() {
        let sim = Simulation::with_points(vec![Point::new(0, 0, 0, 0), Point::new(1, 1, 0, 0)]);
        assert_eq!(vec![&Coord::new(0, 0), &Coord::new(1, 1)], sim.coords());
    }

    #[test]
    fn test_rect_enclosing() {
        assert_eq!(
            Rect::new(0, 0, 0, 0),
            Rect::enclosing(&vec![&Coord::new(0, 0)])
        );
        assert_eq!(
            Rect::new(0, 0, 2, 2),
            Rect::enclosing(&vec![&Coord::new(0, 0), &Coord::new(2, 2)])
        );
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect::enclosing(&vec![&Coord::new(3, 4), &Coord::new(1, 2)])
        );
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect::enclosing(&vec![&Coord::new(3, 2), &Coord::new(1, 4)])
        );
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect::enclosing(&vec![&Coord::new(3, 2), &Coord::new(2, 3), &Coord::new(1, 4)])
        );
    }

    #[test]
    fn test_rect_coords() {
        assert_eq!(
            vec![Coord::new(0, 0), Coord::new(1, 0)],
            Rect::new(0, 0, 1, 0).coords().collect::<Vec<Coord>>()
        );
        assert_eq!(
            vec![
                Coord::new(0, 0),
                Coord::new(1, 0),
                Coord::new(0, 1),
                Coord::new(1, 1)
            ],
            Rect::new(0, 0, 1, 1).coords().collect::<Vec<Coord>>()
        );
    }
}
