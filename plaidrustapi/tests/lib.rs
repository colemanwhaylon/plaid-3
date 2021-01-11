#[cfg(test)]
mod tests
{
    extern crate rustplaidapi;
    use rustplaidapi::PlaidLiabilities;
    use rustplaidapi::SaysHello;

    #[test]
    fn should_be_able_to_instantiate_a_client()
    {
        let c:PlaidLiabilities = SaysHello::create("John");

        //let result = c.talk();

        //let expected =  get();
        assert_eq!(1, 1);
    }

}