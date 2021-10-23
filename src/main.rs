mod uni;
use uni::particle::*;
use uni::universe::*;
use uni::book::Book;
use uni::time::Time;

use std::collections::HashMap;
#[allow(dead_code)]

fn main() {
    if std::env::args().len() > 1 {
    time_module();
    return
    }
    
    uni::hello();
    let mut particles:Vec<Particle<i64, Book>> = vec![];

    let mut book = Book::new(String::from("Symbols of Transformation"),String::from("Carl Jung"));
    Book::get_book_contents();
    let mut book_contents = HashMap::new();
    book_contents.insert(book.chapter_count, "Two types of thinking".to_string());
    book.add_chapter(book_contents);
    book.summarize();
    println!("{:?}",sleeper(33));
    book.summarize();
    let book_particle = Particle::new(1, book);
    //book_particle.display();
    particles.push(book_particle);
  
}
fn time_module(){
    let args:Vec<String> = std::env::args().collect();
    if args.len()> 1 && args[1] == "time"{
        println!("{:?}", Time::now());
    }
}
fn sleeper(n:i32) -> i32 {
    let mut one_and_two:HashMap<i32, i32> = HashMap::new();
    one_and_two.insert(1,1);
    one_and_two.insert(2,1);
    if n < 2 {
        return 1
    } 

    return sleeper(n - 1) + sleeper(n - 2) 
}







