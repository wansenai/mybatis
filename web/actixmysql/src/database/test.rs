use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Test{
    id : i32,
    var1 : i32,
    var2 : Option<String>,
}

#[allow(dead_code)]
pub fn config() -> Result<()> {
    let url = "mysql://root:passw0rd@localhost:3306/COVID-19";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    conn.query_drop(
        r"CREATE TABLE test (
                id  int not null,
                var1 int not null,
                var2 text
        )"
    ).expect("exc");

    let params = vec![
        Test {id : 1, var1 : 2, var2 : None},
        Test {id : 2, var1 : 3, var2 : Some("foo".into())},
        Test {id : 3, var1 : 4, var2 : None},
        Test {id : 4, var1 : 5, var2 : None},
        Test {id : 5, var1 : 6, var2 : Some("bar".into())},
    ];

    conn.exec_batch(
        r"INSERT INTO test (id, var1, var2)
                VALUES (:id, :var1, :var2)",
        params.iter().map(|p| params ! {
            "id" => p.id,
            "var1" => p.var1,
            "var2" => &p.var2,
        })
    )?;

    let query_test = conn.query_map(
        "SELECT id, var1, var2 FROM test",
        |(id, var1, var2)| {
            Test {id, var1, var2}
        },
    )?;

    assert_eq!(params, query_test);
    println!("start....");

    Ok(())
}