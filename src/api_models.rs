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



//#[derive(Debug)]
//pub struct NewListener<'a, T: ?Sized> {
//    pub lhost: &'a String,
//    pub lport: &'a String,
//}
//
//impl<'a, T: ?Sized> From<&'a T> for NewListener<'a, T> {
//    fn from(s: &'a T) -> Self {
//        NewListener { lhost: (), lport: () }
//    }
//}



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


//#[derive(Debug, FromForm)]
//pub struct ListenerModel<'a, T: ?Sized>(&'a T);
//
//impl<'a, T: ?Sized> From<&'a T> for ListenerModel<'a, T> {
//    fn from(s: &'a T) -> Self {
//        ListenerModel(s)
//    }
//}