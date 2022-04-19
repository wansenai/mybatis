#[allow(dead_code)]
fn copy_return<'life>(vector: &'life mut Vec<String>, value: &'life str) -> &'life String {
    vector.push(String::from(value));
    // cancel ; return
    vector.get(vector.len() -1).unwrap()
}

#[test]
fn get_info_all(){
    let var_one = "status";
    let var_two = String::from("type");

    let mut other_info = Vec::new();

    assert_eq!("status", copy_return(&mut other_info, &var_one));
    assert_eq!("type", copy_return(&mut other_info, &var_two));

    assert_eq!(
        other_info,
        vec!["status".to_string(), "type".to_string()]
    );
    let copy_info = String::from(var_one);
    let var_info = var_two;
    println!("{}", var_info);
    if var_info != copy_info{
        other_info.push(copy_info);
        // panic!("not found other_info")
    }
    for item in other_info {
        println!("{:?}", item);
    }
}