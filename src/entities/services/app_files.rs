use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::time::Duration;
use bincode::{serialize_into, deserialize_from};
use serde::{Serialize, Deserialize};
use std::fs::{OpenOptions, create_dir_all};
use std::path::Path;
use std::thread;
use crate::entities::concrete::{token::{Token, TokenInDatFile}};
use reqwest;
use crate::entities::services::panel::Panel;
pub struct ApplicationFiles;

#[derive(Serialize, Deserialize)]
pub struct ProgramFileCounter{

    pub counter: i32,

}

impl ApplicationFiles {
    
    //implementation pending

}

impl ProgramFileCounter {
    
    pub async fn update_file_counter(){

        let file_counter_path = "app_files\\execution.dat";

        let f_path = Path::new(file_counter_path);
        let dir = f_path.parent().unwrap();
        if !dir.exists(){
            create_dir_all(dir).unwrap();
        }

        let mut f = OpenOptions::new().create(true).write(true).read(true).open(f_path).unwrap();

        let mut counter:ProgramFileCounter = match deserialize_from(&f){

            Ok(counter) => counter,

            Err(error) => {
                ProgramFileCounter{ counter:0 }
            }

        };
        
        f.seek(SeekFrom::Start(0)).unwrap();
        
        if counter.counter == 0{

            counter.counter += 1;
            serialize_into(&f, &counter).unwrap();
            //baixa arquivo de ids
            
            let url_list = "https://api.coingecko.com/api/v3/coins/list";
            let response = reqwest::get(url_list).await.unwrap();

            if response.status().is_success(){

                let mut token_dat_list: Vec<TokenInDatFile> = response.json().await.unwrap();

                for token_dat in token_dat_list.iter_mut(){
                    token_dat.name = token_dat.name.to_lowercase();
                }

                token_dat_list.sort_by(|a, b| a.name.cmp(&b.name));

                let index_file_path = "app_files\\index.dat";
                let f_index_path = Path::new(index_file_path);
                let index_dir = f_index_path.parent().unwrap();

                if !index_dir.exists(){
                    create_dir_all(f_index_path).unwrap();
                }

                let mut f_index = OpenOptions::new().create(true).write(true).read(true).open(f_index_path).unwrap();

                let serialized = bincode::serialize(&token_dat_list).unwrap();

                f_index.write_all(&serialized).unwrap();

            }

        }
        else{
            counter.counter += 1;
            serialize_into(&f, &counter).unwrap();
        }

        

    }

}