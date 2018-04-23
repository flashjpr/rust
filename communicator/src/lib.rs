pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);

        // start from the root
        ::client::connect();

        // start from the parent of the current module
        super::client::connect();

        // both from above achieve the same, really

        // or, make good 'use'
        use super::client;
        client::connect();
    }
}
