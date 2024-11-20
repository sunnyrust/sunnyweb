use aok::AokModel;

#[derive(Debug, sqlx::FromRow, AokModel)]
#[aok(db(Postgres))]
#[aok (schema="storyboard" ,table = "emotion")]
pub struct Emotion {
    #[aok(primary_key)]
    pub id: i32,
    pub name: String,
    pub code: String,
    pub unicode: String,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "postgres://sa:123456@localhost:5432/helloworld";
    let pool = aok::connection::connect_postgres(url).await?;
    //let products = Emotion::where_col(|p| p.id.equal(1)).run(&pool).await?;
    let products = Emotion::all().run(&pool).await?;
    println!("{:?}", products);
    println!("Hello, world!");
    Ok(())
}
