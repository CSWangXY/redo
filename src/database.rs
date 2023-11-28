use std::fs::{File, OpenOptions};
use std::io::{Write, BufRead, BufReader, Seek};
use std::fs;
pub struct Record{
    pub id: i32,
    pub content: String,
}

pub struct Database{
    pub file: File,
}

impl Database{
    pub fn open(filename: &str) -> Database{
        let file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(filename)
                .unwrap();
        Database {file}
    }
}



impl Database{
    pub fn add_record(&mut self, record: &Record){
        let line = format!("{},{}\n", record.id, record.content);
        writeln!(self.file, "{}", line).unwrap();
        println!("Item added: {}", record.content);
    }


}


pub fn parse_record_line(line: &str)-> Record{
    let fields: Vec<&str> = line.split(',').collect();
    if fields.len() == 1{
        return Record{
            id : 0,
            content: "".to_string(),
        };
    }
    let content = fields[1..].join(",");
    return Record {
        id: fields[0].parse::<i32>().unwrap(),
        content,
    }

}

impl Database {
    pub fn read_records(&mut self) -> Vec<Record> {
        let reader = BufReader::new(&self.file);
        reader
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.is_empty())
            .map(|line| parse_record_line(&line))
            .collect()
    }
}
impl Database{
    pub fn remove_record(&mut self, id: i32)
    {
        let reader = BufReader::new(&self.file);
        let mut lines = reader.lines().enumerate();
        let line = lines.find(|(_, line)| {
            let record = parse_record_line(line.as_ref().unwrap());
            record.id == id
        });
        // match 匹配判断该行是否存在
        match line {
            Some((i, _)) => {
                // 读取源文件内容
                let contents = fs::read_to_string(".rodorc").unwrap();
                // 过滤掉对应的行，这里使用的对应 api 可以查看 Rust 标准库
                let new_contents = contents
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, line)| line)
                    .collect::<Vec<_>>()
                    .join("\n");
                // 将新的内容写入到源文件中
                // 这里使用了 std::io::Seek，需要导入
                self.file.seek(std::io::SeekFrom::Start(0)).unwrap();
                self.file.write_all(new_contents.as_bytes()).unwrap();
                self.file.set_len(new_contents.len() as u64).unwrap();

                println!(" ❌ Item removed!\n");
            }
            None => {
                println!("No such record: {}", id);
            }
        }
    }
}
