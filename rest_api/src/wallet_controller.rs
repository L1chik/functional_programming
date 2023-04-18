use std::cell::Cell;
use std::env::current_exe;
use actix_web::{get, HttpResponse, web::Json};
use {
    crate::wallet::*,
    crate::util::*,

};

// ---- List all Wallets
#[get("/wallets")]
pub async fn list_wallets() -> HttpResponse {
    let wallets: Vec<Wallet> = vec![];
    ResponseType::Ok(wallets).get_response()
}

// ---- Get Wallet
#[get("/wallets/{id}")]
pub async fn get_wallet() -> HttpResponse {
    let wallet: Option<Wallet> = None;

    match wallet {
        Some(wallet) => ResponseType::Ok(wallet).get_response(),
        None => ResponseType::NotFound(
            NotFoundMessage::new("Wallet not found".to_string())
        ).get_response(),
    }
}

// ---- Create new Wallet
#[post("/wallets")]
pub async fn create_wallet(wallet_request: Json<NewWalletRequest>) -> HttpResponse {
    let wallet: Vec<Wallet> = vec![];
    ResponseType::Created(wallet).get_response()
}