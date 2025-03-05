use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

pub struct Database {
    client: Surreal<Ws>,
}

impl Database {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let client = Surreal::new::<Ws>("wss://vitalogic-06aiiotl05vq9c7qvsffks0g8s.aws-euw1.surreal.cloud/rpc")
            .await?;

        client
            .signin(Root {
                username: "admin",
                password: "admin",
            })
            .await?;

        client.use_ns("Development phase").use_db("Vitalogic").await?;

        Ok(Database { client })
    }
}