#[derive(Debug, Clone, Copy)]
pub struct Circle {
	center: Point,
	radius: f64,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

// Point
impl Point {
    fn distance(self, other: &Point) -> f64 {
        let width = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
    
        let height = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };

        return f64::sqrt(width*width + height*height)
    }
}

// Circle
impl Circle {
    pub fn new(x: f64, y: f64, radius: f64) -> Circle {
        return Circle {
            center: Point {
                x: x, 
                y: y,
            },
            radius: radius,
        }
    }
}

impl Circle {
    fn diameter(self) -> f64 {
        return self.radius+self.radius
    }
}

impl Circle {
    fn area(self) -> f64 {
        return std::f64::consts::PI*self.radius*self.radius
    }
}

impl Circle {
    fn intersect(self, other: &Circle) -> bool {
        let width = if self.center.x > other.center.x {
            self.center.x - other.center.x
        } else {
            other.center.x - self.center.x
        };
        
        let height = if self.center.y > other.center.y {
            self.center.y - other.center.y
        } else {
            other.center.y - self.center.y
        };
        
        if self.radius+other.radius >= f64::sqrt(width*width + height*height) {
            return true
        }
        return false
    }
}

fn main() {
	let circle = Circle::new(500.0, 500.0, 150.0);
	let circle1 = Circle {
		center: Point { x: 80.0, y: 115.0 },
		radius: 30.0,
	};
	let point_a = Point { x: 1.0, y: 1.0 };
	let point_b = Point { x: 0.0, y: 0.0 };
	println!("circle = {:?} area = {}", circle, circle.area());
	println!("circle = {:?} diameter = {}", circle, circle.diameter());
	println!("circle1 = {:?} diameter = {}", circle1, circle1.diameter());
	println!(
		"circle and circle1 intersect = {}",
		circle.intersect(&circle1)
	);

	println!(
		"distance between {:?} and {:?} is {}",
		point_a,
		point_b,
		point_a.distance(&point_b)
	);

}

// circle = Circle { center: Point { x: 500.0, y: 500.0 }, radius: 150.0 } area = 70685.83470577035
// circle = Circle { center: Point { x: 500.0, y: 500.0 }, radius: 150.0 } diameter = 300
// circle1 = Circle { center: Point { x: 80.0, y: 115.0 }, radius: 30.0 } diameter = 60
// circle and circle1 intersect = false
// distance between Point { x: 1.0, y: 1.0 } and Point { x: 0.0, y: 0.0 } is 1.4142135623730951