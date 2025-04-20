use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead, Write},
    net::ToSocketAddrs,
    path::Path,
    process::Command,
};
use socks::Socks5Stream;
use std::net::SocketAddr;
use clap::{Parser, ArgGroup};
use url::Url;

mod func {
    pub mod print_banner;
    pub mod logger;
}
use func::print_banner::print_banner;
use func::logger::{alert, info, question, starred};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about,
    long_about = "Revela IPs unicos de URLs, com opçao Tor, IPv6, filtro CF e saída customizada"
)]
#[command(group(
    ArgGroup::new("input")
        .required(true)
        .args(&["list", "urls"]),
))]
struct Cli {
    #[arg(short = 'l', long = "list", value_name = "FILE")]
    list: Option<String>,

    #[arg(short = 'u', long = "urls", value_name = "URL1,URL2,...")]
    urls: Option<String>,

    #[arg(short = 'o', long = "output-list", value_name = "FILE")]
    output_list: Option<String>,

    #[arg(short = 't', long = "tor")]
    tor: bool,

    #[arg(short = 'v', long = "verbose")]
    verbose: bool,

    #[arg(long = "no-cloudflare")]
    no_cloudflare: bool,

    #[arg(long = "ipv6")]
    ipv6: bool,
}

pub fn resolve_via_tor(hostname: &str) -> Vec<String> {
    let proxy = "127.0.0.1:9050";
    let target = (hostname, 80);
    match Socks5Stream::connect(proxy, target) {
        Ok(stream) => {
            let peer: SocketAddr = stream.get_ref().peer_addr()
                .expect("nao consegui obter peer_addr do SOCKS5Stream");
            vec![peer.ip().to_string()]
        }
        Err(err) => {
            eprintln!("[!] Erro ao resolver via Tor ({}): {}", hostname, err);
            Vec::new()
        }
    }
}

pub fn resolve_hostname(
    hostname: &str,
    _use_ipv6: bool,
    use_tor: bool
) -> Vec<String> {
    if use_tor {
        resolve_via_tor(hostname)
    } else {
        let addr = format!("{}:80", hostname);
        match addr.to_socket_addrs() {
            Ok(iter) => iter.map(|sock| sock.ip().to_string()).collect(),
            Err(_) => {
                eprintln!("[!] Erro ao resolver: {}", hostname);
                Vec::new()
            }
        }
    }
}

fn read_urls_from_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    File::open(path)
        .and_then(|f| {
            let reader = io::BufReader::new(f);
            Ok(reader.lines().filter_map(Result::ok).collect())
        })
}

fn extract_hostname(url: &str) -> Option<String> {
    Url::parse(url).ok().and_then(|u| u.host_str().map(|h| h.to_string()))
}

fn is_cloudflare_ip(ip: &str) -> bool {
    const CF_RANGES: &[&str] = &[
        // — IPv4 — :contentReference[oaicite:0]{index=0}
        "173.245.48.0/20",
        "103.21.244.0/22",
        "103.22.200.0/22",
        "103.31.4.0/22",
        "141.101.64.0/18",
        "108.162.192.0/18",
        "190.93.240.0/20",
        "188.114.96.0/20",
        "197.234.240.0/22",
        "198.41.128.0/17",
        "162.158.0.0/15",
        "104.16.0.0/12",
        "172.64.0.0/13",
        "131.0.72.0/22",

        // — IPv6 — :contentReference[oaicite:1]{index=1}
        "2400:cb00::/32",
        "2606:4700::/32",
        "2803:f800::/32",
        "2405:b500::/32",
        "2405:8100::/32",
        "2a06:98c0::/29",
        "2c0f:f248::/32",
    ];

    CF_RANGES.iter().any(|cidr| {
        let base = cidr.split('/').next().unwrap();
        ip.starts_with(base)
    })
}

fn main() {
    let cli = Cli::parse();

    if cli.tor && env::var("TORSOCKS_RUN").is_err() {
        let exe = env::current_exe().expect("nao achei o executavel");
        let mut cmd = Command::new("torsocks");
        cmd.arg(exe).args(env::args().skip(1));
        let status = cmd.status().expect("falha ao lançar torsocks");
        std::process::exit(status.code().unwrap_or(1));
    }

    print_banner();
    if cli.verbose { info("Iniciando RevealNet..."); }

    let urls: Vec<String> = if let Some(file) = &cli.list {
        if cli.verbose { info(&format!("Lendo URLs de '{}'", file)); }
        read_urls_from_file(file).unwrap_or_else(|_| {
            alert(&format!("Nao consegui abrir o arquivo: {}", file));
            std::process::exit(1);
        })
    } else {
        cli.urls
            .as_ref()
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect()
    };

    if cli.verbose { info(&format!("Total de URLs: {}", urls.len())); }

    let mut unique_ips = HashSet::new();
    let mut resolved_hosts = HashSet::new();

    for url in urls {
        match extract_hostname(&url) {
            Some(host) => {
                if !resolved_hosts.insert(host.clone()) {
                    if cli.verbose {
                        info(&format!("Host '{}' ja resolvido, pulando", host));
                    }
                    continue;
                }

                if cli.verbose {
                    info(&format!("Resolvendo '{}'", host));
                }
                for ip in resolve_hostname(&host, cli.ipv6, cli.tor) {
                    if cli.no_cloudflare && is_cloudflare_ip(&ip) {
                        if cli.verbose {
                            question(&format!("Pulando CF IP {}", ip));
                        }
                    } else {
                        unique_ips.insert(ip.clone());
                        if cli.verbose {
                            starred(&format!("Encontrado: {}", ip));
                        }
                    }
                }
            }
            None => alert(&format!("URL invalida: {}", url)),
        }
    }

    starred("IPs unicos encontrados:");
    for ip in &unique_ips {
        println!("  {}", ip);
    }

    if let Some(out) = &cli.output_list {
        if cli.verbose { info(&format!("Salvando em '{}'", out)); }
        let mut f = File::create(out).unwrap_or_else(|_| {
            alert(&format!("Nao consegui criar arquivo: {}", out));
            std::process::exit(1);
        });
        for ip in &unique_ips {
            writeln!(f, "{}", ip).unwrap();
        }
    }

    question("Execute `revealnet -h` para ver todas as opções.");
}
