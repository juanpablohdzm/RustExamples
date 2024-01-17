mod server {
    pub struct Server {
        address: String,
    }

    impl Server {
        pub fn new(address: String) -> Self // or Server
        {
            return Server { address: address };
        }

        pub fn run(self) // this takes ownership of self, self will deallocate after run
        {
            println!("Listening on {}", self.address);
        }
    }
}
