use chrono::{DateTime, Utc};
use rand::Rng;

pub struct Wallet{

    pub id: i32,
    pub name: String,
    pub status: Status,
    pub owner_name: String,
    pub password: String,
    pub creation_date: DateTime<Utc>,

}

pub enum Status{

    ACTIVE,
    INACTIVE,

}

impl Wallet{

    pub fn generate_wallet_id() -> i32 {
        
        let id = rand::thread_rng().gen_range(1..999999999);

        id

    }
    


}

impl Status{

    pub fn to_string(&self) -> String{

        match self{

            Status::ACTIVE => "ACTIVE".to_string(),
            Status::INACTIVE => "INACTIVE".to_string(),

        }

    }

    pub fn to_status(str_status: &String) -> Status{

        if str_status.to_string() == "ACTIVE".to_string(){

            return Status::ACTIVE;

        }
        else{
            Status::INACTIVE
        }

    }


}