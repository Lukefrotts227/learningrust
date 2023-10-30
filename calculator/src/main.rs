fn main() {
    let mut args: Args = args(); 


    let first: String = args.nth(1).unwrap(); 

    let operator : String = args.nth(0).unwrap(); 

    let second: String = args.nth(0).unwrap(); 
    
    println!("{} {} {}", first, operator, second); 
    
}

