use chrono::{DateTime, Utc};
use crate::{schema::{c2_tasks}, api_models};

use diesel::{
    prelude::*,
    Queryable,
    self,
    Identifiable, query_dsl::methods::FilterDsl, associations::HasTable,
};

use rocket::{
    data::{ FromData },
    serde::{ Serialize, Deserialize, json::Json },
};

//#[table_name = "c2_tasks"]
#[derive(Debug, Queryable, Clone)]
pub struct C2TaskModel {
    pub id: i32,
    pub created_at:  DateTime<Utc>,
    pub executed_at: Option<DateTime<Utc>>,
    pub task: String,
    pub args: Json<Vec<String>>,
    pub result: Option<String>,

    pub implant_id: String,
}

impl Into<ApiModels::C2TaskModel> for C2TaskModel {
    fn into(self) -> ApiModels::C2TaskModel {
        ApiModels::C2TaskModel {
            id: self.id,
            created_at: self.created_at,
            executed_at: self.executed_at,
            command: self.command,
            args: self.args.0,
            output: self.output,
            agent_id: self.agent_id,
        }
    }
}

//#[derive(Insertable, Serialize, Deserialize, Debug, //FromForm)]
//#[table_name = "c2_tasks"]
//pub struct NewTask<'a> {
//    pub agent_id: &'a str,
//    pub agent_pid: &'a str,
//    pub agent_ip: &'a str,
//
//    pub id: i32,
//    pub created_at:  DateTime<Utc>,
//    pub executed_at: Option<DateTime<Utc>>,
//    pub task: String,
//    pub args: Json<Vec<String>>,
//    pub result: Option<String>,
//
//    pub implant_id: String,
//}