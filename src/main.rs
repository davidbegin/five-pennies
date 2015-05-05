#[macro_use] extern crate nickel;

use nickel::Nickel;
use nickel::router::http_router::HttpRouter;

fn main() {
    let mut server = Nickel::new();

    server.utilize(router! {
           get "**" => |_req, _res| {
                "Hello, world"
           }
    });

    server.listen("127.0.0.1:1987");
}
