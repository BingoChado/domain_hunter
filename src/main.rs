use url::Url;
use clap::{Arg, App};
use tokio;


mod log;
mod stats;
mod spider;
mod common;
mod active;
mod passive;
use crate::log::{
    log,
    LogType
};




#[tokio::main]
async fn main() {
    let matches = App::new("Domain Hunter")
			.version("0.1.0")
			.about("Active OSINT tool for discovering subdomains")
			.setting(clap::AppSettings::ArgRequiredElseHelp)
            .arg(Arg::with_name("domain")
                .short("d")
                .long("domain")
                .takes_value(true)
                .help("The base domain to begin searching from"))
            .arg(Arg::with_name("passive")
                .short("p")
                .long("passive")
				.help("Only use passive techniques")
			)
            .arg(Arg::with_name("spider")
                .short("s")
                .long("spider")
                .help("Crawl for subdomains")
            )
			.get_matches();

    // fetch our domain
    let domain_url = match matches.value_of("domain") {
        Some(a) => a,
        None => {
            log(LogType::LogCrit, format!("No domain provided through CLI"));
            std::process::exit(1);
        }
    };

    let base_domain = match Url::parse(domain_url){
        Ok(a) => a,
        Err(_) => {
            match Url::parse(
                &("https://".to_string() + domain_url)
            ) {
                Ok(a) => a,
                Err(e) => {
                    log(LogType::LogCrit, format!("Failed to parse URL: {}", e));
                    std::process::exit(1);
                }
            }
        }
    };

    // see if we are doing active/passive operations
    if matches.is_present("passive") {
        log(LogType::LogWarn, format!("This feature is experimental, and most features do not work"));
        passive::passive_test(base_domain).await;
    } else if matches.is_present("spider") {
        let links = spider::spider(base_domain).await;
    } else {
        // define our base domain
        println!("{}", base_domain);
    
        // actively find the domains
        let subdomains = active::run_active(base_domain).await;
        println!("Subdomains found:");
        for s in subdomains {
            println!("{}", s.host_str().unwrap());
        }
    }
}
