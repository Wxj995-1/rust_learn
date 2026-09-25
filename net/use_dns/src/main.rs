use std::env;

use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("please provide a name to query!");
        std::process::exit(1);
    }

    let query = format!("{}.", args[1]);
    let resolver =
        Resolver::new(ResolverConfig::default(), ResolverOpts::default()).unwrap();

    let response = resolver.lookup_ip(query.as_str()).unwrap();
    for ans in response.iter() {
        println!("{:?}", ans);
    }
}