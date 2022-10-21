//use diesel::pg::PgConnection;
use r2d2;
use r2d2::ManageConnection;
use rocket::data::FromData;
use rocket::{Request, State, outcome::Outcome};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket_sync_db_pools::diesel::connection;
use rocket_sync_db_pools::diesel::r2d2::ConnectionManager;
use std::env;
use std::ops::Deref;
use rocket::{Rocket, Build, Error, FromForm};
use rocket::fairing::AdHoc;
use rocket::response::{Debug, status::Created};
use rocket::serde::{Serialize, Deserialize};
use rocket::form::Form;
use diesel;
use diesel::prelude::*;
use diesel::Queryable;
use rocket::serde::json::Json;
use crate::schema::agents;



//type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
//
//pub fn init_pool() -> Pool {
//    let manager = ConnectionManager::<PgConnection>::new//(database_url());
//    Pool::new(manager).expect("db pool")
//}
//fn database_url() -> String {
//    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
//}

#[database("postgres")]
struct Db(rocket_sync_db_pools::diesel::PgConnection);



pub fn establish_conn() -> PgConnection {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE URL MUST BE SET!!");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}",  database_url))
}
//impl<'r> FromRequest<'r> for Db {
//    type Error = ();
//    fn from_request(request: &'a Request<'r>) -> request::Outcome<Db, Self::Error> {
//        match Db.get() {
//           Ok(conn) => Outcome::Success(Db(conn)),
//            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//        }
//    }
//}


// this is to get users from the database
#[derive(Serialize, Clone, Deserialize, Debug, Queryable, FromForm)]
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



#[derive(Debug, Queryable, AsChangeset, FromForm, Serialize, Deserialize)]
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



//#[rocket::async_trait]
//impl<'r> FromData<'r> for NewAgent {
//    type Error = MyError;
//
//    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
//        /* .. */
//    }
//}

//#[rocket::async_trait]
//impl<'r> FromRequest<'r> for NewAgent {
//    type Error = ();
//
//    fn from_request(request: &'r Request<'_>) -> Outcome<AgentModel, (), ()> {
//        // This will unconditionally query the database!
//        let agent = try_outcome!(request.guard::<User>().await);
//        if agent.agent_id {
//            Outcome::Success(AgentModel { agent_id })
//        } else {
//            Outcome::Forward(())
//        }
//    }
//}


//impl NewAgent {
//    fn from_agent(agent: Agent) -> NewAgent {
//        NewAgent {
//            agent_id: agent.agent_id,
//            agent_pid: agent.agent_pid,
//            agent_ip: agent.agent_ip,
//        }
//    }
//}

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

    pub fn get_by_agent_id_and_pid(agent_id_: String, password_: String, connection: &mut PgConnection) -> Option<Agent> {
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


//type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

//pub fn init_pool() -> Pool {
//    let manager = ConnectionManager::<PgConnection>::new(database_url());
//    Pool::new(manager).expect("db pool")
//}

//fn database_url() -> String {
//    env::var("DATABASE_URL").expect("DATABASE_URL must be //set")
//}

//pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);
//
//impl<'a, 'r> FromRequest<'r> for DbConn {
//    type Error = ();
//
//    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
//        let pool = request.guard::<State<Pool>>()?;
//        match pool.get() {
//            Ok(conn) => Outcome::Success(DbConn(conn)),
//            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
//        }
//    }

//impl Deref for DbConn {
//    type Target = PgConnection;
//
//    fn deref(&self) -> &Self::Target {
//        &self.0
//    }
//}


//async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
//    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
//    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
//    Db::get_one(&rocket).await
//        .expect("database connection")
//        .run(|conn: &mut PgConnection | { conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations"); })
//        .await;

//    rocket
//}
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
    //Json(_res);
    //let agent_id = "HAHAHA".to_string();
    //let agent_pid = "4123".to_string();
    //let agent_ip = "127.0.0.1".to_string();

    //Json(InsertableAgent { agent_id, agent_pid, agent_ip })
    Json(
        InsertableAgent { 
            agent_id: (_res.agent_id), 
            agent_pid: (_res.agent_pid), 
            agent_ip: (_res.agent_ip),
        }
    )
}


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
            ]
        )
    })
}
