use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("没有足够请求参数")
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {query, filename, case_sensitive})
    }

    pub fn new_two(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("没有足够请求参数")
        }
        args.next();

        let query = match args.next(){
            Some(arg) => arg,
            None => return Err("没有找到query参数")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("没有找到filename参数")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

///
/// lib test metod
/// 
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_one(){
        let query = "hello";
        let contents = "\
        hello,
        this project is learing about 
        rust kwlodge.";

        assert_eq!(vec!["hello,"], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "hi,";
        let contents = "\
        hi,
        good morning
        good evening";

        assert_eq!(vec!["hi,"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        Rust
        Java
        C#";

        assert_eq!(vec!["Rust"], search_case_insensitive(query, contents));
    }

    #[test]
    fn test_sss() {
        let s1 = String::from("hello");
        let s2 = &s1;
        // ⚠️：这里会导致编译错误 s1的所有权已经交给了s2 此时在调用s1会出错
        println!("{},{}", s1, s2);

        let a = [1, 2, 3, 4, 5];
  
        let slice = &a[1..3];

        println!("{:?}", slice);

    }

    #[test]
    fn test_inte() {
        let v = vec![1, 2, 3, 4];

        let v_iter = v.iter();

        let total: i32 = v_iter.sum();

        assert_eq!(10, total);
    }
}