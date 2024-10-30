struct PointInteger {
    x: i32,
    y: i32,
}

impl PointInteger {
    fn x(&self) -> &i32 {
        &self.x
    }

    fn y(&self) -> &i32 {
        &self.y
    }
}

//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
struct PointFloat {
    x: f32,
    y: f32,
}

impl PointFloat {
    fn x(&self) -> &f32 {
        &self.x
    }

    fn y(&self) -> &f32 {
        &self.y
    }
}
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++

fn main() {
    let p = PointInteger { x: 5, y: 10 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());

    let p = PointFloat { x: 1.2, y: 5.3 };
    println!("p.x = {}", p.x());
    println!("p.y = {}", p.y());
}
