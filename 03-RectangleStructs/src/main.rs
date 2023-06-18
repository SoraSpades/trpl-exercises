#[derive(Debug)]
struct Rectangle {
    width:  u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn fits_inside(&self, other: &Self) -> bool {
        (other.width >= self.width) && (other.height >= self.height)
    }

    fn square(side: u32) -> Self {
        Rectangle {width: side, height: side}
    }
}

fn main() {
    let r = Rectangle::square(12);
    let s = Rectangle {
        height: 30,
        width: 13
    };
    println!("{}, {}", r.area(), r.fits_inside(&s));
}
