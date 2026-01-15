use std::net::SocketAddr;

pub struct BannerConfig<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub tagline: &'a str,
    pub addr: &'a str,
}

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

pub fn print_address(addr: SocketAddr) -> String {
    let ip = addr.ip().to_string();
    let port = addr.port();

    // Echo-style: if bound to 0.0.0.0, show only :PORT
    let display = if ip == "0.0.0.0" || ip == "::" {
        format!(":{port}")
    } else {
        format!("{ip}:{port}")
    };

    format!("{GREEN}{display}{RESET}")
}

pub fn print(config: &BannerConfig<'_>) {
    let addr: SocketAddr = config.addr.parse().unwrap();

    println!(
        r#"
   ____    __
  / __/___/ /  ___
 / _// __/ _ \/ _ \
/___/\__/_//_/\___/ v{version}

{tagline}

 ⇨ {name} listening on {addr}
"#,
        version = config.version,
        tagline = config.tagline,
        name = config.name,
        addr = print_address(addr),
    );
}
