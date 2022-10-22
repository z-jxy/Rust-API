use r2d2::{
    ManageConnection,
    self,
};

use rocket_sync_db_pools::{
    diesel::{connection, r2d2::ConnectionManager},
};

use rocket::{
    data::{ FromData },
    http::{ Status },
    request::{ self, FromRequest },
    fairing::{ AdHoc },
    response::{ Debug, status::Created },
    serde::{ Serialize, Deserialize, json::Json },
    form::{ Form },
    outcome::{Outcome},
    Rocket,
    Build,
    Error,
    FromForm,
    Request,
    State,
};

use std::{
    env,
    collections::HashMap,
};
use diesel::{
    prelude::*,
    Queryable,
    self,
    Identifiable, query_dsl::methods::FilterDsl, associations::HasTable,
};

use crate::schema::{agents, self};


#[database("postgres")]
struct Db(rocket_sync_db_pools::diesel::PgConnection);

pub fn establish_conn() -> PgConnection {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL MUST BE SET!!");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}",  database_url))
}


// this is to get users from the database
#[derive(Serialize, Clone, Deserialize, Debug, Queryable, FromForm, Identifiable)]
#[serde(crate = "rocket::serde")] 
pub struct Agent {
    pub id: i32,
    pub agent_id: String,
    pub agent_pid: String,
    pub agent_ip: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, FromForm, Clone)]
#[table_name = "agents"]
pub struct InsertableAgent {
    pub agent_id: String,
    pub agent_pid: String,
    pub agent_ip: String,
}

impl InsertableAgent {
    fn from_agent(agent: Agent) -> InsertableAgent {
        InsertableAgent {
            agent_id: agent.agent_id,
            agent_pid: agent.agent_pid,
            agent_ip: agent.agent_ip,
        }
    }
}



#[derive(Debug, Queryable, AsChangeset, FromForm, Serialize, Deserialize, Identifiable)]
#[table_name = "agents"]
pub struct AgentModel {
    pub id: i32,
    pub agent_id:  String,
    pub agent_pid: String,
    pub agent_ip: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug, FromForm)]
#[table_name = "agents"]
pub struct NewAgent<'a> {
    pub agent_id: &'a str,
    pub agent_pid: &'a str,
    pub agent_ip: &'a str,
}


impl Agent {
    pub fn create(agent: Agent, connection: &mut PgConnection) -> QueryResult<Agent> {
        let encrypted_agent = Agent {
            ..agent
        };
        diesel::insert_into(agents::table)
        .values(&InsertableAgent::from_agent(encrypted_agent))
        .execute(connection)?;

        agents::table.order(agents::id.desc()).first(connection)
    }

    //pub fn get_by_agent_id_and_pid(agent_id_: String, password_: String, connection: &//mut PgConnection) -> Option<Agent> {
    //    let res = agents::table
    //        .filter(agents::agent_id.eq(agent_id_))
    //        .get_result::<Agent>(connection);
    //    match res {
    //        Ok(agent) => {
    //                    return Some(agent)
    //                }
    //        Err(_) => {
    //            None
    //        }
    //    }
    //}
//
    //pub fn get_by_agent_id(agent_id_: String, connection: &mut PgConnection) -> //Option<Agent> {
    //    let res: Result<Agent, Error> = agents::table
    //        .filter(agents::agent_id.eq(agent_id_))
    //        .get_result::<Agent>(connection);
    //    match res {
    //        Ok(agent) => {
    //                    return Some(agent)
    //                }
    //        Err(_) => {
    //            None
    //        }
    //    }
    //}
}


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/test")]
fn test() -> &'static str {
    "Hello, agent!"
}

#[get("/checking")]
fn apply() -> Json<InsertableAgent> {
    let agent_id = "HAHAHA".to_string();
    let agent_pid = "4123".to_string();
    let agent_ip = "127.0.0.1".to_string();

    Json(InsertableAgent { agent_id, agent_pid, agent_ip })
}


#[post("/submitting", data="<new_agent>")]
fn test_reg(new_agent: Form<InsertableAgent>) -> Json<InsertableAgent> {
    let _res = new_agent.into_inner();
    Json(
        InsertableAgent { 
            agent_id: (_res.agent_id), 
            agent_pid: (_res.agent_pid), 
            agent_ip: (_res.agent_ip),
        }
    )
}

// VIEW AGENTS

#[get("/view-agents")]
fn get_agents() -> Json<Vec<AgentModel>> {
    use crate::schema::agents::dsl::*;
    let connection: &mut PgConnection = &mut establish_conn();

    let results: Json<Vec<AgentModel>> = agents.load::<AgentModel>(connection)
        .map(Json)
        .expect("Error loading agents");

    println!("[+] SUCCESS \nFound {} agents. \n", results.len());

    return Json(results.into_inner());

}

//LOG AGENTS TO DB

#[post("/test-submit", data="<db_agent>")]
fn test_db_log(db_agent: Form<InsertableAgent>) -> Json<InsertableAgent> {
    use crate::schema::agents::dsl::*;

    // connecting to db
    let connection: &mut PgConnection = &mut establish_conn();


    let _new_agent = NewAgent {
        agent_id: &db_agent.agent_id,
        agent_pid: &db_agent.agent_pid,
        agent_ip: &db_agent.agent_ip,
    };
    println!("agent data parsed!");
    
    //TODO: Add better error handling
    diesel::insert_into(agents)
    .values(&_new_agent)
    .execute(connection)
        .map(Json)
        .map_err(|_| Status::InternalServerError)
        .ok();

    println!("logged in db!!!!!");

    Json(
        InsertableAgent { 
            agent_id: ((*db_agent.agent_id)).to_string(), 
            agent_pid: (*db_agent.agent_pid).to_string(), 
            agent_ip: (*db_agent.agent_ip).to_string(),
        }
    )
}



#[delete("/remove-agent/<target_id>")]
fn remove_agent(target_id: String) {
    
    let agent_id_: String = target_id;
    
    use crate::schema::agents::dsl::*;
    // connecting to db
    
    println!("agent data parsed!");
  
    let connection: &mut PgConnection = &mut establish_conn();
    let agent_deleted = diesel::delete(agents)
    .filter(agent_id.like(&agent_id_))
    .execute(connection)
    .expect("ERrrror!");

    //Ok(())
    
    println!("agent removed: {:#?}", agent_deleted);

    //format!("Successfully removed agent: {:#?} from db.", target_id)
}






pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket| async {
        rocket.attach(Db::fairing())
            //.attach(AdHoc::try_on_ignite("Diesel Migrations", run_migrations))
            .mount("/api", routes![
                //list, 
                //create, 
                //read, 
                //delete, 
                //destroy
                index,
                test,
                apply,
                //create_agent,
                test_reg,
                test_db_log,
                get_agents,
                remove_agent,
            ]
        )
    })
}
