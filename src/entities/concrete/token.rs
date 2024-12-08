
pub struct Token{
    pub id: String,
    pub name: String,
    pub symbol: String,
    pub mkt_value: f64,

}


#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TokenInDatFile{
    
    pub id: String,
    pub name: String,
    pub symbol: String,

}

impl Token{





}