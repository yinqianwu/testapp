use rocket::response::NamedFile;
use rocket_contrib::json::{JsonValue};
use crate::db;
use crate::db::article::{find_latest_articles};
use crate::models::article;

use std::path::{Path, PathBuf};
use std::fs::{File, read_dir};
use std::io::prelude::*;

#[get("/")]
pub fn index() -> Option<NamedFile> {
    println!("pub fn index");
    NamedFile::open(Path::new("web/dist/index.html")).ok()
}

#[allow(unused_variables)]
#[get("/article/<name>")]
pub fn get_article(name: String) -> Option<NamedFile> {
    println!("pub fn article");
    NamedFile::open(Path::new("web/dist/index.html")).ok()
}

#[get("/api/articles/<slug>")]
pub fn api_get_article_by_slug(slug: String, conn: db::Conn) -> JsonValue {
    println!("routes find_article_by_slug ");
    article::to_article_json(db::article::find_article_by_slug(&conn, &slug))
}


#[get("/api/articles")]
pub fn api_get_all_articles(conn: db::Conn) -> JsonValue {

    let arts = article::to_articles_json(find_latest_articles(&conn, 1));

    arts
    // let artic = read_dir(Path::new("pages/article/")).unwrap().filter_map(|entry| {
    //     entry.ok().and_then(|e| {
    //         e.path().file_name().and_then(|n| n.to_str().map(|s| String::from(s)))
    //     })
    // }).collect::<Vec<String>>();;
    //
    // json!({
    //     "files": files
    // })
}

// #[get("/api/articles/id/<id>")]
// pub fn api_get_article_by_id(id: String) -> JsonValue {
//
//     println!("pub fn api_article");
//
//     // /* Check if the article exists */
//     // if Path::new(&format!("pages/article/{}", name)).exists() {
//     //     let mut article_content = String::new();
//     //
//     //     match File::open(format!("pages/article/{}/content.md", name)) {
//     //         Err(why) => return json!({
//     //             "status": "error",
//     //             "content": format!("Could not open /pages/article/{}/content.md, details: {}", name, why.to_string())
//     //         }),
//     //
//     //         Ok(mut file) => {
//     //             match file.read_to_string(&mut article_content) {
//     //                 Err(why) => return json!({
//     //                     "status": "error",
//     //                     "content": format!("Could read the file /pages/article/{}/content.md as string, details: {}", name, why.to_string())
//     //                 }),
//     //                 Ok(_) => {}
//     //             }
//     //         }
//     //     };
//     //
//     //     let have_image = Path::new(&format!("pages/article/{}/image.jpg", name)).exists();
//     //
//     //     json!({
//     //         "status": "ok",
//     //         "title": name,
//     //         "content": article_content,
//     //         "haveImage": have_image
//     //     })
//     // } else {
//     //     json!({
//     //         "status": "error",
//     //         "content": format!("Could not open /pages/article/{}/", name)
//     //     })
//     // }
//
//     json!({
//             "status": "ok",
//             "title": id,
//             "content": "123",
//             "haveImage": false
//         })
// }


// #[get("/api/page/<name>")]
// pub fn api_page(name: String) -> JsonValue {
//     if Path::new(&format!("pages/{}.md", name)).exists() {
//         let mut page_content = String::new();
//
//         match File::open(format!("pages/{}.md", name)) {
//             Err(why) => return json!({
//                 "status": "error",
//                 "content": format!("Could not open pages/{}.md, details: {}", name, why.to_string())
//             }),
//
//             Ok(mut file) => {
//                 match file.read_to_string(&mut page_content) {
//                     Err(why) => return json!({
//                         "status": "error",
//                         "content": format!("Could read the file pages/{}.md as string, details: {}", name, why.to_string())
//                     }),
//                     Ok(_) => {}
//                 }
//             }
//         };
//
//         json!({
//             "status": "ok",
//             "title": name,
//             "content": page_content
//         })
//     } else {
//         json!({
//             "status": "error",
//             "content": format!("Could not open pages/{}.md", name)
//         })
//     }
// }