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
