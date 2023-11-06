extern crate spider;

use spider::tokio;
use spider::website::Website;
use std::io::Error;
use std::time::{Duration, Instant};

use http::header::{ACCEPT, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, USER_AGENT, UPGRADE_INSECURE_REQUESTS};
use http::HeaderMap;

// Tokio main is needed to use async function
#[tokio::main]
// Result<(), Error> is needed because in order for main to have ? returns in the body it also
// needs to return a Result<T, E> type
async fn main() -> Result<(), Error> {
    let mut website = Website::new("https://www.eventim.hr");

    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert(
        ACCEPT_LANGUAGE,
        "en-GB,en;q=0.9,hr;q=0.8,en-US;q=0.7,bs;q=0.6,pt;q=0.5"
            .parse()
            .unwrap(),
    );
    headers.insert(CACHE_CONTROL, "max-age=0".parse().unwrap());
    headers.insert(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert(CONNECTION, "keep-alive".parse().unwrap());
    headers.insert(UPGRADE_INSECURE_REQUESTS, "1".parse().unwrap());

    website
        .with_headers(Some(headers))
        .with_subdomains(true)
        .with_respect_robots_txt(true)
        .with_delay(150)
        .with_request_timeout(Some(Duration::new(10, 0)))
        .with_http2_prior_knowledge(false)
        .with_proxies(None);

    website.on_link_find_callback = Some(|s, ss| {
        println!("link target: {:?}", s);
        // forward link to a different destination
        (s,ss)
    });

    let start = Instant::now();
    website.crawl().await;

    let duration = start.elapsed();

    let links = website.get_links();

    for link in links {
        println!("- {:?}", link.as_ref());
    }

    println!(
        "Time elapsed in website.crawl() is: {:?} for total pages: {:?}",
        duration,
        links.len()
    );

    Ok(())
}
