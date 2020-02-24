fn main(){
    //println!("Hello");
    another_func(54);
    println!("foo sum: {}", foo(5,5));
}

fn another_func(x: i32){
    println!("foo ran x = {}", x);
}

fn foo(x:i32, y:i32) -> i32{
    x + y
}
