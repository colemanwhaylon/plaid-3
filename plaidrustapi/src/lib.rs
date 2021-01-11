pub mod greetings
{
    pub mod english;

    pub mod spanish
    {
        pub fn hello() -> String { "hola".to_string() }
        pub fn goodbye() -> String { "asta la vista".to_string() }
    }
}
