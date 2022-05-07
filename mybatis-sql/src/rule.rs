use crate::template::TEMPLATE;
use mybatis_core::db::DriverType;

pub trait SqlRule {
    fn make_where(&self, where_sql: &str) -> String {
        let sql = where_sql.trim_start();
        if sql.is_empty() {
            return String::new();
        }
        if sql.starts_with(TEMPLATE.order_by.right_space)
            || sql.starts_with(TEMPLATE.group_by.right_space)
            || sql.starts_with(TEMPLATE.limit.right_space)
        {
            sql.to_string()
        } else {
            format!(
                " {} {} ",
                TEMPLATE.r#where.value,
                sql.trim_start_matches(TEMPLATE.r#where.right_space)
                    .trim_start_matches(TEMPLATE.and.right_space)
                    .trim_start_matches(TEMPLATE.or.right_space)
            )
        }
    }

    fn make_left_insert_where(&self, insert_sql: &str, where_sql: &str) -> String {
        let sql = where_sql
            .trim()
            .trim_start_matches(TEMPLATE.r#where.right_space)
            .trim_start_matches(TEMPLATE.and.right_space);
        if sql.is_empty() {
            return insert_sql.to_string();
        }
        if sql.starts_with(TEMPLATE.order_by.right_space)
            || sql.starts_with(TEMPLATE.group_by.right_space)
            || sql.starts_with(TEMPLATE.limit.right_space)
        {
            format!(
                " {} {} {}",
                TEMPLATE.r#where.value,
                insert_sql
                    .trim()
                    .trim_end_matches(TEMPLATE.and.left_space),
                sql
            )
        } else {
            format!(
                " {} {} {} {}",
                TEMPLATE.r#where.value,
                insert_sql
                    .trim()
                    .trim_end_matches(TEMPLATE.and.left_space),
                TEMPLATE.and.value,
                sql
            )
        }
    }
}

impl SqlRule for DriverType {}