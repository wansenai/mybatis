pub mod get_info {

    pub fn get_user(var : &str) -> String {
        return var.to_uppercase();
    }

    pub fn get_dept(mut var : i32) {
        return var += 1;
    }
}

#[test]
fn main(){
    let java = get_info::get_user("java");
    println!("{}", java);

    println!("{:?}",get_info::get_dept(5));
}