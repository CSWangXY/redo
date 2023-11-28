use std::env;
mod database;
use database::Database;
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
    let mut db = Database::open(".rodorc");
    match command.as_str()
    {
        "add" => {
            if args.len() < 3
            {
                println!("Usage: rodo [add] [contents]");
                return;
            }
            //println!("Add");
            let contents = &args[2..].join(" ");
            let id = db.read_records().last().map(|r| r.id + 1).unwrap_or(1);
            db.add_record(&database::Record{
                id,
                content:contents.to_string(),
            });
        }
        "rm" => {
                if args.len() < 3 {
                    println!("Usage: rodo rm [id]");
                    return;
                }
                // 这里 id 是字符串，需要转换成 i32
                let id = args[2].parse::<i32>().unwrap();
                db.remove_record(id);
        }
        "ls" => {
            let records = db.read_records();
            if records.is_empty() {
                println!("No records. You can add one with `rodo add [content]`");
                return;
            }
            for record in records {
                println!("{}: {}", record.id, record.content);
            }
        }
        _ => {
            println!("Unknown Command:{}", command);
        }
    }
}
