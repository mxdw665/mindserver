use curl::easy::Easy;
use log::{error, info};
use std::{fs::File, io::Write};

pub fn download(url: &str, save: &str) {
    info!("Downloading of {url}");
    println!("Downloading of {url}");
    let mut easy = Easy::new();
    let change_url = url.replace('\"', "");
    let file = format!("{save}/server.jar");
    let mut buf = File::create(file).unwrap();
    easy.useragent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36").unwrap();
    easy.url(&change_url).unwrap();
    easy.write_function(move |data| {
        buf.write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    error!("URL using bad/illegal format or missing URL: {}", url);
    easy.perform().unwrap();

    let correct_code: u32 = "200".parse().unwrap();
    if easy.response_code().unwrap() != correct_code {
        error!("Download of {url} failed");
        panic!("Download of {url} failed");
    }
}
