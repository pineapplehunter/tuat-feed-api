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
    pub title: String,
    /// 本文
    #[serde(rename = "本文")]
    pub contents: String,
    /// 最終更新日
    #[serde(rename = "最終更新日")]
    pub updated_date: String,
    /// 公開期間
    #[serde(rename = "公開期間")]
    pub show_date: (String, String),
    ///担当者
    #[serde(rename = "担当者")]
    pub person_in_charge: String,
    /// 発信元
    #[serde(rename = "発信元")]
    pub origin: String,
    /// カテゴリー
    #[serde(rename = "カテゴリー")]
    pub category: String,
    /// 対象
    #[serde(rename = "対象")]
    pub target: String,
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
    /// creates a new post instance
    pub fn new(post_id: u32) -> Self {
        Self {
            post_id,
            title: String::new(),
            contents: String::new(),
            updated_date: String::new(),
            show_date: (String::new(), String::new()),
            person_in_charge: String::new(),
            origin: String::new(),
            category: String::new(),
            target: String::new(),
            attachment: HashMap::new(),
            other: HashMap::new(),
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
        post_compat.data.insert("タイトル".to_string(), post.title);
        post_compat.data.insert("本文".to_string(), post.contents);
        post_compat
            .data
            .insert("最終更新日".to_string(), post.updated_date);
        post_compat.data.insert("公開期間".to_string(), {
            format!("{} 〜 {}", post.show_date.0, post.show_date.1)
        });
        post_compat
            .data
            .insert("担当者".to_string(), post.person_in_charge);
        post_compat.data.insert("発信元".to_string(), post.origin);
        post_compat
            .data
            .insert("カテゴリー".to_string(), post.category);
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
