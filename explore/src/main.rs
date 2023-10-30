fn main() {
    println!("Hello, world!");
}

fn func_example(x:u8, y:u8) -> u8{
    return(x + y); 
}



fn string_slice(){
    let the_str: &str = "Hello There"; 
    let mine_str: String = String::from("Hello There"); 

    let other_str = String::from("here is an example string"); 
    let other_derv_str: &str = &other_str[11..18]; 

   

}


