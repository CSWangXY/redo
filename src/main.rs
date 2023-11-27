use std::env;
fn main() 
{
    let args : Vec<String> = env::args().collect();
    if args.len() < 2
    {
        println!("Usage: rodo [add|rm|ls] [args]");
        return;
    }
    //println!("{}", args[0]);
    let command = &args[1];
    match command.as_str()
    {
        "add" => {
            if args.len() < 3
            {
                println!("Usage: rodo [add] [contents]");
                return;
            }
            println!("Add");
        }
        "rm" => {
            if args.len() < 3
            {
                println!("Usage: rodo [rm] [id]");
                return;
            }
            println!("Remove");
        }
        "ls" => {
            println!("List");
        }
        _ => {
            println!("Unknown Command:{}", command);
        }
    }
}
