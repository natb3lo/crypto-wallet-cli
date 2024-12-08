use chrono::Date;
use tokio_postgres::{tls::NoTlsStream, types::ToSql, Client, Connection, NoTls, Socket};
use crate::entities::concrete::{owner::{self, Owner}, wallet::{Wallet, Status}};

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
        let query_wallet = "insert into wallet (id, name, status, ownername, password, creationdate) values ($1, $2, $3, $4, $5, $6)";

        psql_client.execute(query_owner, &[&owner.name]).await.unwrap();
        psql_client.execute(query_wallet, &[&wallet.id, &wallet.name, &wallet.status.to_string(), &wallet.owner_name, &wallet.password, &wallet.creation_date.format("%Y-%m-%d %H:%M:%S%.f UTC").to_string()]).await.unwrap();

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
        let query_wallet = "SELECT id, name, status, ownername, password, creationdate 
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
        };

        (owner, wallet)


        


    }
}