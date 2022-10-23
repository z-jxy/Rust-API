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



