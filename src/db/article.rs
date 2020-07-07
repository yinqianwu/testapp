use crate::db::OffsetLimit;
use crate::models::article::{ArticleJson, Article, to_article_json};
use crate::common::schema::articles;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use slug;
use rocket_contrib::json::JsonValue;

const SUFFIX_LEN: usize = 6;
const DEFAULT_LIMIT: i64 = 20;

// #[derive(Insertable)]
// #[table_name = "articles"]
// struct NewArticle<'a> {
//     title: &'a str,
//     description: &'a str,
//     body: &'a str,
//     slug: &'a str,
//     predict_result: i32,
//     event_tag_list: &'a Vec<String>,
// }

// pub fn create(
//     conn: &PgConnection,
//     predict_result: i32,
//     title: &str,
//     description: &str,
//     body: &str,
//     event_tag_list: &Vec<String>,
// ) -> ArticleJson {
//
//     let new_article = &NewArticle {
//         title,
//         description,
//         body,
//         predict_result,
//         event_tag_list,
//         slug: &slugify(title),
//     };
//
//     diesel::insert_into(articles::table)
//         .values(new_article)
//         .get_result::<Article>(conn)
//         .expect("Error creating article")
//         .attach()
// }

fn slugify(title: &str) -> String {
    if cfg!(feature = "random-suffix") {
        format!("{}-{}", slug::slugify(title), generate_suffix(SUFFIX_LEN))
    } else {
        slug::slugify(title)
    }
}

fn generate_suffix(len: usize) -> String {
    let mut rng = thread_rng();
    (0..len).map(|_| rng.sample(Alphanumeric)).collect()
}

//
// #[derive(FromForm, Default)]
// pub struct FindArticles {
//     tag: Option<String>,
//     predict_result: Option<i32>,
//     limit: Option<i64>,
//     offset: Option<i64>,
// }


pub fn find_article_by_slug(conn: &PgConnection, slug: &str) -> Option<Article> {
    println!("db find_article_by_slug slug: {}", slug);
    let article = articles::table
        .filter(articles::slug.eq(slug))
        .first::<Article>(conn)
        .map_err(|err| eprintln!("articles::find_one: {}", err))
        .ok()?;

    Some(article)
}

// pub fn find_article_by_slug(conn: &PgConnection, slug: &str) -> JsonValue {
//     println!("db find_article_by_slug slug: {}", slug);
//     let article = articles::table
//         .filter(articles::slug.eq(slug))
//         .first::<article::Article>(conn)
//         .map_err(|err| eprintln!("articles::find_article_by_slug: {}", err))
//         .ok()?;
//
//     json!(article::to_article_json(article))
// }

pub fn find_latest_articles(conn: &PgConnection, limit: i32) -> Option<Vec<Article>> {
    println!("db find_latest_articles");
    let articles = articles::table
        .order(articles::created_at)
        .load::<Article>(conn)
        .map_err(|err| eprintln!("articles::find_latest_articles: {}", err))
        .ok()?;

    println!("after db find_latest_articles");
    Some(articles)
}

// fn to_json(article: Article) -> ArticleJson {
//     article::to_article_json()
// }

