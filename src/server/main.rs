mod configuration;
pub mod server;
use configuration::Configuration;
use server::Server;

fn main() {
    let configuration: Configuration = Configuration::new();
    // TODO: Graceful Shutdown implementation
    // Shutdown without error
    ctrlc::set_handler(|| std::process::exit(0)).expect("Error setting Ctrl-C handler");
    // Initiate and run server
    Server::new().bind(configuration).run();
}
