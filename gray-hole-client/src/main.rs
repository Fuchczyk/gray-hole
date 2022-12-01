use gray_hole_lib::client::Client;

fn main() {
    let client = Client::new("34.168.167.186".parse().unwrap(), 27950).unwrap();
}
