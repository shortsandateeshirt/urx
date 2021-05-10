use urx::example;
use std::thread;
fn main(){
    let mut c = vec![];

    for i in 0..16{
        c.push(thread::spawn(move || {
            println!("thread number {}",i);
        }))
    }
    for i in 16..32{
        c.push(thread::spawn(move || {
            println!("thread number {}",i);
        }))
    }
    for j in c{
        j.join();
    }
    println!("Hello!");
}