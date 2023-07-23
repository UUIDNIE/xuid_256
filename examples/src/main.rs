use xuid_256::v7v4_xuid;

fn main() {
    loop {
      let id = v7v4_xuid();
      println!("{}", id);
    }
}
