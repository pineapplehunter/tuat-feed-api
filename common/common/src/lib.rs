#![warn(missing_docs)]
//! This crate holds some common data structures for both server and client

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// data for a post on feed
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Post {
    /// the id of the information. found in the tuat feed.
    #[serde(rename = "投稿ID")]
    pub post_id: u32,
    /// タイトル
    #[serde(rename = "タイトル")]
    pub title: Option<String>,
    /// 本文
    #[serde(rename = "本文")]
    pub contents: Option<String>,
    /// 最終更新日
    #[serde(rename = "最終更新日")]
    pub updated_date: Option<String>,
    /// 公開期間
    #[serde(rename = "公開期間")]
    pub show_date: Option<(String, String)>,
    ///担当者
    #[serde(rename = "担当者")]
    pub person_in_charge: Option<String>,
    /// 発信元
    #[serde(rename = "発信元")]
    pub origin: Option<String>,
    /// カテゴリー
    #[serde(rename = "カテゴリー")]
    pub category: Option<String>,
    /// 添付ファイル
    #[serde(rename = "添付ファイル")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default = "HashMap::default")]
    pub attachment: HashMap<String, String>,
    /// その他のフィールド
    #[serde(rename = "その他")]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    #[serde(default = "HashMap::default")]
    pub other: HashMap<String, String>,
}

impl Post {
    /// creates a new `Info`
    pub fn new(id: u32) -> Self {
        Self {
            post_id: id,
            other: HashMap::new(),
            origin: None,
            person_in_charge: None,
            title: None,
            contents: None,
            updated_date: None,
            show_date: None,
            category: None,
            attachment: HashMap::new(),
        }
    }
}

/// Compatibility layer for Post
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostCompatv1 {
    #[serde(rename = "id")]
    post_id: u32,
    data: HashMap<String, String>,
}

impl From<Post> for PostCompatv1 {
    fn from(post: Post) -> Self {
        let mut post_compat = PostCompatv1 {
            post_id: post.post_id,
            data: HashMap::new(),
        };
        if let Some(title) = post.title {
            post_compat.data.insert("タイトル".to_string(), title);
        }
        if let Some(contents) = post.contents {
            post_compat.data.insert("本文".to_string(), contents);
        }
        if let Some(updated_date) = post.updated_date {
            post_compat.data.insert("最終更新日".to_string(), {
                let mut s = updated_date.replace("-", "/");
                s.push_str("(XXX)");
                s
            });
        }
        if let Some((mut start, mut end)) = post.show_date {
            post_compat.data.insert("公開期間".to_string(), {
                start = start.replace("-", "/");
                start.push_str("(XXX)");
                end = end.replace("-", "/");
                end.push_str("(XXX)");
                format!("{} 〜 {}", start, end)
            });
        }
        if let Some(person_in_charge) = post.person_in_charge {
            post_compat
                .data
                .insert("担当者".to_string(), person_in_charge);
        }
        if let Some(origin) = post.origin {
            post_compat.data.insert("発信元".to_string(), origin);
        }
        if let Some(category) = post.category {
            post_compat.data.insert("カテゴリー".to_string(), category);
        }
        let attachment_string = post
            .attachment
            .iter()
            .map(|(name, url)| format!("[{}]({})", name, url))
            .collect::<Vec<String>>()
            .join("\n");
        if !attachment_string.is_empty() {
            post_compat
                .data
                .insert("添付ファイル".to_string(), attachment_string);
        }
        // その他
        for (k, v) in post.other {
            post_compat.data.insert(k, v);
        }
        post_compat
    }
}
