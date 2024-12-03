use config::CONFIG;
use postgres::{Client, Error, NoTls, Row};

#[macro_export]
macro_rules! create_connection_postgres_string {
    ($host:expr, $user:expr, $password:expr, $port:expr, $dbname:expr) => {
        format!(
            "host={} user={} password={} port={} dbname={}",
            $host, $user, $password, $port, $dbname
        )
    };
}

fn database_url() -> String {
    CONFIG.postgres.to_string()
}

pub struct Engine {
    client: Client,
}

impl Engine {
    pub fn new() -> Self {
        let client = Client::connect(&database_url(), NoTls).unwrap();
        Self { client }
    }
}

pub trait EngineOp {
    fn insert(&mut self, sql: &str);
    fn query(&mut self, sql: &str) -> Result<Vec<Row>, Error>;
}

impl EngineOp for Engine {
    fn insert(&mut self, _sql: &str) {
        // let data = None::<&[u8]>;
        // self.client.execute(
        // "INSERT INTO person (name, data) VALUES ($1, $2)",
        // &[&name, &data],
        // ).unwrap();
    }

    fn query(&mut self, _sql: &str) -> Result<Vec<Row>, Error> {
        for row in self.client.query("SELECT * FROM hjobs", &[]).unwrap() {
            let prompt: &str = row.get(2);
            println!("found prompt: {:?}", prompt);
        }

        todo!()
    }
}