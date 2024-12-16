use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write, ErrorKind};
use std::process::exit;
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
    
    pub async fn update_file_counter() -> i32{

        let mut fc_path = String::new();

        if cfg!(target_os = "windows"){
            fc_path = "app_files\\execution.dat".to_string();
            //println!("Windows!");
            //thread::sleep(Duration::from_secs(3));
        }
        else if cfg!(target_os = "linux"){
            fc_path = "app_files/execution.dat".to_string();
        }
        else{
            println!("[ERROR] : UNSUPORTED OS VERSION!");
            exit(1);
        }

        let mut fc = match File::open(&fc_path){
            Ok(_f) => OpenOptions::new().create(true).write(true).read(true).open(fc_path).unwrap(),

            Err(error) => match error.kind(){
                
                ErrorKind::NotFound => {

                    if let Some(parent_dir) = std::path::Path::new(&fc_path).parent(){

                        if let Err(e) = create_dir_all(parent_dir){
                            panic!("[ERROR] : PROBLEM CREATING DIRECTORY FOR <execution.dat> FILE!");
                        }

                    }
                    match File::create(&fc_path){
                        Ok(fc) => fc,
                        Err(e) => panic!("[ERROR] : PROBLEM CREATING <execution.dat> FILE!"),

                    }
                }
                other_error => {
                    panic!("[ERROR] : PROBLEM OPENING <execution.dat> FILE!")
                }

            }
        };

        //let mut f = OpenOptions::new().create(true).write(true).read(true).open(fc_path).unwrap();

        let mut counter:ProgramFileCounter = match deserialize_from(&fc){

            Ok(counter) => counter,

            Err(error) => {
                ProgramFileCounter{ counter:0 }
            }

        };
        
        fc.seek(SeekFrom::Start(0)).unwrap();
        
        if counter.counter == 0{
            
            counter.counter += 1;
            serialize_into(&fc, &counter).unwrap();
            return 0;

        }
        else{
            
            counter.counter += 1;
            serialize_into(&fc, &counter).unwrap();
            counter.counter
        }

    }

    pub async fn get_index_file(counter: i32) -> Result<File, Error>{

        let mut fi_path = String::new();
        if cfg!(target_os = "windows"){
            fi_path = ".\\app_files\\index.dat".to_string();
        }
        else if cfg!(target_os = "linux"){
            fi_path = "./app_files/index.dat".to_string();
        }
        else{
            println!("[ERROR] : UNSUPORTED OS VERSION!");
            exit(1);
        }

        if counter == 0{
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
    
                    //let index_file_path = "app_files\\index.dat";
                    //let f_index_path = Path::new(index_file_path);
                   //let index_dir = f_index_path.parent().unwrap();
                    /*
                    if !index_dir.exists(){
                        create_dir_all(f_index_path).unwrap();
                    }
                    */
    
                    let mut fi = match File::open(&fi_path){
                        Ok(_f) => OpenOptions::new().create(true).write(true).read(true).open(fi_path).unwrap(),
            
                        Err(error) => match error.kind(){
                            
                            ErrorKind::NotFound => {
            
                                if let Some(parent_dir) = std::path::Path::new(&fi_path).parent(){
            
                                    if let Err(e) = create_dir_all(parent_dir){
                                        panic!("[ERROR] : PROBLEM CREATING DIRECTORY FOR <index.dat> FILE!");
                                    }
            
                                }
                                match File::create(&fi_path){
                                    Ok(fc) => fc,
                                    Err(e) => panic!("[ERROR] : PROBLEM CREATING <index.dat> FILE!"),
            
                                }
                            }
                            other_error => {
                                panic!("[ERROR] : PROBLEM OPENING <index.dat> FILE!")
                            }
            
                        }
                    };
    
                    //let mut f_index = OpenOptions::new().create(true).write(true).read(true).open(f_index_path).unwrap();
    
                    for tk in token_dat_list{
    
                        let mut buffer = [0u8; 96];
    
                        buffer[0..32].copy_from_slice(&ApplicationFiles::pad_field(&tk.id, 32));
                        buffer[32..64].copy_from_slice(&ApplicationFiles::pad_field(&tk.name, 32));
                        buffer[64..96].copy_from_slice(&ApplicationFiles::pad_field(&tk.symbol, 32));
    
                        fi.write_all(&buffer).unwrap();
                    }
    
                    /* 
                    
                    let serialized = bincode::serialize(&token_dat_list).unwrap();
                    
                    f_index.write_all(&serialized).unwrap();
                    */
    
                    Ok(fi)
    
                }
                else{
    
                    Err(Error)
    
                }

        }
        else{

            let mut fi = match File::open(&fi_path){
                Ok(_f) => OpenOptions::new().create(true).write(true).read(true).open(fi_path).unwrap(),
    
                Err(error) => match error.kind(){
                    
                    ErrorKind::NotFound => {
    
                        if let Some(parent_dir) = std::path::Path::new(&fi_path).parent(){
    
                            if let Err(e) = create_dir_all(parent_dir){
                                panic!("[ERROR] : PROBLEM CREATING DIRECTORY FOR <execution.dat> FILE!");
                            }
    
                        }
                        match File::create(&fi_path){
                            Ok(fc) => fc,
                            Err(e) => panic!("[ERROR] : PROBLEM CREATING <execution.dat> FILE!"),
    
                        }
                    }
                    other_error => {
                        panic!("[ERROR] : PROBLEM OPENING <execution.dat> FILE!")
                    }
    
                }
            };

            Ok(fi)

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