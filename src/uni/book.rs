use std::collections::HashMap;
use std::io::{self, Read};
use chrono::prelude::*;

#[derive(Debug)]
pub struct Book{
    pub title: String,
    pub author: String,
    //pub chapters: i64,
    pub chapter_count: i64,
    pub contents: Vec<HashMap<i64,String>>,
    pub created_on: String,
    pub edited: bool,

}
impl Book{
    pub fn new(title: String, author: String/*, chapter_count: i64, contents: Vec<String>*/) -> Book{
        let created_on = Local::now();
        Book{
            title,
            author,
            chapter_count: 0,
            contents: vec![],
            edited: false,
            created_on: created_on.to_string()
        }
    }
    pub fn summarize(&self){
        println!("{:?} : {:?}", &self.author, &self.title);
        println!("Edited on : {:?}", &self.created_on);    
    
    }
        
    pub fn add_chapter(&mut self, content: HashMap<i64, String>){
        self.edited == true;
        self.created_on == Local::now().to_string();
        self.chapter_count += 1;
        &self.contents.push(content);
    }

    pub fn get_book_contents() -> io::Result<()>{
        let mut buffer = String::new();
        println!("Hello, please type in the chapter number");
        io::stdin().read_line(&mut buffer)?;
        println!("Thank you, editing chapter {}", &buffer);
        Ok(())
    }
    
}