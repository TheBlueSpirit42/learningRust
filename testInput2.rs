use std::io;
fn main()
{
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    /*match io::stdin().read_line(&mut input) 
    {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {}", error),
    }*/

    println!("{}", input);
}