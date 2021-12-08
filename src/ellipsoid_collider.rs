struct Ellipse {
	x: i32,
	y: i32,
	r: u32,
}

impl Ellipse {
	fn new(x: i32, y: i32, r: u32) -> Ellipse{
		Ellipse {
			x,
			y,
			r,
		}
	}

	// Getters
	fn x(&self) -> i32 { self.x }
	fn y(&self) -> i32 { self.y }
	fn r(&self) -> u32 { self.r }

	// Setters
	fn set_x(&mut self, x: i32) { self.x = x; }
	fn set_y(&mut self, y: i32) { self.y = y; }

	fn left(&self) -> i32 { self.x - (self.r as i32) }
	fn right(&self) -> i32 { self.x + (self.r as i32) }
	fn top(&self) -> i32 { self.y - (self.r as i32) }
	fn bottom(&self) -> i32 { self.y + (self.r as i32) }

	fn get_points(&self) -> Vec<Point> {
		let rad = self.r() as i32;
		(-rad..rad)
			.flat_map(|i| {
				(-rad..rad).map(move |j| { (i, j) })
			})
			.filter_map(|t| {
				if (t.0.pow(2) + t.1.pow(2)) < rad.pow(2) {
					Some(Point::new(self.x + t.0, self.y + t.1))
				}
				else {
					None
				}
			})
			.collect()
	}
}

fn check_collision(a: &Ellipse, b: &Ellipse) -> bool {
	let radsum = (a.r() + b.r()) as i32;
	let distsq = (a.x() - b.x()).pow(2) + (a.y() - b.y()).pow(2);
	distsq < radsum.pow(2)
}

fn resist(vel: i32, deltav: i32) -> i32 {
	if deltav == 0 {
		if vel > 0 {
			-1
		}
		else if vel < 0 {
			1
		}
		else {
			deltav
		}
	}
	else {
		deltav
	}
}