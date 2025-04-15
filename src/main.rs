mod agent;
mod chat;
mod document_loader;
mod models;
mod tools;
mod vector_store;

#[tokio::main]
async fn main() {
    chat::start_chat_session().await;
}
