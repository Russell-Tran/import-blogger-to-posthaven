extern crate minidom;
//extern crate xml-rs;

use minidom::Element;
//use xml::reader::EventReader;
use std::fs;
//use std::fs::File;

// const XHTML_NS: &'static str = "http://www.w3.org/1999/xhtml";

// const DATA: &'static str = r#"<DocumentElement param="value" xmlns='http://www.w3.org/2005/Atom'>
// <FirstElement>
//     Some Text
// </FirstElement>
// <SecondElement param2="something">
//     Pre-Text <Inline>Inlined text</Inline> Post-text.
// </SecondElement>
// </DocumentElement>"#;

// #[derive(Debug)]
// pub struct Article {
//     updated: String,
//     title: String,
//     content: String,
// }

const DATA: &'static str = r#"<DocumentElement param="value" xmlns='http://www.w3.org/2005/Atom'>
<FirstElement>
    Some Text
</FirstElement>
<SecondElement param2="something">
    Pre-Text <Inline>Inlined text</Inline> Post-text.
</SecondElement>
</DocumentElement>"#;

fn main() {

    let data = fs::read_to_string("examples/something.xml").expect("Unable to read file");
    let root: minidom::Element = data.parse().unwrap();
    println!("{:#?}", root);

    // let mut f = File::open("examples/something.xml");
    // let parser = xml::reader::EventReader::new(f);
    // for event in parser {
    //     println!("{:?}", event.unwrap());
    // }
    // let data = fs::read_to_string("examples/something.xml").expect("Unable to read file");
    // let root: minidom::Element = data.parse().unwrap();
    // println!("{:#?}", root);

    // //let data = fs::read_to_string("/Users/russelltran/Downloads/your_blogger_blog.xml").expect("Unable to read file");
    // let data = fs::read_to_string("examples/something.xml").expect("Unable to read file");
    // let root: Element = DATA.parse().unwrap();

    // let mut articles: Vec<Article> = Vec::new();

    // for child in root.children() {
    //     if child.is("entry", XHTML_NS) {
    //         let updated = child.get_child("updated", XHTML_NS).unwrap().text();
    //         let title = child.get_child("title", XHTML_NS).unwrap().text();
    //         let content = child.get_child("content", XHTML_NS).unwrap().text();
    //         articles.push(Article {
    //             updated: updated,
    //             title: title,
    //             content: content,
    //         });
    //     }
    // }

    // println!("{:?}", articles);
}
