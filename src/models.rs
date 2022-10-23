use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use rocket::serde::{Serialize};
use crate::models::diesel::Queryable;
//use crate::
//use crate::schema::users::dsl::users as all_users;

// this is to get users from the database
#[derive(Serialize, Queryable)] 
pub struct Agent {
    pub id: i32,
    pub agent_id: String,
    pub agent_pid: String,
    pub agent_ip: String,
}

#[derive(Insertable)]
#[table_name = "agents"]
pub struct InsertableAgent {
    pub agent_id: String,
    pub agent_pid: String,
    pub agent_ip: String
}

impl InsertableAgent {
    fn from_agent(agent: Agent) -> InsertableAgent {
        InsertableAgent {
            agent_id: agent.id,
            agent_pid: agent.pid,
            agent_ip: agent.ip,
        }
    }
}

impl Agent {
    pub fn create(agent: Agent, connection: &PgConnection) -> QueryResult<Agent> {
        let encrypted_agent = Agent {
            pid: hash(agent.pid,DEFAULT_COST).unwrap(),
            ..agent
        };
        diesel::insert_into(agents::table)
        .values(&InsertableAgent::from_user(encrypted_user))
        .execute(connection)?;

        agents::table.order(agents::id.desc()).first(connection)
    }

    pub fn get_by_agent_id_and_pid(agent_id_: String, password_: String, connection: &PgConnection) -> Option<Agent> {
        let res = agents::table
            .filter(agents::agent_id.eq(agent_id_))
            .get_result::<Agent>(connection);
        match res {
            Ok(agent) => {
                return Some(agent)
                }
            Err(_) => {
                None
            }
        }
    }
}




//TESTING

// routes?

//#[post("/reg", format="application/json", data = "<agent>")]
//fn create(agent: Json<Agent>, connection: &mut diesel::PgConnection) -> Result<Json<Agent>, Status> {
//    Agent::create(agent.into_inner(), connection)
//        .map(Json)
//        .map_err(|_| Status::InternalServerError);
//        Ok(Outcome::Success("YAY"));
//        Err(Outcome::Failure((Status::ServiceUnavailable, ())));
//}

//#[post("/reg", data = "<newagent>")]
//pub fn create_agent(newagent: NewAgent<'r> ) {
//    println!("creating agent!: {:#?}", agent);
//    use crate::schema::agents::dsl::*;
//
//    let connection: &mut PgConnection = &mut establish_conn();
//
//    let new_agent = NewAgent {
//        agent_id: &agent.agent_id,
//        agent_pid: &agent.agent_pid,
//        agent_ip: &agent.agent_ip,
//    };
//
//    diesel::insert_into(agents)
//        .values(&new_agent)
//        .execute(connection);
//}

// error handler ?


    //use crate::db::diesel::deserialize::Result;
    //let myError = Some(Status::ServiceUnavailable);

    //let x = Json(
    //        InsertableAgent { 
    //            agent_id: ((*db_agent.agent_id)).to_string(), 
    //            agent_pid: (*db_agent.agent_pid).to_string(), 
    //            agent_ip: (*db_agent.agent_ip).to_string(),
    //        }
    //    );
    //if Error {
    //    Json(myError)
    //}

    //Err(Outcome::Failure((Status::ServiceUnavailable, ())));


// RETURNING ITEMS FROM DB

    //leaving annotes bc will prob need to reference for new routes..

    //let mut v: Vec<AgentModel> = Vec::new();

    //for a in results.0 {
    //    v.push(a);
    //    //println!("AGENTS DB!: {:#?}", *a);
    //};

    //return Json(v);