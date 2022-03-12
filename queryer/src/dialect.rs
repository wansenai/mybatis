use sqlparser::dialect::Dialect;

#[derive(Debug, Default)]
pub struct TyrDialect;

// 创建sql方言，TryDialect 支持 identifier 可以为简单的url
impl Dialect for TyrDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ('a'..='z').contains(&ch) || ('A'..'Z').contains(&ch) || ch == '_'
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ('a'..= 'z').contains(&ch) 
        || ('A'..='Z').contains(&ch)
        || ('0'..='9').contains(&ch)
        || [':', '/', '?', '&', '=', '-', '_', '~', '.'].contains(&ch)
    }
}

// test sup function
pub fn example_sql() -> String {
    let url = "https://raw.githubusercontent.com/owid/covid-19-data/master/public/data/latest/owid-covid-latest.csv";

    let sql = format!(
        "SELECT location name, total_cases, new_cases, total_deaths, new_deaths FROM {} where new_deaths >= 500 ORDER BY new_cases DESC LIMIT 6 OFFSET 5",
        url
    );

    sql
}

#[cfg(test)]
mod test{
    use super::*;
    use sqlparser::parser::Parser;

    #[test]
    fn test(){
        assert!(Parser::parse_sql(&TyrDialect::default(), &example_sql()).is_ok());
    }
}