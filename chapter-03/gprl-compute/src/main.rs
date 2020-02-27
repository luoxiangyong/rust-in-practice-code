extern crate clap; 

use clap::{Arg, App, SubCommand};
 
fn main() { 
    let app = App::new("gprl-compute")
                   .version("0.1.0")
                   .about("简单计算演示程序")
                   .author("Luo Xiangyong <luoxiangyong@topgridcloud.com>")
                   .subcommand(SubCommand::with_name("add")
                              .about("compute add to integer")
                              .arg(Arg::with_name("first")
                                    .required(true)
                                    .takes_value(true)
                                    .short("a")
                                    .help("第一个整数")
                                      )
                              .arg(Arg::with_name("second")
                                    .required(true)
                                    .takes_value(true)
                                    .short("b")
                                    .help("第二个整数")
                                  ))
                    .subcommand(SubCommand::with_name("sub")
                              .about("compute sub to integer")
                              .arg(Arg::with_name("first")
                                    .required(true)
                                    .takes_value(true)
                                    .short("a")
                                    .help("第一个整数")
                                      )
                              .arg(Arg::with_name("second")
                                    .required(true)
                                    .takes_value(true)
                                    .short("b")
                                    .help("第二个整数")
                                  )
                      );
                    
    let matches = app.get_matches(); 
                    
    match matches.subcommand() {
        ("add",  Some(sub_m)) => {
            if sub_m.is_present("first") {
                println!("Value for first integer is:{:?}",sub_m.value_of("first").unwrap());
            } 
            
            if sub_m.is_present("second") {
                println!("Value for second integer is:{:?}",sub_m.value_of("second").unwrap());
            } 
            
            let first = sub_m.value_of("first").unwrap().parse::<i32>().unwrap();
            let second = sub_m.value_of("second").unwrap().parse::<i32>().unwrap();
            
            println!("Add计算结果为:{}", first + second);
            
            return;   
        
        },
        ("sub",  Some(sub_m)) => {
            if sub_m.is_present("first") {
                println!("Value for first integer is:{:?}",sub_m.value_of("first").unwrap());
            } 
            
            if sub_m.is_present("second") {
                println!("Value for second integer is:{:?}",sub_m.value_of("second").unwrap());
            } 
            
            let first = sub_m.value_of("first").unwrap().parse::<i32>().unwrap();
            let second = sub_m.value_of("second").unwrap().parse::<i32>().unwrap();
            
            println!("Sub计算结果为:{}", first - second);
            
            return;   
        
        },
        _ => {
            println!("{}",matches.usage());
            return;
        }
    }    
}
