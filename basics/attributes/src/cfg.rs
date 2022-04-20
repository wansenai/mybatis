///
/// cfg属性  #[cfg(....)] 配置属性
/// cfg!宏   cfg!(....) 布尔表达式
///

#[cfg(target_os = "windows")]
fn get_windows_system_info(){
    println!("获取windows系统数据")
}

#[cfg(not(target_os = "windows"))]
#[allow(dead_code)]
fn get_windows_system_info(){
    println!("监测非windows系统, 请更换操作系统")
}

#[test]
fn main(){
    get_windows_system_info();

    if cfg!(target_os = "windows"){
        println!("windows X86 4V CPU 16G SSD550")
    } else {
        println!("系统不支持")
    }
}


#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}
#[test]
fn main_rest() {
    // 没有自定义cfg标签会报错
   // conditional_function();
}
