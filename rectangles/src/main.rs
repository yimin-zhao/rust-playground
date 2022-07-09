fn main() {
    let width1 = 30;
    let height1 = 50;

    println!("The area of the rectangle is {} square pixels.",
        area1(width1, height1));

    let rect1 = (30, 50);
    println!("The area of the rectangle is {} square pixels.",
    area2(rect1));

    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} square pixels.",
    area3(&rect2));
    println!("The rect2 is {:#?}", rect2);
    dbg!(&rect2);

    println!("The area of the rectangle is {} square pixels.",
        rect2.area());

    let rect3 = Rectangle::square(25);
    if rect2.can_hold(&rect3) {
        println!("The rect {:?} can hold {:?}", rect2, rect3);
    } else {
        println!("The rect {:?} cannot hold {:?}", rect2, rect3);
    }
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area3(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}