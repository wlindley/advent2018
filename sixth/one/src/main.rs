use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let points = load_points();
    let areas = calculate_areas(points);
    let mut point = Point::new(0, 0);
    let mut largest = std::i32::MIN;
    for (p, area) in areas {
        if area > largest {
            point = p;
            largest = area;
        }
    }
    println!("Point {:?} has largest area: {}", point, largest);
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        return Point { x, y };
    }

    fn distance(&self, other: &Point) -> i32 {
        return (other.x - self.x).abs() + (other.y - self.y).abs();
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Rect {
    top_left: Point,
    bottom_right: Point,
}

impl Rect {
    fn new(x: i32, y: i32, x2: i32, y2: i32) -> Rect {
        return Rect {
            top_left: Point::new(x, y),
            bottom_right: Point::new(x2, y2),
        };
    }

    fn enclosing(points: &Vec<Point>) -> Rect {
        return points.iter().fold(
            Rect::new(std::i32::MAX, std::i32::MAX, std::i32::MIN, std::i32::MIN),
            |rect, p| {
                let min_x = cmp::min(rect.top_left.x, p.x);
                let min_y = cmp::min(rect.top_left.y, p.y);
                let max_x = cmp::max(rect.bottom_right.x, p.x);
                let max_y = cmp::max(rect.bottom_right.y, p.y);
                return Rect {
                    top_left: Point::new(min_x, min_y),
                    bottom_right: Point::new(max_x, max_y),
                };
            },
        );
    }

    fn points(&self) -> PointIterator {
        return PointIterator {
            rect: self.clone(),
            cur: self.top_left.clone(),
        };
    }

    fn on_border(&self, point: &Point) -> bool {
        return point.x == self.top_left.x
            || point.x == self.bottom_right.x
            || point.y == self.top_left.y
            || point.y == self.bottom_right.y;
    }
}

struct PointIterator {
    rect: Rect,
    cur: Point,
}

impl Iterator for PointIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.cur.y > self.rect.bottom_right.y {
            return Option::None;
        }
        let point = self.cur.clone();
        self.cur.x += 1;
        if self.cur.x > self.rect.bottom_right.x {
            self.cur.x = self.rect.top_left.x;
            self.cur.y += 1;
        }
        return Option::Some(point);
    }
}

fn calculate_areas(points: Vec<Point>) -> HashMap<Point, i32> {
    let rect = Rect::enclosing(&points);
    let mut results: HashMap<Point, i32> = HashMap::new();
    for point in rect.points() {
        match find_closest(&point, &points) {
            Option::None => continue,
            Option::Some(closest) => {
                if rect.on_border(&point) {
                    *results.entry(closest.clone()).or_insert(0) = std::i32::MIN;
                } else {
                    *results.entry(closest.clone()).or_insert(0) += 1;
                }
            },
        }
    }
    return results;
}

fn find_closest<'a>(probe: &Point, points: &'a Vec<Point>) -> Option<&'a Point> {
    let mut distances = HashMap::new();
    for p in points {
        let dist = probe.distance(p);
        if distances.contains_key(&dist) {
            distances.insert(dist, Option::None);
        } else {
            distances.insert(dist, Option::Some(p));
        }
    }

    let mut closest = std::i32::MAX;
    for (&dist, _) in &distances {
        closest = cmp::min(dist, closest);
    }
    return *distances.entry(closest).or_default();
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
        assert_eq!(
            Rect::new(1, 2, 3, 4),
            Rect::enclosing(&vec![Point::new(3, 2), Point::new(2, 3), Point::new(1, 4)])
        );
    }

    #[test]
    fn test_points() {
        assert_eq!(
            vec![Point::new(0, 0), Point::new(1, 0)],
            Rect::new(0, 0, 1, 0).points().collect::<Vec<Point>>()
        );
        assert_eq!(
            vec![
                Point::new(0, 0),
                Point::new(1, 0),
                Point::new(0, 1),
                Point::new(1, 1)
            ],
            Rect::new(0, 0, 1, 1).points().collect::<Vec<Point>>()
        );
    }

    #[test]
    fn test_on_border() {
        let rect = Rect::new(0, 0, 5, 5);
        assert_eq!(true, rect.on_border(&Point::new(0, 0)));
        assert_eq!(true, rect.on_border(&Point::new(5, 5)));
        assert_eq!(true, rect.on_border(&Point::new(2, 0)));
        assert_eq!(true, rect.on_border(&Point::new(0, 3)));
        assert_eq!(false, rect.on_border(&Point::new(1, 1)));
        assert_eq!(false, rect.on_border(&Point::new(2, 3)));
    }

    #[test]
    fn test_distance() {
        assert_eq!(1, Point::new(0, 0).distance(&Point::new(1, 0)));
        assert_eq!(1, Point::new(0, 0).distance(&Point::new(0, 1)));
        assert_eq!(2, Point::new(0, 0).distance(&Point::new(1, 1)));
        assert_eq!(7, Point::new(1, 2).distance(&Point::new(5, -1)));
    }

    #[test]
    fn test_find_closest() {
        assert_eq!(Point::new(0, 0), *find_closest(&Point::new(0, 0), &vec![Point::new(0, 0), Point::new(2, 2)]).unwrap());
        assert_eq!(Point::new(2, 2), *find_closest(&Point::new(2, 1), &vec![Point::new(0, 0), Point::new(2, 2)]).unwrap());
        assert_eq!(Option::None, find_closest(&Point::new(1, 1), &vec![Point::new(0, 0), Point::new(2, 2)]));
    }

    #[test]
    fn test_calculate_areas() {
        let areas = calculate_areas(vec![
            Point::new(1, 1), //inf
            Point::new(1, 6), //inf
            Point::new(8, 3), //inf
            Point::new(3, 4), // 9
            Point::new(5, 5), //17
            Point::new(8, 9), //inf
        ]);
        assert_eq!(9, areas[&Point::new(3, 4)]);
        assert_eq!(17, areas[&Point::new(5, 5)]);

        let areas = calculate_areas(vec![
            Point::new(0, 0), //inf
            Point::new(0, 8), //inf
            Point::new(8, 0), //inf
            Point::new(8, 8), //inf
            Point::new(4, 4), //23
        ]);
        assert_eq!(25, areas[&Point::new(4, 4)]);
    }
}
