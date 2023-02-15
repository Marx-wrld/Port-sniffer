//Changing the duration of the WebSocket
//HEARTBEAT is the duration to keep the connection alive with the client
//CLIENT_TIMEOUT is the duration to check if the client is still connected

const HEARBEET: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

//Creating structs to store all the data we need
#[derive(Debug)]
pub struct WsChatSession {
    pub id: usize,
    pub hb: Instant,
    pub room: String,
    pub name: Option<String>,
    pub addr: Addr<server::ChatServer>,
    pub db_pool: web::Data<DbPool>,
}
#[derive(PartialEq, Serialize, Deserialize)]
pub enum ChatType {
    TYPING,
    TEXT,
    CONNECT,
    DISCONNECT,
}
#[derive(Serialize, Deserialize)]
struct ChatMessage {
    pub chat_type: ChatType,
    pub value: Vec<String>,
    pub room_id: String,
    pub user_id: String,
    pub id: usize,
}