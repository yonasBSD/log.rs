#[cfg(test)]
mod banner_tests {
    use crate::banner::*;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

    // Constants for testing
    const GREEN: &str = "\x1b[32m";
    const RESET: &str = "\x1b[0m";

    // Test print_address function
    mod print_address_tests {
        use super::*;

        #[test]
        fn test_ipv4_with_specific_ip() {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            let result = print_address(addr);

            assert_eq!(result, format!("{GREEN}127.0.0.1:8080{RESET}"));
        }

        #[test]
        fn test_ipv4_wildcard_shows_port_only() {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 3000);
            let result = print_address(addr);

            assert_eq!(result, format!("{GREEN}:3000{RESET}"));
        }

        #[test]
        fn test_ipv6_wildcard_shows_port_only() {
            let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0)), 8080);
            let result = print_address(addr);

            assert_eq!(result, format!("{GREEN}:8080{RESET}"));
        }

        #[test]
        fn test_ipv6_with_specific_ip() {
            let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)), 8080);
            let result = print_address(addr);

            assert_eq!(result, format!("{GREEN}::1:8080{RESET}"));
        }

        #[test]
        fn test_different_ports() {
            let ports = [80, 443, 3000, 8080, 9000, 65535];

            for port in ports {
                let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);
                let result = print_address(addr);

                assert_eq!(result, format!("{GREEN}:{port}{RESET}"));
            }
        }

        #[test]
        fn test_ansi_color_codes_present() {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
            let result = print_address(addr);

            assert!(result.starts_with(GREEN));
            assert!(result.ends_with(RESET));
        }

        #[test]
        fn test_port_zero() {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 0);
            let result = print_address(addr);

            assert_eq!(result, format!("{GREEN}127.0.0.1:0{RESET}"));
        }

        #[test]
        fn test_max_port() {
            let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 65535);
            let result = print_address(addr);

            assert_eq!(result, format!("{GREEN}127.0.0.1:65535{RESET}"));
        }
    }

    // Test BannerConfig struct
    mod banner_config_tests {
        use super::*;

        #[test]
        fn test_banner_config_creation() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: Some("A test application"),
                addr: Some("127.0.0.1:8080"),
            };

            assert_eq!(config.name, "TestApp");
            assert_eq!(config.version, "1.0.0");
            assert_eq!(config.tagline, Some("A test application"));
            assert_eq!(config.addr, Some("127.0.0.1:8080"));
        }

        #[test]
        fn test_banner_config_with_none_values() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: None,
            };

            assert_eq!(config.tagline, None);
            assert_eq!(config.addr, None);
        }

        #[test]
        fn test_banner_config_lifetime() {
            let name = String::from("TestApp");
            let version = String::from("1.0.0");

            let config = BannerConfig {
                name: &name,
                version: &version,
                tagline: None,
                addr: None,
            };

            assert_eq!(config.name, "TestApp");
        }
    }

    // Test print function behavior
    mod print_tests {
        use super::*;

        // Helper to capture output would go here
        // For now, we test the logic indirectly

        #[test]
        fn test_default_tagline_when_none() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: None,
            };

            // The default tagline should be "app.rs framework"
            let tagline = config.tagline.unwrap_or("app.rs framework");
            assert_eq!(tagline, "app.rs framework");
        }

        #[test]
        fn test_custom_tagline_when_some() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: Some("Custom tagline"),
                addr: None,
            };

            let tagline = config.tagline.unwrap_or("app.rs framework");
            assert_eq!(tagline, "Custom tagline");
        }

        #[test]
        fn test_addr_line_empty_when_none() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: None,
            };

            let addr_line = config
                .addr
                .filter(|s| !s.is_empty())
                .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
                .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
                .unwrap_or_default();

            assert_eq!(addr_line, "");
        }

        #[test]
        fn test_addr_line_empty_when_empty_string() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: Some(""),
            };

            let addr_line = config
                .addr
                .filter(|s| !s.is_empty())
                .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
                .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
                .unwrap_or_default();

            assert_eq!(addr_line, "");
        }

        #[test]
        fn test_addr_line_with_valid_address() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: Some("127.0.0.1:8080"),
            };

            let addr_line = config
                .addr
                .filter(|s| !s.is_empty())
                .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
                .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
                .unwrap_or_default();

            assert!(addr_line.contains("⇨"));
            assert!(addr_line.contains("TestApp"));
            assert!(addr_line.contains("listening on"));
            assert!(addr_line.contains("127.0.0.1:8080"));
        }

        #[test]
        fn test_addr_line_with_wildcard_address() {
            let config = BannerConfig {
                name: "MyServer",
                version: "2.0.0",
                tagline: None,
                addr: Some("0.0.0.0:3000"),
            };

            let addr_line = config
                .addr
                .filter(|s| !s.is_empty())
                .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
                .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
                .unwrap_or_default();

            assert!(addr_line.contains("MyServer"));
            assert!(addr_line.contains(":3000"));
            assert!(!addr_line.contains("0.0.0.0"));
        }

        #[test]
        fn test_addr_line_with_invalid_address() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: Some("invalid:address"),
            };

            let addr_line = config
                .addr
                .filter(|s| !s.is_empty())
                .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
                .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
                .unwrap_or_default();

            // Invalid address should result in empty string
            assert_eq!(addr_line, "");
        }

        #[test]
        fn test_addr_line_with_ipv6_address() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: Some("[::1]:8080"),
            };

            let addr_line = config
                .addr
                .filter(|s| !s.is_empty())
                .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
                .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
                .unwrap_or_default();

            assert!(addr_line.contains("TestApp"));
            assert!(addr_line.contains("listening on"));
            assert!(addr_line.contains("::1"));
        }
    }

    // Integration tests
    mod integration_tests {
        use super::*;

        #[test]
        fn test_full_config_with_all_fields() {
            let config = BannerConfig {
                name: "MyApp",
                version: "1.2.3",
                tagline: Some("The best app ever"),
                addr: Some("127.0.0.1:8080"),
            };

            // Test that config can be used
            assert_eq!(config.name, "MyApp");
            assert_eq!(config.version, "1.2.3");
            assert_eq!(config.tagline.unwrap(), "The best app ever");
            assert_eq!(config.addr.unwrap(), "127.0.0.1:8080");
        }

        #[test]
        fn test_minimal_config() {
            let config = BannerConfig {
                name: "MinimalApp",
                version: "0.1.0",
                tagline: None,
                addr: None,
            };

            assert_eq!(config.name, "MinimalApp");
            assert_eq!(config.version, "0.1.0");
            assert!(config.tagline.is_none());
            assert!(config.addr.is_none());
        }

        #[test]
        fn test_config_with_only_addr() {
            let config = BannerConfig {
                name: "ServerApp",
                version: "1.0.0",
                tagline: None,
                addr: Some("0.0.0.0:8080"),
            };

            assert!(config.addr.is_some());
            assert!(config.tagline.is_none());
        }

        #[test]
        fn test_config_with_only_tagline() {
            let config = BannerConfig {
                name: "InfoApp",
                version: "1.0.0",
                tagline: Some("Just information"),
                addr: None,
            };

            assert!(config.tagline.is_some());
            assert!(config.addr.is_none());
        }
    }

    // Edge case tests
    mod edge_cases {
        use super::*;

        #[test]
        fn test_empty_name() {
            let config = BannerConfig {
                name: "",
                version: "1.0.0",
                tagline: None,
                addr: None,
            };

            assert_eq!(config.name, "");
        }

        #[test]
        fn test_empty_version() {
            let config = BannerConfig {
                name: "TestApp",
                version: "",
                tagline: None,
                addr: None,
            };

            assert_eq!(config.version, "");
        }

        #[test]
        fn test_very_long_name() {
            let long_name = "A".repeat(1000);
            let config = BannerConfig {
                name: &long_name,
                version: "1.0.0",
                tagline: None,
                addr: None,
            };

            assert_eq!(config.name.len(), 1000);
        }

        #[test]
        fn test_special_characters_in_tagline() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: Some("App with 🚀 emoji and <special> chars"),
                addr: None,
            };

            assert!(config.tagline.unwrap().contains("🚀"));
            assert!(config.tagline.unwrap().contains("<special>"));
        }

        #[test]
        fn test_multiline_tagline() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: Some("Line 1\nLine 2\nLine 3"),
                addr: None,
            };

            assert!(config.tagline.unwrap().contains('\n'));
        }

        #[test]
        fn test_whitespace_only_addr() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0",
                tagline: None,
                addr: Some("   "),
            };

            // Whitespace-only should not be filtered as empty
            // but will fail to parse as SocketAddr
            let addr_line = config
                .addr
                .filter(|s| !s.is_empty())
                .and_then(|addr_str| addr_str.parse::<SocketAddr>().ok())
                .map(|addr| format!(" ⇨ {} listening on {}", config.name, print_address(addr)))
                .unwrap_or_default();

            assert_eq!(addr_line, "");
        }

        #[test]
        fn test_version_with_pre_release() {
            let config = BannerConfig {
                name: "TestApp",
                version: "1.0.0-alpha.1+build.123",
                tagline: None,
                addr: None,
            };

            assert_eq!(config.version, "1.0.0-alpha.1+build.123");
        }
    }

    // Test address parsing edge cases
    mod address_parsing_tests {
        use super::*;

        #[test]
        fn test_parse_localhost() {
            let addr_str = "127.0.0.1:8080";
            let parsed = addr_str.parse::<SocketAddr>();

            assert!(parsed.is_ok());
            assert_eq!(parsed.unwrap().port(), 8080);
        }

        #[test]
        fn test_parse_ipv6_localhost() {
            let addr_str = "[::1]:8080";
            let parsed = addr_str.parse::<SocketAddr>();

            assert!(parsed.is_ok());
        }

        #[test]
        fn test_parse_invalid_formats() {
            let invalid_addrs = vec![
                "not-an-address",
                "127.0.0.1",          // missing port
                ":8080",              // missing IP
                "127.0.0.1:99999",    // port too high
                "999.999.999.999:80", // invalid IP
                "localhost:8080",     // hostname not supported
            ];

            for addr in invalid_addrs {
                let parsed = addr.parse::<SocketAddr>();
                assert!(parsed.is_err(), "Should fail to parse: {}", addr);
            }
        }

        #[test]
        fn test_parse_edge_ports() {
            // Port 1 (minimum valid port)
            assert!("127.0.0.1:1".parse::<SocketAddr>().is_ok());

            // Port 65535 (maximum valid port)
            assert!("127.0.0.1:65535".parse::<SocketAddr>().is_ok());
        }
    }
}
