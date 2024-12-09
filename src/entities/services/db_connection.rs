use std::{thread, time::Duration};

use bincode::Error;
use chrono::Date;
use crossterm::execute;
use tokio_postgres::{tls::NoTlsStream, types::ToSql, Client, Connection, NoTls, Socket};
use crate::entities::concrete::{owner::{self, Owner}, token::Token, wallet::{Status, Wallet}, walletoken::Walletoken};

pub struct DbConnection;

impl DbConnection{

    pub async fn establish_connection(db_url: &String) -> (Client, Connection<Socket, NoTlsStream>){

        let (pslq_client, psql_connection) = tokio_postgres::connect(db_url, NoTls).await
        .expect(format!("[ 🚨 ]: CONNECTION TO HOST \'{}\' HAS FAILED!", db_url).as_str());

        (pslq_client, psql_connection)

    }

    pub async fn is_user_in_db(psql_client: &Client, user_name: &String) -> bool{
        
        let query = "SELECT * FROM owner WHERE name = $1";
        let row = psql_client.query(query, &[user_name]).await.unwrap();

        if row.is_empty(){
            return false;
        }
        else{
            true
        }
  

    }

    pub async fn create_user_wallet_relation(owner: &Owner, wallet: &Wallet, psql_client: &Client) -> bool{

        let query_owner = "insert into owner (name) values ($1)";
        let query_wallet = "insert into wallet (id, name, status, ownername, password, creationdate, balance) values ($1, $2, $3, $4, $5, $6, $7)";

        psql_client.execute(query_owner, &[&owner.name]).await.unwrap();
        psql_client.execute(query_wallet, &[&wallet.id, &wallet.name, 
            &wallet.status.to_string(), &wallet.owner_name, &wallet.password, 
            &wallet.creation_date.format("%Y-%m-%d %H:%M:%S%.f UTC").to_string(), 
            &wallet.balance]).await.unwrap();

        true

    }

    pub async fn exist_wallet_id(id: i32, psql_client: &Client) -> bool{

        let query = "select * from wallet where id=$1";
        let row = psql_client.query(query, &[&id]).await.unwrap();

        if row.is_empty(){
            return false;
        }
        else{
            true
        }


    }

    pub async fn exists_user_wallet_relation(psql_client: &Client, username: &String, password: &String) -> bool{

        let query = "SELECT 1 FROM Wallet JOIN Owner ON Owner.name = Wallet.ownername
                            WHERE Owner.name = $1 AND Wallet.password = $2 LIMIT 1 ";

        let rows = psql_client.query(query, &[&username, &password]).await.unwrap();

        if !(rows.is_empty()){
            return true;
        }
        else {
            false
        }


    }

    pub async fn get_owner_wallet_ralation(psql_client: &Client, username: &String) -> (Owner, Wallet){

        let query_owner = "SELECT * FROM owner WHERE name=$1";
        let query_wallet = "SELECT id, name, status, ownername, password, creationdate, balance 
                                    FROM wallet where ownername=$1
                                    LIMIT 1";

        let row_owner = psql_client.query_one(query_owner, &[&username]).await.unwrap();
        let row_wallet = psql_client.query_one(query_wallet, &[&username]).await.unwrap();

        let owner = Owner{
            name: row_owner.get("name"),
        };

       
        let wallet_status_field_row: String = row_wallet.get("status");
        let wallet_creation_date_field_row: String = row_wallet.get("creationdate");
        
        let wallet = Wallet{
            id: row_wallet.get("id"),
            name: row_wallet.get("name"),
            status: Status::to_status(&wallet_status_field_row),
            owner_name: row_wallet.get("ownername"),
            password: row_wallet.get("password"),
            creation_date: Wallet::from_str_to_utc(&wallet_creation_date_field_row),
            balance: row_wallet.get("balance"),
        };

        (owner, wallet)


    }

    pub async fn set_wallettoken_relation(wallet: &Wallet, token: &Token, wallettoken: &mut Walletoken, psql_client: &Client) -> bool{

        let verification_query = "SELECT * FROM walletoken WHERE walletid=$1 AND tokensymbol=$2";

        let row_verify = psql_client.query(verification_query, 
                                                &[&wallet.id, &token.symbol]).await.unwrap();

        if row_verify.is_empty(){

            let query = "INSERT INTO walletoken (walletid, tokensymbol, amount) VALUES ($1, $2, $3)";
            let row = psql_client.execute(query, &[&wallettoken.wallet_id, 
                                                &wallettoken.token_symbol, &wallettoken.amount]).await.unwrap();


        }
        else{
            let query_old = "SELECT * FROM walletoken WHERE walletid=$1 AND tokensymbol=$2";
            let row_old = psql_client.query_one(query_old, &[&wallet.id, &token.symbol]).await.unwrap();

            let old_wallettoken: Walletoken = Walletoken{

                wallet_id: row_old.get("walletid"),
                token_symbol: row_old.get("tokensymbol"),
                amount: row_old.get("amount"),

            };

            wallettoken.amount += old_wallettoken.amount;
            //println!("{:?}", wallettoken.amount);
            //thread::sleep(Duration::from_secs(5));

            let query = "UPDATE walletoken SET amount=$1";
            let row = psql_client.execute(query, &[&wallettoken.amount]).await.unwrap();

        }

        true                                                


    }

    pub async fn update_wallet_balance(wallet: &Wallet, psql_client: &Client){

        let query = "UPDATE wallet SET balance=$1 WHERE id=$2";
        psql_client.execute(query, &[&wallet.balance, &wallet.id]).await.unwrap();

    }

    pub async fn set_token_relation(token: &Token, psql_client: &Client){

        let verification_query = "SELECT * FROM token WHERE symbol=$1";
        
        let row_verification = psql_client.query(verification_query, &[&token.symbol]).await.unwrap();
        
        if  row_verification.is_empty(){
            
            let query = "INSERT INTO token (name, symbol, marketvalue) VALUES ($1, $2, $3)";
            psql_client.execute(query, &[&token.name, &token.symbol, &token.mkt_value]).await.unwrap();
            
        }
        else{

            let query = "UPDATE token SET marketvalue=$1 WHERE symbol=$2";
            psql_client.execute(query, &[&token.mkt_value, &token.symbol]).await.unwrap();

        }
        
        

    }

    pub async fn delete_user_wallet_relation(owner: &Owner, wallet: &Wallet, psql_client: &Client) -> bool{

        let mut query = "DELETE FROM walletoken WHERE walletid=$1";
        psql_client.execute(query, &[&wallet.id]).await.unwrap();

        query = "DELETE FROM owner WHERE name=$1";
        psql_client.execute(query, &[&owner.name]).await.unwrap();

        return true;

    }

    pub async fn get_walletoken_list_from_wallet_id(wallet: &Wallet, psql_client: &Client) -> Result<(Vec<Walletoken>, Vec<Token>), Error>{

        let mut query = "SELECT walletid, tokensymbol, amount FROM walletoken WHERE walletid=$1";
        let mut rows = psql_client.query(query, &[&wallet.id]).await.unwrap();
        let walletoken_list: Vec<Walletoken> = rows.iter().map(Walletoken::from_row).collect();

        query = "SELECT * FROM token";
        rows = psql_client.query(query, &[]).await.unwrap();
        let token_list: Vec<Token> = rows.iter().map(Token::from_row).collect();

        Ok((walletoken_list, token_list))

    }
}