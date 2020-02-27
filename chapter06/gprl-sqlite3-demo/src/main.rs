extern crate sqlite;


fn main() {
    let connection = sqlite::open("gprl-sqlite3-demo.db").unwrap();
    
    println!("修改前的数据为：");
    connection.iterate("SELECT * FROM my_demo_apps", |pairs| {
        for &(column, value) in pairs.iter() {
            print!("{} = {} | ", column, value.unwrap());
        }
        println!("");
        true
    }).unwrap();
    
    connection.execute("update my_demo_apps set authors="\祥勇\"　where id=1;").unwrap();
    
    println!("修改后的数据为：");
    connection.iterate("SELECT * FROM my_demo_apps", |pairs| {
        for &(column, value) in pairs.iter() {
            print!("{} = {} | ", column, value.unwrap());
        }
        println!("");
        true
    }).unwrap();
}
