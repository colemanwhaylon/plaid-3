#[cfg(test)]
mod tests
{

    extern crate phrases;

    #[test]
    fn english_greeting_correct()
    {
        assert_eq!("hello", phrases::greetings::english::hello());
    }

    #[test]
    fn spanish_greeting_correct()
    {
        assert_eq!("hola", phrases::greetings::spanish::hello());
    }

    #[test]
    fn should_be_able_to_instantiate_a_client()
    {
        assert_eq!("hola", phrases::greetings::spanish::hello());
    }

}