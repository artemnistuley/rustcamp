
#[derive(Debug, Copy, Clone, Default)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    
    fn new(point: Point) -> Self {
        Polyline { points: vec![point] }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point);
    }
    

    fn remove_point(&mut self, index: usize) -> Option<Point> {
        if index < self.points.len() && self.points.len() > 1 {
            Some(self.points.remove(index))
        } else {
            None
        }
    }

    fn get_point(&self, index: usize) -> Option<&Point> {
        self.points.get(index)
    }

    fn len(&self) -> usize {
        self.points.len()
    }
}

fn main() {
    // Part 1
    let point_default: Point = Default::default();
    // println!("{:?}", point_default);

    let point = Point { x: 1.1, y: 2.1 };
    let point_copy = point;
    // println!("{:?}, {:?}", point, point_copy);


    // Part 2
    let point1 = Point { x: 0., y: 0. };
    let point2 = Point { x: 3.0, y: 4.0 };
    let point3 = Point { x: 5.0, y: 7.0 };

    let mut polyline = Polyline::new(point1);
    println!("{:?}", polyline);

    polyline.add_point(point2);
    polyline.add_point(point3);
    println!("{:?}", polyline);

    let removed = polyline.remove_point(2);
    println!("removed: {:?}", removed);
    println!("{:?}", polyline);

    let len = polyline.len();
    println!("len: {}", len);

    let first_point = polyline.get_point(0);
    println!("{:?}", first_point);
}
