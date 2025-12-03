use std::fs::File;
use std::io::Read;
use std::path::Path;

fn read_file() -> File {
    let path = Path::new("./src/puzzle_input2.txt");
    File::open(path).expect("Could not open file")
}

fn splice_till_comma(s: &String) -> Vec<(u64, u64)> {
    let mut vector: Vec<(u64, u64)> = vec![];
    let mut count: usize= 0;
    for (i, &letter) in s.as_bytes().iter().enumerate() {
        if letter == b',' || letter == b'\n' {
            vector.push(get_range(&s[count..i]));
            count = i+1;
        }   
    }

    if count < s.len() {
        vector.push(get_range(&s[count..]));
    }
    vector
}

fn get_range(s: &str) -> (u64, u64) {
    for (i, &letter) in s.as_bytes().iter().enumerate() {
        if letter == b'-' {
            return (s[0..i].parse::<u64>().unwrap(), s[i+1..].parse::<u64>().unwrap());
        }
    }

    panic!("No '-' found in string: {}", s);
}
pub fn run() -> u64 {
    let mut content: String = String::new();
    let mut data: File = read_file();
    data.read_to_string(&mut content);
    content = content.trim().to_string();
    let y = splice_till_comma(&content);
    println!("{:?}", y);
    let mut total = 0;
    for index in y{
        for ind in index.0..=index.1{
            let ind_str = ind.to_string();
            let half = ind_str.len()/2;
            if ind_str[..half] == ind_str[half..]{
                total += ind;
            }
        }
    }
    total
}
