use std::cmp;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let points = load_points();
}

#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rect {
    topLeft: Point,
    bottomRight: Point,
}

impl Rect {
    fn new(x: i32, y: i32, x2: i32, y2: i32) -> Rect {
        return Rect {
            topLeft: Point::new(x, y),
            bottomRight: Point::new(x2, y2),
        };
    }

    fn enclosing(points: &Vec<Point>) -> Rect {
        return points.iter().fold(
            Rect::new(std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
            |rect, p| {
                let min_x = cmp::min(rect.topLeft.x, p.x);
                let min_y = cmp::min(rect.topLeft.y, p.y);
                let max_x = cmp::max(rect.bottomRight.x, p.x);
                let max_y = cmp::max(rect.bottomRight.y, p.y);
                return Rect {
                    topLeft: Point::new(min_x, min_y),
                    bottomRight: Point::new(max_x, max_y),
                };
            },
        );
    }
}

fn load_points() -> Vec<Point> {
    let f = File::open("input.txt").expect("could not find file");
    let r = BufReader::new(&f);
    return r
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let mut tokens = l.split(|c| c == ',').map(|t| t.trim());
            return Point {
                x: tokens.next().unwrap().parse().unwrap(),
                y: tokens.next().unwrap().parse().unwrap(),
            };
        })
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_enclosing() {
        assert_eq!(
            Rect::new(0, 0, 0, 0),
            Rect::enclosing(&vec![Point::new(0, 0)])
        );
        assert_eq!(
            Rect::new(0, 0, 2, 2),
            Rect::enclosing(&vec![Point::new(0, 0), Point::new(2, 2)])
        );
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect::enclosing(&vec![Point::new(3, 4), Point::new(1, 2)])
        );
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect::enclosing(&vec![Point::new(3, 2), Point::new(1, 4)])
        );
    }

    #[test]
    fn test_points() {
        unimplemented!();
    }
}
