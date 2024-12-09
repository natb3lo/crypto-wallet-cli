
use tokio_postgres::{Client, Row};

#[derive(Debug)]
pub struct Token{
    pub name: String,
    pub symbol: String,
    pub mkt_value: f64,

}


#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
pub struct TokenInDatFile{
    
    pub id: String,
    pub name: String,
    pub symbol: String,

}

impl Token{

    pub fn from_row(row: &Row) -> Self{

        Self{

            name: row.get("name"),
            symbol: row.get("symbol"),
            mkt_value: row.get("marketvalue"),

        }

    }



}

impl TokenInDatFile {
    
    pub fn from_bytes_to_token_in_dat(bytes: &[u8]) -> Self{

        let id = String::from_utf8_lossy(&bytes[0..32]).trim_matches('\0').to_string();
        let name = String::from_utf8_lossy(&bytes[32..64]).trim_matches('\0').to_string();
        let symbol = String::from_utf8_lossy(&bytes[64..96]).trim_matches('\0').to_string();

        TokenInDatFile { id, name, symbol, }

    }

}