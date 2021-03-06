extern crate iron;
extern crate static_file;
extern crate mount;

// This example serves the docs from target/doc/static_file at /doc/
//
// Run `cargo doc && cargo test && ./target/doc_server`, then
// point your browser to http://127.0.0.1:3000/doc/

use std::io::net::ip::Ipv4Addr;

use iron::Iron;
use static_file::Static;
use mount::Mount;

fn main() {
    let mut mount = Mount::new();

    // Serve the shared JS/CSS at /
    mount.mount("/", Static::new(Path::new("target/doc/")));
    // Serve the static file docs at /doc/
    mount.mount("/doc/", Static::new(Path::new("target/doc/static_file/")));
    // Serve the source code at /src/
    mount.mount("/src/", Static::new(Path::new("target/doc/src/static_file/src/lib.rs.html")));

    Iron::new(mount).listen(Ipv4Addr(127, 0, 0, 1), 3000);

    println!("Doc server running on http://localhost:3000/doc/");
}
