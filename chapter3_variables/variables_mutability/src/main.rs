fn main() {
    let x = 5;
    println!("The value of x is {x}");
    
    //x = 6;//Can not assign twice to immutable variable
    //println!("The value of x is {x}");
    println!("------------------------------");
    let mut y = 5;
    println!("The value of y is {y}");
    y = 6;
    println!("The value of y is {y}");
  
    println!("------------------------------");
    //contant , contant can not be mut
    const  THREE_HOURS_INSECONDS:u32 = 60*60*3;
    println!("THREE HOURS IN SECONDS : {THREE_HOURS_INSECONDS}");
    println!("------------------------------");

    //Shadowing

    let z = 5;
    println!("The value of z is {z}");

    let z = z + 1;
    println!("The value of z is {z}");

    let spaces = "      ";
    let spaces = spaces.len();
    println!("Spaces : {spaces}");

    //mut variables can not be shadowed
    let mut spaces1 = "      ";
    //spaces1 = spaces1.len();
    //println!("Spaces : {spaces1}");

}
