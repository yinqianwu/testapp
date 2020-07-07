
// pub fn find_astocks_info_by_id(conn: &PgConnection, id: &u32) -> Option<Article> {
//     println!("db find_article_by_slug slug: {}", slug);
//     let article = articles::table
//         .filter(articles::slug.eq(slug))
//         .first::<Article>(conn)
//         .map_err(|err| eprintln!("articles::find_one: {}", err))
//         .ok()?;
//
//     Some(article)
// }
