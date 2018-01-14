mod tags;
mod server;

use server::Server;

fn main() {
    Server::start_with_config_file("enigma.yaml");
}
