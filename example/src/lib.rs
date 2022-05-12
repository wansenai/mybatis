#[macro_use]
extern crate mybatis_macro;

use mybatis::mybatis::Mybatis;

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4+4, 8);
    }
}

/// this is table model(see ../database.sql)

#[derive(CRUDTable, Clone, Debug)]
pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub pc_banner_img: Option<String>,
    pub h5_banner_img: Option<String>,
    pub sort: Option<String>,
    pub status: Option<i32>,
    pub remark: Option<String>,
    pub version: Option<i64>,
    pub delete_flag: Option<i32>,
}

#[py_sql("example/example.html")]
pub async fn py_select_rb(mybatis: &Mybatis, name: &str) -> Option<BizActivity> {
    
}