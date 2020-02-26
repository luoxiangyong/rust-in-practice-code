extern crate clap; 

use clap::App; 
 
fn main() { 
    App::new("gprl-compute")
       .version("0.1.0")
       .about("简单计算演示程序")
       .author("Luo Xiangyong <luoxiangyong@topgridcloud.com>")
       .get_matches(); 
}
