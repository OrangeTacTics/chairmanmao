#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let store = chairmanmao::store::Store::new().await;
    let profile = store.load_profile(876378479585808404).await;

    dbg!(profile);
}
