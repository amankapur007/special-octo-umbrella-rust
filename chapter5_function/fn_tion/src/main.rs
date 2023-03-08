fn main() {
    println!("Hello, world!");
    another_function();
    parameterized_function(String::from("name"));

    println!("{}",plus_one(5));
}

fn another_function(){
    println!("Another function");
}

fn parameterized_function(name:String){
    println!("Name is {}", name);
}

fn plus_one(i:i32)->i32{
    i+1
}