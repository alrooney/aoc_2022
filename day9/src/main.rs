use std::collections::HashSet;
use std::fs;

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct Rope {
    segments: Vec<Point>,
    path: HashSet<Point>
}

impl Rope {
    fn new(start: Point, num_segments: usize) -> Rope {
        let mut path = HashSet::new();
        path.insert(start.clone());
        let mut segments: Vec<Point> = vec![];
        (0..num_segments).for_each(|_| segments.push(start.clone()));
        Rope {segments, path}
    }

    fn len(&self) -> usize {
        self.segments.len()
    }

    fn move_it(&mut self, direction: &str, distance: i32) {
        let len = self.len();
        // move head
        match direction {
            "U" => self.segments[0].y += 1,
            "R" => self.segments[0].x += 1,
            "D" => self.segments[0].y -= 1,
            "L" => self.segments[0].x -= 1,
            _ => panic!("oops!")
        }
        // println!("head: {:?}", self.segments[0]);
        // move tail to follow head
        for i in 1..len {
            let delta_y = self.segments[i-1].y - self.segments[i].y;
            let mut delta_x = self.segments[i-1].x - self.segments[i].x;
            // println!("move segment {} from: {:?}", i, &self.segments[i]);
            if delta_y > 1 {
                // println!("moving up");
                self.segments[i].y += 1;
                if delta_x > 0 {
                    self.segments[i].x += 1;
                    delta_x -= 1;
                }
                if delta_x < 0 {
                    self.segments[i].x -= 1;
                    delta_x += 1;
                }
            }
            if delta_y < -1 {
                // println!("moving down");
                self.segments[i].y -= 1;
                if delta_x > 0 {
                    self.segments[i].x += 1;
                    delta_x -= 1;
                }
                if delta_x < 0 {
                    self.segments[i].x -= 1;
                    delta_x += 1;
                }
            }
            if delta_x > 1 {
                // println!("moving right");
                self.segments[i].x += 1;
                if delta_y > 0 {
                    self.segments[i].y += 1;
                }
                if delta_y < 0 {
                    self.segments[i].y -= 1;
                }
            }
            if delta_x < -1 {
                // println!("moving left");
                self.segments[i].x -= 1;
                if delta_y > 0 {
                    self.segments[i].y += 1;
                }
                if delta_y < 0 {
                    self.segments[i].y -= 1;
                }
            }
            // println!("to: {:?}", &self.segments[i]);
        }
        // println!("tail: {:?}", self.segments[len -1]);
        self.path.insert(self.segments[len - 1].clone());
        if distance > 1 {
            self.move_it(direction, distance - 1);
        }

    }
}

fn main() {
    // let file_path = "input_sample.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("could not open file");

    // start at 0,0
    let mut rope = Rope::new(Point {x: 0, y: 0}, 2);
    let mut rope2 = Rope::new(Point {x: 0, y: 0}, 10);
    for line in contents.lines() {
        let vector: Vec<&str> = line.split(" ").collect();
        let direction = vector[0];
        let distance = vector[1].parse::<i32>().unwrap();
        // println!("rope.move_it({}, {})", direction, distance);
        // println!("move rope");
        rope.move_it(direction, distance);
        // println!("move rope2");
        rope2.move_it(direction, distance);
    }
    // dbg!(&rope.path);
    dbg!(rope.path.len());

    // part2
    // dbg!(&rope2.path);
    dbg!(rope2.path.len());
}
