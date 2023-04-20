use {
    diesel::{
        ExpressionMethods,
        Insertable,
        Queryable,
        QueryDsl,
        RunQueryDsl
    },
    diesel::result::Error,
    rand::Rng,
    serde::{
        Deserialize,
        Serialize
    },
    uuid::Uuid,
    // super::schema::miners,
    // crate::DBPooledConnection,
    crate::miner::*,
};

// ---- JSON Payload
#[derive(Debug, Deserialize, Serialize)]
pub struct Wallet {
    pub address: String,
    pub club_name: String,

    pub total_hash_rate: i32,
    pub total_shares_mined: i32,
    pub total_workers_online: i32,
    pub workers_online: Vec<Miner>,
}

// ---- POST Request Body for new Miner
#[derive(Debug, Deserialize, Serialize)]
pub struct NewWalletRequest {
    club_name: String,
}

// ---- DAO Object (DB Table Records)
#[derive(Insertable, Queryable)]
#[table_name = "wallets"]
pub struct WalletDAO {
    pub address: Uuid,
    pub club_name: String,
}


// ---- IMPLEMENTATIONS

impl Wallet {
        pub fn to_wallet_dao(&self) -> WalletDAO {
            WalletDAO {
                address: Uuid::parse_str(self.address.as_str()).unwrap(),
                club_name: self.club_name.to_string(),

            }
        }
}

impl WalletDAO {
    pub fn to_walle(&self, workers_online: Vec<Miners>) -> Wallet {
         Wallet {
             address: self.address.to_string(),
             club_name: self.club_name.to_string(),
             total_hash_rate: workers_online.iter().map(|w| w.has_rate).sum(),
             total_shares_mined: workers_online.iter().map(|w| w.shares_mined).sum(),
             total_workers_online: workers_online.len() as i32,
             workers_online,
         }
    }
}