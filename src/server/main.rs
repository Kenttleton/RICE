mod configuration;
pub mod server;
use configuration::Configuration;
use server::Server;

fn main() {
    let configuration: Configuration = Configuration::new();
    let server = Server::new();
    // Shutdown without error
    ctrlc::set_handler(|| {
        server.kill();
        std::process::exit(0)
    })
    .expect("Error setting Ctrl-C handler");
    // Initiate and run server
    server.bind(configuration).run();
}
