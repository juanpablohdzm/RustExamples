fn main() {

    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Server
{
    address : String
}

impl Server 
{
    fn new(address : String) -> Self // or Server
    {
        return Server {
            address: address
        };
    }

    fn run(self) // this takes ownership of self, self will deallocate after run
    {
        println!("Listening on {}", self.address);
    }
}

struct Request 
{
    path : String,
    query_string : String,
    method : Method,
}

enum Method 
{
    GET = 0,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}