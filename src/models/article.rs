use chrono::{DateTime, Utc};
use serde::Serialize;
use rocket_contrib::json::JsonValue;
use crate::common::market_types::{MarketType};
use crate::common::const_types;
use crate::common::event_types::EventType;
use crate::common::section_types::SectionType;
use crate::common::status_type::StatusType;

#[derive(Queryable)]
pub struct Article {
    pub id: String,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub market_type: i32,
    pub media_source: String,
    pub section_tag_list: Vec<i32>,
    pub event_tag_list: Vec<i32>,
    pub stakeholder_list: Vec<String>,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Default)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleJson {
    pub status: u32,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub media_source: String,
    pub market_type: String,
    pub section_tag_list: Vec<String>,
    pub event_tag_list: Vec<String>,
    pub stakeholder_list: Vec<String>,
    pub occurred_at: String,
}

#[derive(Default)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticlesJson {
    pub status: u32,
    pub articles: Vec<ArticleJson>,
}

pub fn to_article_json(art: Option<Article>) -> JsonValue {

    let mut status = StatusType::Success;
    match art {
        Some(art) => {

            println!("to_articles_json slug: {} title: {} ",art.slug, art.title);
            let article_json = ArticleJson {
                status: status as u32,
                title: art.title,
                slug: art.slug,
                description: art.description,
                content: art.body,
                media_source: art.media_source,
                market_type: MarketType::from_u32(art.market_type as u32).to_string(),
                section_tag_list: SectionType::vi32_to_vstring(art.section_tag_list),
                event_tag_list: EventType::vi32_to_vstring(art.event_tag_list),
                stakeholder_list: art.stakeholder_list,
                occurred_at: art.occurred_at.format(const_types::DATE_FORMAT).to_string(),
            };
            json!(article_json)
        }
        None => {
            status = StatusType::ArticleNotFoundError;
            let article_json = ArticleJson { status: status as u32, ..Default::default() };
            json!(article_json)
        }
    }
}

pub fn to_articles_json(arts: Option<Vec<Article>>) -> JsonValue {
    println!("to_articles_json");

    let mut all_status = StatusType::Success;

    match arts {
        Some(arts) => {
            let mut v: Vec<ArticleJson> = Vec::new();
            for art in arts {
                println!("to_articles_json slug: {} title: {} ",art.slug, art.title);

                let mut status = StatusType::Success;

                let article_json = ArticleJson {
                    status: status as u32,
                    title: art.title,
                    slug: art.slug,
                    description: art.description,
                    media_source: art.media_source,
                    content: art.body,
                    market_type: MarketType::from_u32(art.market_type as u32).to_string(),
                    section_tag_list: SectionType::vi32_to_vstring(art.section_tag_list),
                    event_tag_list: EventType::vi32_to_vstring(art.event_tag_list),
                    stakeholder_list: art.stakeholder_list,
                    occurred_at: art.occurred_at.format(const_types::DATE_FORMAT).to_string(),
                };
                v.push(article_json);
            }

            println!("after iter");

            let articles_json = ArticlesJson {
                status: all_status as u32,
                articles: v,
            };
            println!("json {:?}", json!(articles_json));
            json!(articles_json)

        }
        None => {
            //TODO
            let mut v: Vec<ArticleJson> = Vec::new();
            all_status = StatusType::ArticleNotFoundError;
            let articles_json = ArticlesJson {
                status: all_status as u32,
                articles: v,
            };
            json!(articles_json)
        }
    }
}


