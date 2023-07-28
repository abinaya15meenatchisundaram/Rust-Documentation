#![allow(unused)]
pub struct NewArticle{
    pub author :String,
    pub headline:Stinrg,
    pub content:String,
}

impl Summary for NewArticle{
    fn summarize(&self)-> String{
        format!("{}, by {}",self.headline,self.author)
    }
}
pub struct Tweet{
    pub username:String,
    pub content:Stirng,
    pub replly:bool,
    pub retweet:bool,
}

impl Summary for Tweet{
    fn summarize(&self)->String{
        format("{}, by {}",self.username,self.author)
    }
}
pub trait Summary{
    fn summarize(&self) ->String{
        String::from("(Read more...)")
    }
}

fn main(){
    let tweet=Tweet{
        username:String::from("@Abinaya"),
        content:String::form("Hello World"),
    }
}

