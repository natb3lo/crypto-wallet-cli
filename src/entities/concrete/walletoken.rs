

use tokio_postgres::{Client, Row};

use crate::entities::services::{db_connection::DbConnection, panel::Panel};

use super::{token::Token, wallet::Wallet};

#[derive(Debug)]
pub struct Walletoken{

    pub wallet_id: i32,
    pub token_symbol: String,
    pub amount: f64,

}

impl Walletoken{

    pub async fn acquisition(order: f64 ,wallet: &mut Wallet, token: &Token, psql_client: &Client) -> Walletoken{

        let amount_of_tokens_bought = order / token.mkt_value;
        wallet.balance = wallet.balance - order;

        let mut wallettoken = Walletoken{
            wallet_id: wallet.id,
            token_symbol: token.symbol.clone(),
            amount: amount_of_tokens_bought,
        };

        DbConnection::set_token_relation(&token, &psql_client).await;
        DbConnection::set_wallettoken_relation(&wallet, &token, &mut wallettoken, &psql_client).await;
        DbConnection::update_wallet_balance(&wallet, &psql_client).await;

        wallettoken


    }

    pub fn from_row(row: &Row) -> Self{

        Self{

            wallet_id: row.get("walletid"),
            token_symbol: row.get("tokensymbol"),
            amount: row.get("amount"),

        }

    }

}