// pub fn find(conn: &PgConnection, params: &FindArticles) -> (Vec<ArticleJson>, i64) {
//     let mut query = articles::table
//         .select((
//             articles::all_columns
//         ))
//         .into_boxed();
//
//     if let Some(ref predict_result) = params.predict_result {
//         query = query.filter(articles::predict_result.eq(predict_result))
//     }
//     if let Some(ref tag) = params.tag {
//         let result = articles::event_tag_list
//             .contains(vec![tag])
//             .get_result::<Article>(conn);
//
//         match result {
//             Ok(id) => {
//                 query = query.filter(diesel::dsl::sql(&format!(
//                     "article.id IN (SELECT favorites.article FROM favorites WHERE favorites.user = {})",
//                     id
//                 )));
//             }
//             Err(err) => match err {
//                 diesel::result::Error::NotFound => return (vec![], 0),
//                 _ => panic!("Cannot load favorited user: {}", err),
//             },
//         }
//     }
//
//     query
//         .offset_and_limit(
//             params.offset.unwrap_or(0),
//             params.limit.unwrap_or(DEFAULT_LIMIT),
//         )
//         .load_and_count::<(Article)>(conn)
//         .map(|(res, count)| {
//             (
//                 res.into_iter()
//                     .map(|(article)| article)
//                     .collect(),
//                 count,
//             )
//         })
//         .expect("Cannot load article")
// }
//
// pub fn find_one(conn: &PgConnection, slug: &str, user_id: Option<i32>) -> Option<ArticleJson> {
//     let article = article::table
//         .filter(article::slug.eq(slug))
//         .first::<Article>(conn)
//         .map_err(|err| eprintln!("article::find_one: {}", err))
//         .ok()?;
//
//     let favorited = user_id
//         .map(|id| is_favorite(conn, &article, id))
//         .unwrap_or(false);
//
//     Some(populate(conn, article, favorited))
// }
//
// #[derive(FromForm, Default)]
// pub struct FeedArticles {
//     limit: Option<i64>,
//     offset: Option<i64>,
// }
//
// // select * from article where author in (select followed from follows where follower = 7);
// pub fn feed(conn: &PgConnection, params: &FeedArticles, user_id: i32) -> Vec<ArticleJson> {
//     article::table
//         .filter(
//             article::author.eq_any(
//                 follows::table
//                     .select(follows::followed)
//                     .filter(follows::follower.eq(user_id)),
//             ),
//         )
//         .inner_join(users::table)
//         .left_join(
//             favorites::table.on(article::id
//                 .eq(favorites::article)
//                 .and(favorites::user.eq(user_id))),
//         )
//         .select((
//             article::all_columns,
//             users::all_columns,
//             favorites::user.nullable().is_not_null(),
//         ))
//         .limit(params.limit.unwrap_or(DEFAULT_LIMIT))
//         .offset(params.offset.unwrap_or(0))
//         .load::<(Article, User, bool)>(conn)
//         .expect("Cannot load feed")
//         .into_iter()
//         .map(|(article, author, favorited)| article.attach(author, favorited))
//         .collect()
// }
//
// pub fn favorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<ArticleJson> {
//     conn.transaction::<_, diesel::result::Error, _>(|| {
//         let article = diesel::update(article::table.filter(article::slug.eq(slug)))
//             .set(article::favorites_count.eq(article::favorites_count + 1))
//             .get_result::<Article>(conn)?;
//
//         diesel::insert_into(favorites::table)
//             .values((
//                 favorites::user.eq(user_id),
//                 favorites::article.eq(article.id),
//             ))
//             .execute(conn)?;
//
//         Ok(populate(conn, article, true))
//     })
//     .map_err(|err| eprintln!("article::favorite: {}", err))
//     .ok()
// }
//
// pub fn unfavorite(conn: &PgConnection, slug: &str, user_id: i32) -> Option<ArticleJson> {
//     conn.transaction::<_, diesel::result::Error, _>(|| {
//         let article = diesel::update(article::table.filter(article::slug.eq(slug)))
//             .set(article::favorites_count.eq(article::favorites_count - 1))
//             .get_result::<Article>(conn)?;
//
//         diesel::delete(favorites::table.find((user_id, article.id))).execute(conn)?;
//
//         Ok(populate(conn, article, false))
//     })
//     .map_err(|err| eprintln!("article::unfavorite: {}", err))
//     .ok()
// }
//
// #[derive(Deserialize, AsChangeset, Default, Clone)]
// #[table_name = "articles"]
// pub struct UpdateArticleData {
//     title: Option<String>,
//     description: Option<String>,
//     body: Option<String>,
//     #[serde(skip)]
//     slug: Option<String>,
//     #[serde(rename = "tagList")]
//     event_tag_list: Vec<String>,
// }
//
// pub fn update(
//     conn: &PgConnection,
//     slug: &str,
//     user_id: i32,
//     mut data: UpdateArticleData,
// ) -> Option<ArticleJson> {
//     if let Some(ref title) = data.title {
//         data.slug = Some(slugify(&title));
//     }
//     // TODO: check for not_found
//     let article = diesel::update(article::table.filter(article::slug.eq(slug)))
//         .set(&data)
//         .get_result(conn)
//         .expect("Error loading article");
//
//     let favorited = is_favorite(conn, &article, user_id);
//     Some(populate(conn, article, favorited))
// }
//
// pub fn delete(conn: &PgConnection, slug: &str, user_id: i32) {
//     let result = diesel::delete(
//         article::table.filter(article::slug.eq(slug).and(article::author.eq(user_id))),
//     )
//     .execute(conn);
//     if let Err(err) = result {
//         eprintln!("article::delete: {}", err);
//     }
// }
//
// fn is_favorite(conn: &PgConnection, article: &Article, user_id: i32) -> bool {
//     use diesel::dsl::exists;
//     use diesel::select;
//
//     select(exists(favorites::table.find((user_id, article.id))))
//         .get_result(conn)
//         .expect("Error loading favorited")
// }
//
// fn populate(conn: &PgConnection, article: Article, favorited: bool) -> ArticleJson {
//     let author = users::table
//         .find(article.author)
//         .get_result::<User>(conn)
//         .expect("Error loading author");
//
//     article.attach(author, favorited)
// }
//
// pub fn tags(conn: &PgConnection) -> Vec<String> {
//     article::table
//         .select(diesel::dsl::sql("distinct unnest(event_tag_list)"))
//         .load::<String>(conn)
//         .expect("Cannot load tags")
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_generate_suffix() {
//         for len in 3..9 {
//             assert_eq!(generate_suffix(len).len(), len);
//         }
//     }
// }
