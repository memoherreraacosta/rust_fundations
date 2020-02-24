fn main(){
    loop{
        println!("loopcito");
        break;
    }
    let mut x = 43;
    while x != 0 {
        println!("value x = {}", x);
        x = 0;
    }

    let a = (1,2,3,4,5,"fgd", 3.43);

    for element in a{
        println!("Value is {}", element)
    }
}
