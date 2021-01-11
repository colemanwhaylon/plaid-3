
//mod genericclient;
pub use self::types::*;

mod types;

pub trait SaysHello
{
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;

    fn talk(&self) 
    {
        println!("{} cannot talk", self.name());
    }

    fn say_hello(&self) -> String
    {
        "Hello".to_string()
    }
}

pub struct PlaidLiabilities
{
    pub name: &'static str
}

impl SaysHello for PlaidLiabilities
{
     fn create(name: &'static str) -> PlaidLiabilities
    {
        PlaidLiabilities{name: name}
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn talk(&self)
    {
        println!("{} says hello", self.name());
    }

    fn say_hello(&self) -> String
    {
        "Hello".to_string()
    }
}
