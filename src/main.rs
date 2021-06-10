use std::io;
 
fn main()  -> io::Result<()>{
     
    let mut input = String::new();
    let mut tinput;
    println!("Welcome");
    let end = String::from("exit");

    loop {
    println!("ready for work hard:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    tinput = input.replace(" ","");
    if tinput.trim() == end{
        println!("bye bye!");
        break;
    }
    println!("{}",tinput);

    let parts =  input.split("=");
    for part in parts {
        
        println!("{}", part);
    }
    input = String::new();
    }
    Ok(())
}
