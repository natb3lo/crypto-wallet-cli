use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::atomic::Ordering;
use std::time::Duration;
use bincode::{deserialize_from, serialize_into};
use serde::{Serialize, Deserialize};
use tokio::time::sleep;
use std::fs::{OpenOptions, create_dir_all};
use std::path::Path;
use std::thread;
use crate::entities::concrete::{token::{Token, TokenInDatFile}};
use reqwest;
use crate::entities::services::panel::Panel;
use std::fmt::Error;

pub struct ApplicationFiles;

#[derive(Serialize, Deserialize)]
pub struct ProgramFileCounter{

    pub counter: i32,

}

impl ApplicationFiles {
    
    //implementation pending

}

impl ApplicationFiles {
    
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

        }
        else{
            counter.counter += 1;
            serialize_into(&f, &counter).unwrap();
        }

    }

    pub async fn get_index_file() -> Result<File, Error>{

        let url_list = "https://api.coingecko.com/api/v3/coins/list";
        let response = reqwest::get(url_list).await.unwrap();
        if response.status().is_success(){
            
                //println!("get_dex...");
                //thread::sleep(Duration::from_secs(3));
                let mut token_dat_list: Vec<TokenInDatFile> = response.json().await.unwrap();
                //println!("{:?}", token_dat_list.get(100));
                //thread::sleep(Duration::from_secs(3));
                for token_dat in token_dat_list.iter_mut(){
                    token_dat.name = token_dat.name.to_lowercase();
                }

                //println!("{:?}", token_dat_list.get(100));
                //thread::sleep(Duration::from_secs(3));

                token_dat_list.sort_by(|a, b| a.name.cmp(&b.name));

                let index_file_path = "app_files\\index.dat";
                let f_index_path = Path::new(index_file_path);
                let index_dir = f_index_path.parent().unwrap();

                if !index_dir.exists(){
                    create_dir_all(f_index_path).unwrap();
                }

                let mut f_index = OpenOptions::new().create(true).write(true).read(true).open(f_index_path).unwrap();

                for tk in token_dat_list{

                    let mut buffer = [0u8; 96];

                    buffer[0..32].copy_from_slice(&ApplicationFiles::pad_field(&tk.id, 32));
                    buffer[32..64].copy_from_slice(&ApplicationFiles::pad_field(&tk.name, 32));
                    buffer[64..96].copy_from_slice(&ApplicationFiles::pad_field(&tk.symbol, 32));

                    f_index.write_all(&buffer);
                }

                /* 
                
                let serialized = bincode::serialize(&token_dat_list).unwrap();
                
                f_index.write_all(&serialized).unwrap();
                */

                Ok(f_index)

            }
            else{

                Err(Error)

            }



    }

    pub fn search_in_index_file(f: &mut File, guess: &String) -> Result<TokenInDatFile, Error>{

        let struct_size = 96;

        let file_len = f.metadata().unwrap().len();
        let num_records = (file_len / struct_size as u64) as usize;

        //println!("{}", num_records);
        //thread::sleep(Duration::from_secs(3));

        let mut low = 0;
        let mut high = num_records - 1;
        while low <= high {

            let mid = (low + high) / 2;

            let pos = mid * struct_size;

            f.seek(SeekFrom::Start(pos as u64)).unwrap();

            let mut buffer = vec![0u8; struct_size];

            f.read_exact(&mut buffer).unwrap();

            let dtoken = TokenInDatFile::from_bytes_to_token_in_dat(&buffer);

            //println!("{:?}", dtoken);
            //thread::sleep(Duration::from_secs(3));

            match dtoken.name.cmp(guess) {
                
                std::cmp::Ordering::Equal => {
                    Panel::clear_panel();
                    //println!("Found: {:?}", dtoken);
                    //thread::sleep(Duration::from_secs(3));
                    return Ok(dtoken);
                }
                std::cmp::Ordering::Greater => {

                    high = mid - 1;

                }
                std::cmp::Ordering::Less => {

                    low = mid + 1;

                }

            }


        }

        Err(Error)


    }

    pub fn pad_field(value: &str, size: usize) -> Vec<u8>{

        let mut padded = vec![0u8; size];
        let bytes = value.as_bytes();
        let len = bytes.len().min(size);
        padded[..len].copy_from_slice(&bytes[..len]);
        padded

    }

    

}