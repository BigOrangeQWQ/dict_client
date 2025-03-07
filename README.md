# DictClient 

A simple [Dictionary Server Protocol](https://datatracker.ietf.org/doc/html/rfc2229) client rust implementation

## Exmaples 

```rust
use dict_client::Command;
use dict_client::DictClient;

fn main() {
    let mut connect = DictClient::connect("dict.catflap.org:2628");
    let resp = connect.command(Command::define("xdict", "test"));
    println!("{:?}", resp);
}   
```

## Future

- Support for async traits.