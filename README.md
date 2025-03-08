# DictClient 

A simple [Dictionary Server Protocol](https://datatracker.ietf.org/doc/html/rfc2229) client rust implementation

## Examples

```rust
use dict_client::Command;
use dict_client::DictClient;

fn main() {
    let mut connect = DictClient::connect("dict.catflap.org:2628");
    let resp = connect.command(Command::define("xdict", "test"));
    println!("{:?}", resp);
}   
```

Communication is an ordered sequence, so async features many `await`.
```rust
#[tokio::main]
async fn main() {
    let mut connect = AsyncDClient::connect("dict.catflap.org:2628").await;
    let resp = connect.command(Command::define("xdict", "test")).await;
    println!("{:?}", resp);
}
```
