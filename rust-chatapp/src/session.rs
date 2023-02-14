//Changing the duration of the WebSocket
//HEARTBEAT is the duration to keep the connection alive with the client
//CLIENT_TIMEOUT is the duration to check if the client is still connected

const HEARBEET: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);
type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;