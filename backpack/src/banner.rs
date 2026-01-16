// banner.rs
use std::net::SocketAddr;

pub struct BannerConfig<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub tagline: Option<&'a str>,
    pub addr: Option<&'a str>,
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
    let tagline = config.tagline.unwrap_or("app.rs framework");
    let addr_line = config.addr
        .filter(|s| !s.is_empty())
        .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
        .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
        .unwrap_or_default();

    println!(
        r#"
   ____    __
  / __/___/ /  ___
 / _// __/ _ \/ _ \
/___/\__/_//_/\___/ v{version}

{tagline}

{addr_line}
"#,
        version = config.version,
        tagline = tagline,
        addr_line = addr_line,
    );
}
