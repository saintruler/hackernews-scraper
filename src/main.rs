use select::document::Document;
use select::predicate::{Name, And, Class, Attr, Descendant, Any, Predicate};
use select::node::{Node, Attrs};

use reqwest;

struct Post {
    id: i32,
    title: String,
    source: String,
    points: i32,
    author: String,
    comments: i32
}

// fn getAttribute(attrs: Attrs, name: String) -> Option<String> {
//
// }

fn getPage(url: &str) -> Option<String> {
    match reqwest::blocking::get(url) {
        Ok(res) => match res.text() {
            Ok(text) => return Some(text),
            Err(_) => return None
        },
        Err(_) => return None
    }
}

fn main() {
    let page = getPage("https://news.ycombinator.com/"); 
    let page = match page {
        Some(ref text) => text.as_str(),
        None => panic!("Returned page is empty)")
    };

    let doc = Document::from(page);
    let table = doc.find(Class("itemlist").descendant(Name("tbody")))
        .next()
        .unwrap();

    let mut nodes: Vec<Node> = Vec::new();
    for (idx, node) in table.children().enumerate() {
        if node.is(Class("morespace")) { 
            break; 
        }
        else if !node.is(Class("spacer")) && node.is(Name("tr")) {
            nodes.push(node);
        }
    }

    for node in nodes.windows(2) {
        let link = node[0].find(Class("storylink")).next().unwrap();
        let title = link.text();
        println!("{}", link.html());
        println!("{}", title);
    }
}
