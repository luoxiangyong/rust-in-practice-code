extern crate gprl_lib_demo;


fn main() {
    println!("本程序演示如何使用外部库");

    let result =  gprl_lib_demo::compute_add(1,1000);

    println!("调用结果为:{}", result);
}
