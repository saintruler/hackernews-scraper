use select::document::Document;
use select::predicate::{Name, Class, Predicate};
use select::node::{Node, Attrs};

use reqwest;

struct Post {
    id: i32,
    title: String,
    source: String,
    score: i32,
    author: String,
}

impl Post {
    pub fn to_string(&self) -> String {
        return [
            "Post {",
            format!("  id: {}", self.id).as_str(),
            format!("  title: {}", self.title).as_str(),
            format!("  source: {}", self.source).as_str(),
            format!("  score: {}", self.score).as_str(),
            format!("  author: {}", self.author).as_str(),
            "}"
        ].join("\n");
    }
}

fn get_attribute(attrs: Attrs, name: &str) -> Option<String> {
    let name = String::from(name);
    for (key, value) in attrs {
        if String::from(key) == name {
            return Some(String::from(value));
        }
    }
    return None;
}

fn get_page(url: &str) -> Option<String> {
    match reqwest::blocking::get(url) {
        Ok(res) => match res.text() {
            Ok(text) => return Some(text),
            Err(_) => return None
        },
        Err(_) => return None
    }
}

fn parse_hackernews(document: Document) -> Vec<Post> {
    let table = document.find(Class("itemlist").descendant(Name("tbody")))
        .next()
        .unwrap();

    let mut nodes: Vec<Node> = Vec::new();
    for node in table.children() {
        if node.is(Class("morespace")) { 
            break; 
        }
        else if !node.is(Class("spacer")) && node.is(Name("tr")) {
            nodes.push(node);
        }
    }

    println!("jopa1");
    let mut posts: Vec<Post> = Vec::new();
    for node in nodes.windows(2) {
        let link = node[0].find(Class("storylink")).next();
        let link = match link {
            Some(node) => node,
            None => continue
        };

        let id = get_attribute(node[0].attrs(), "id");
        let id = match id {
            Some(id) => {
                match id.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => continue
                }
            },
            None => continue
        };
        let title = link.text();
        let source = get_attribute(link.attrs(), "href");
        let source = match source {
            Some(href) => href,
            None => continue
        };


        let subscript = node[1].find(Class("subtext")).next();
        let subscript = match subscript {
            Some(node) => node,
            None => continue
        };

        let score = subscript.find(Class("score")).next();
        let score = match score {
            Some(node) => {
                let text = node.text();
                let n_text = text.split(' ').collect::<Vec<&str>>()[0];
                match n_text.parse::<i32>() {
                    Ok(num) => num,
                    Err(_) => continue
                }
            },
            None => continue
        };

        let author = subscript.find(Class("hnuser")).next();
        let author = match author {
            Some(user) => user.text(),
            None => continue
        };

        posts.push(Post {
            id,
            title,
            source,
            score,
            author
        });
    }


    return posts;
}

fn main() {
    let page = get_page("https://news.ycombinator.com/"); 
    let page = match page {
        Some(ref text) => text.as_str(),
        None => panic!("Returned page is empty)")
    };

    let doc = Document::from(page);
    let posts = parse_hackernews(doc);

    for post in posts {
        println!("{}", post.to_string());
    }
}
