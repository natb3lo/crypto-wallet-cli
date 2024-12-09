use chrono::Utc;
use tokio_postgres::{tls::NoTlsStream, Client, Connection, NoTls, Socket};

use crate::entities::{concrete::wallet::Status, services::panel::Panel};
use crate::entities::concrete::{wallet::Wallet, owner::Owner};
use std::{thread, time::Duration};

use super::db_connection::DbConnection;

pub struct Validation;

impl Validation{

    pub async fn is_create_wallet_valid(username: &String, pass: &String, pass_confirmation: &String, psql_client: &Client) -> bool{
        
        if username.len() < 4{
            Panel::clear_panel();
            println!("⚠️: USERNAME NEEDS TO BE AT LEAST 4 CHARACTERS LONG!");
            thread::sleep(Duration::from_secs(2));
            return false;
        }
        else if pass.len() < 4{

            Panel::clear_panel();
            println!("⚠️: PASSWORD NEEDS TO BE AT LEAST 4 CHARACTERS LONG!");
            thread::sleep(Duration::from_secs(2));
            return false;
        }
        else if pass != pass_confirmation {

            Panel::clear_panel();
            println!("⚠️: PASSWORDS DO NOT MATCH!");
            thread::sleep(Duration::from_secs(2));
            return false;

        }
        else{

            let user_exist = DbConnection::is_user_in_db(psql_client, username).await;
            
            if user_exist{
                return false;
            }
            else{

                let mut is_id_unique = false;
                while(!is_id_unique){
                    
                    let wallet_id = Wallet::generate_wallet_id();

                    if(DbConnection::exist_wallet_id(wallet_id, psql_client)).await{
                        continue;
                    }
                    else{
                        
                        is_id_unique = true;
                        
                        let wallet_name = format!("{}'s Wallet", username.to_string());

                        let wallet = Wallet{
                            id: wallet_id,
                            name: wallet_name,
                            status: Status::ACTIVE,
                            owner_name: username.to_string(),
                            password: pass.to_string(),
                            creation_date: Utc::now(),
                            balance: 10000.00,
                        };

                        let owner = Owner{

                            name: username.to_string()

                        };

                        println!("Wallet Date: {}", wallet.creation_date);
                        //thread::sleep(Duration::from_secs(3));
                        DbConnection::create_user_wallet_relation(&owner, &wallet, psql_client).await;

                    }

                }

                true
            }

        }

    }


}