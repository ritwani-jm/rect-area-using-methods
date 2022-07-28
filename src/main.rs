struct Rect {
    width: u32,
    height: u32
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }
}

fn main() {
    let rect = Rect {
        width: 12,
        height: 10
    };

    println!("Area of rect is {}", rect.area());

    println!("[Setting up width getter] Width is: {}", rect.width());
    println!("[Setting up height getter] Height is: {}", rect.height());
}
