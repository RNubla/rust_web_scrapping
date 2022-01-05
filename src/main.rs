use clap::{App, Arg};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Or, Predicate};
fn main() {
    let matches = App::new("StackOverflow Scraper")
        .version("0.1.0")
        .author("Robert Nubla")
        .about("This will scrape question from stackoverflow and this project is based on https://github.com/chaudharypraveen98/stackoverflow-scraping-with-rust")
        .arg(
            Arg::new("tag")
                .short('t')
                .long("tag")
                .takes_value(true)
                .help("The tag to search for"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .takes_value(true)
                .help("give n count of posts"),
        )
        .arg(
            Arg::new("query")
                .short('q')
                .long("query")
                .takes_value(true)
                .help("The query to search for"),
        )
        .get_matches();

    if matches.is_present("tag") && matches.is_present("count") {
        let url = format!(
            "https://stackoverflow.com/questions/tagged/{}?tab=Votes",
            matches.value_of("tag").unwrap()
        );
        let count: i32 = matches.value_of("count").unwrap().parse::<i32>().unwrap();
        news(&url, count as usize);
    } else if matches.is_present("tag") {
        let url = format!(
            "https://stackoverflow.com/questions/tagged/{}?tab=Votes",
            matches.value_of("tag").unwrap()
        );
        news(&url, 12);
    } else if matches.is_present("query") {
        let url = format!(
            "https://stackoverflow.com/search?q={}",
            matches.value_of("query").unwrap()
        );
        news(&url, 12);
    }
}

#[tokio::main]
// async fn search_result(ur: &str, count: usize) -> Result<(), reqwest::Error> {
//     let resp = reqwest::get(ur).await?;
//     let document = Document::from(&*resp.text().await?);

//     for node in document.find(Class("mln24")).take(count) {
//         let question = node.find(Class("excerpt")).next().unwrap().text();
//     }

//     Ok(());
// }
async fn news(url: &str, count: usize) -> Result<(), reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let document = Document::from(&*resp.text().await?);

    for node in document.find(Class("mln24")).take(count) {
        let question = node.find(Class("excerpt")).next().unwrap().text();
        // println!("{}", &question);
        let title_element = node.find(Class("question-hyperlink")).next().unwrap();
        let title = title_element.text();
        let question_link = title_element.attr("href").unwrap();
        let votes = node.find(Class("vote-count-post")).next().unwrap().text();
        let views = node.find(Class("views")).next().unwrap().text();
        let striped_views = views.trim();
        let tags = node
            .find(Attr("class", "post-tag grid--cell"))
            .map(|tag| tag.text())
            .collect::<Vec<_>>();
        let answer = node
            .find(Or(
                Attr("class", "status answered-accepted").descendant(Name("strong")),
                Attr("class", "status answered").descendant(Name("strong")),
            ))
            .next()
            .unwrap()
            .text();
        println!("Question       => {}", question);
        println!(
            "Question-link  => https://stackoverflow.com{}",
            question_link
        );
        println!("Question-title => {}", title);
        println!("Votes          => {}", votes);
        println!("Views          => {}", striped_views);
        println!("Tags           => {}", tags.join(" ,"));
        println!("Answers        => {}", answer);
        println!("-------------------------------------------------------------\n");
    }
    Ok(())
}
