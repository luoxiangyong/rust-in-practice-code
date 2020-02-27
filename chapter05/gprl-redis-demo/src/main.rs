extern crate redis;

use redis::Commands;

use std::collections::HashMap;

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    let map : HashMap<String, String> = con.hgetall("gprl-redis-demo").unwrap();
    
    println!("修改前的gprl-redis-demo:\n{:?}", map);

    let () = con.hset("gprl-redis-demo","authors", "罗祥勇 <solo_lxy@126.com>").unwrap();
    
    
    let map : HashMap<String, String> = con.hgetall("gprl-redis-demo").unwrap();
    
    println!("修改后的gprl-redis-demo:\n{:?}", map);
}
