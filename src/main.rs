extern crate glob;
extern crate scoped_threadpool;
use glob::glob;
use std::env;
use std::path::Path;
use std::result::Result;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use scoped_threadpool::Pool;
fn main() {
    let args: Vec<String> = env::args().collect();
    let max_workers = 4;
    if args.len() < 2 {
        panic!("Program arguments missing! please provide a file name");
    }
    let files: Vec<String> = Vec::from(&args[1..]);
    let mut pool = Pool::new(max_workers);
    pool.scoped(|scoped|  {
        for file_arg in files.iter() {
            for file_name in glob(file_arg).unwrap().filter_map(Result::ok) {
                scoped.execute(move || {
                    let path= Path::new(&file_name);
                    match process_file(path) {
                        Ok ((lines, words) ) => {
                            println!("{}\t{} lines {} words", path.display(), lines, words)
                        },
                        Err(err) => {
                            panic!("Error - {}", err)
                        }
                    };

                });
            }
        }

    });

}

fn process_file (file_path: &Path) -> Result< (i32,i32), String> {
    let file_handle = match File::open(&file_path) {
        Err(why) => return Err(why.to_string()),
        Ok(file_handle) => file_handle
    };
    let mut reader = BufReader::new(file_handle);
    let (lines, words) = counter(&mut reader)?;
    Ok((lines, words))
}

fn counter<R: BufRead> (reader: &mut R) -> Result<(i32, i32), String> {
    let mut total_lines: i32 = 0;
    let mut total_words: i32 = 0;
    let mut line = String::from("");
    loop {
        match reader.read_line(&mut line)  {
            Ok( _ ) => {
                if line.len() == 0 {
                    break;
                }
                line = line.trim().to_string();
                total_lines += 1;
                total_words += count_words(&line);
                line.clear();
            },
            Err (why) => return Err(why.to_string())
        };
    }

    Ok((total_lines, total_words))
}

fn count_words(s: &String) -> i32 {
    let mut words: i32 = 0;
    for c in s.chars() {
        if c.is_whitespace() {
            words += 1;
        }
    }

    words + 1
}
