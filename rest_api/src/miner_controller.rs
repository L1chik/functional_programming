use actix_web::{get, HttpResponse, web::Json};
use {
    crate::miner::*,
    crate::util::*,
    crate::wallet::*,
};

// ---- List all miners
#[get("/miners")]
pub async fn list_miners() -> HttpResponse {
    // TODO: Get all MinterDAO objects from DB and convert to Miner objects
    let miners: Vec<Miner> = vec![];
    ResponseType::Ok(miners).get_response()
}

// ---- Get Miner
#[get("/miners/{id}")]
pub async fn get_miner() -> HttpResponse {
    let miner: Option<Miner> = None;

    match miner {
        Some(miner) => ResponseType::Ok(miner).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Miner not found".to_string())
        ).get_response(),
    }
}

// ---- Create new Miner
#[post("wallets/{id}/miners")]
pub async fn create_miner(miner_request: Json<NewMinerRequest>) -> HttpResponse {
    let miner: Vec<Miner> = vec![];
    ResponseType::Created(miner).get_response()
}
