use chrono::{DateTime, Utc};
use std::collections::HashMap;
use rocket::{
    serde::{ Serialize, Deserialize, json::Json
    }
};

// enums

#[derive(Debug)]
pub enum C2Tasks {
    Args(Vec<String>),
    IsArgs(String),
    CreatedAt(DateTime<Utc>),
}


// models

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct NewC2Task {
    pub task: String,
    pub args: Vec<String>,

    pub implant_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromForm)]
pub struct ListenerModel {
    pub lhost: String,
    pub lport: String,
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}