
fn main() {
    let a:i32 = -100;
    let b:u32 = 100;
    let f = 2.0;//f64 by default
    let d:f32 = 1.0;

    //add
    let sum = 5+10;
    println!("{sum}");
    //difference
    let diff = 95.5 - 4.3;
    println!("{diff}");
    //product
    let product = 4*30;
    println!("{product}");
    //division
    let div = 56.7/32.2;
    println!("{div}");
    //truncated
    let tru = -5/3; //Results -1
    println!("{tru}");
    //remainder
    let rem = 43%5;
    println!("{rem}");

    let t = true;
    let fa:bool =  false;

    let ch = 'z';
    let cha:char = 'Z';

    let tup = (1,1.2, false);

    let tup1:(i32, f64, bool) = (1,3.2,true);

    let (x,y,z) = tup;
    println!("x={x} y={y} z={z}");

    let one = tup1.0;
    let three_point_two = tup1.1;
    let tru = tup1.2;

    //Array type

    let arr = [1,2,3,4,5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    let arr1 = [3;5];
    println!("arr1 : {:?}", arr1);

    println!("Accessing element 2 of arr : {}", arr[1]);

}
