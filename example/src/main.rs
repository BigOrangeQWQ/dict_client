use dict_client::Command;
use dict_client::DictClient;

fn main() {
    let mut connect = DictClient::connect("dict.catflap.org:2628");
    let resp = connect.command(Command::define("xdict", "test"));
    println!("{:?}", resp);
}   