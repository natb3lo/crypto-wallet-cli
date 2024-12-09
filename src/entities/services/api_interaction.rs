
use std::{thread, time::Duration};

use serde::Deserialize;
use crate::entities::concrete::token::Token;
use reqwest::{self};

#[derive(Deserialize, Debug)]
pub struct GekoToken{

    pub id: String,
    pub symbol: String,
    pub name: String,
    pub market_data: MarketData,


}

#[derive(Deserialize, Debug)]
pub struct MarketData{

    pub current_price: CurrentPrice,

}

#[derive(Deserialize, Debug)]
pub struct CurrentPrice{

    pub usd: f64,

}

pub struct WebInteraction;

impl WebInteraction{

    pub async fn get_token(token_id: &String) -> Token{

        let url = format!("https://api.coingecko.com/api/v3/coins/{}", token_id);

        let response = reqwest::get(url).await.unwrap();

        if response.status().is_success(){

            let geko_tk: GekoToken = response.json().await.unwrap();
            //println!("From API: {:?}", geko_tk);
            //thread::sleep(Duration::from_secs(3));
            let token = Token{

                name: geko_tk.name,
                symbol: geko_tk.symbol,
                mkt_value: geko_tk.market_data.current_price.usd,
    
            };

           return token;


        }
        else{

            Token{name: "".to_string(), symbol: "".to_string(), mkt_value: 0.0}

        }




    }




}