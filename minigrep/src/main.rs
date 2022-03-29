use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("实例化Config出错 : {}", err);
        process::exit(1);
    });

    println!("正在查询关键字: {}", config.query);
    println!("文件名称: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("运行错误 : {}", e);
        process::exit(1);
    }
}