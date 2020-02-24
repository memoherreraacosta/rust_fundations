struct Rectangle {
     width: u32,
     height: u32,   
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main() {

    
    let width = 30;
    let height = 50;

    let rect1 = Rectangle {width: 10, height: 20};

    println!("Area is {}", rect1.area());

    //println!("Area is {}", area(width, height));
}


fn area(rectangle: &Rectangle)-> u32{
    rectangle.width * rectangle.height
}
