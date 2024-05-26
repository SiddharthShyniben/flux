extern crate reqwest;

pub fn fetch_feed(urls: Vec<String>) -> Result<Vec<String>, reqwest::Error> {
    let mut results: Vec<String> = vec![];

    for url in urls {
        let res = reqwest::blocking::get(url)?
            .text()?;
            
        results.push(res);
    }

    Ok(results)
}
