use anyhow::{anyhow, Result};
use clap::Parser;
use colored::Colorize;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings}
};

/// 显示终端header output info
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "James Zow <JamesZow@163.com>")]

/// 输出命令 结构体
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

/// 命令枚举 包含get 和 post处理命令
#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// get 结构体
#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

// post 结构体
#[derive(Parser, Debug)]
struct Post {
    // 请求url
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    // 请求体
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KVPair>,
}

// post请求body参数kv形式封装 结构体
#[derive(Debug, PartialEq)]
struct KVPair {
    k: String,
    v: String,
}

// 对body封装对数据进行解析转换，分割参数
impl FromStr for KVPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // == 分割
        let mut split = s.split("=");
        let err = || anyhow!(format!("参数分割错误：{}", s));
        Ok(Self {
            k : (split.next().ok_or_else(err)?).to_string(),
            v : (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

// 解析url reqwest::Url
fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

// 参数kv解析
fn parse_kv_pair(s: &str) -> Result<KVPair> {
    Ok(s.parse()?)
}

// 打印响应状态
fn print_status(resp: &Response) {
    let status = format!("version: {:?},  status: {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

// 打印响应headers
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }

    println!()
}

// 打印解析出来的body json结构
fn print_body(m: Option<Mime>, body: &str) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => print_syntect(body, "json"),
        Some(v) if v == mime::TEXT_HTML => print_syntect(body, "html"),

        _ => println!("{}", body),
    }
}

// 主题格式化打印
fn print_syntect(s: &str, ext: &str) {
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();
    let syntax = ps.find_syntax_by_extension(ext).unwrap();
    let mut h = HighlightLines::new(syntax, &ts.themes["base16-ocean.dark"]);

    for line in LinesWithEndings::from(s) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        print!("{}", escaped);
    }
}

// 获取响应后的Content-Type
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

// 打印响应信息
async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

// get处理命令
async fn get(client: Client, args: &Get) -> Result<()>{
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

// post处理命令
async fn post(client: Client, args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// main application 
#[tokio::main]
async fn main() -> Result<()> {
    let opts: Opts = Opts::parse(); 
    let mut headers = header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);

    // creat htto client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

        let result = match opts.subcmd {
            SubCommand::Get(ref args) => get(client, args).await?,
            SubCommand::Post(ref args) => post(client, args).await?,
        };
    
    Ok(result)
}

///
/// unit test
/// 
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_url_test(){
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok()); 
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_test(){
        assert!(parse_kv_pair("s").is_err());

        assert_eq!( 
            parse_kv_pair("a=1").unwrap(), 
            KVPair { 
                k: "a".into(), 
                v: "1".into() 
            } 
        );

        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KVPair { 
                k: "b".into(), 
                v: "".into() 
            } 
        )
    }

    #[test]
    fn parse_kv_pair_print_test(){
        println!("{:?}", parse_url("https://httpbin.org/post"));

        println!("{:?}", parse_kv_pair("https://httpbin.org/post a=1"));
    }
}