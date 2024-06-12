mod ya_gpt;
mod tg;
mod logic;

#[tokio::main]
async fn main() {
    logic::start_main_loop().await;
}
