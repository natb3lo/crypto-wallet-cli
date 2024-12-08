mod enums;
mod entities;

use enums::{menu_option::MenuOption, prog_status::ProgramStatus::{RUNNING as RUNNING, NOT_RUNNING as NOT_RUNNING}};
use entities::{concrete::token::TokenInDatFile, services::{db_connection::DbConnection, panel::Panel, validate_registration::Validation}};
use std::{fs, thread::{self, sleep}};
use std::time::Duration;
use std::{env, process};
use tokio_postgres::NoTls;
use std::fs::{OpenOptions, create_dir_all};
use entities::services::app_files::ProgramFileCounter;

#[tokio::main]
async fn main() {
    
    let mut program_status = RUNNING;
    let mut db_ip = String::new();
    let mut db_url = String::new();
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        
        println!("🚨: PROGRAM NEEDS THE DATABASE'S IP ADRESS AS ARGUMENT [xxx.xxx.xxx.xxx]!");
        process::exit(1);

    }
    else {
        db_ip = args[1].clone();
        db_url = format!("postgres://postgres:natunix.23@{}:5432/walletproject", db_ip);
    }

    ProgramFileCounter::update_file_counter().await;
    //let data = fs::read("app_files\\index.dat").unwrap();
    //let token_dat_list: Vec<TokenInDatFile> = bincode::deserialize(&data).unwrap();
    //thread::sleep(Duration::from_secs(20));
    /* 
    for token_dat in token_dat_list{
        println!("{:?}", token_dat);
    }
    */

    //process::exit(1);

    let (psql_client, psql_connection) = DbConnection::establish_connection(&db_url).await;
    tokio::spawn(psql_connection);

    //----------->Main program loop<-----------------
    while program_status == RUNNING {

        Panel::generate_panel(&MenuOption::MAIN_MENU);
        let user_option: MenuOption = match Panel::get_integer_input(&MenuOption::MAIN_MENU){

            Ok(number) => {

                match number {

                    0 => MenuOption::EXIT,
                    
                    1 => MenuOption::LOGIN,

                    2 => MenuOption::CREATE_WALLET,

                    _ => MenuOption::INVALID_OPTION
                }

            }

            Err(_error) => {

                Panel::clear_panel();
                println!("[ERROR] : ONLY NUMBERS ARE ALLOWED!");
                thread::sleep(Duration::from_secs(2));
                continue;

            }

        };

        match user_option {
            
            MenuOption::LOGIN => {

                Panel::clear_panel();
                Panel::generate_panel(&user_option);
                let (username, password) = match Panel::get_str_input(&user_option){

                    Ok((user, pass, _)) => (user, pass),

                    Err(error) => {
                        Panel::clear_panel();
                        continue;
                    }
                };

                if (DbConnection::exists_user_wallet_relation(&psql_client, &username, &password).await){

                    //USUÁRIO LOGADO...
                    println!("USUÁRIO LOGADO...");

                }
                else{

                    println!("[ALERT] : USERNAME OR PASSWORD DOES NOT EXIST!");
                    thread::sleep(Duration::from_secs(2));
                    continue;

                }

            }

            MenuOption::CREATE_WALLET => {

                Panel::clear_panel();

                
                Panel::generate_panel(&user_option);
                //Panel::get_integer_input(&user_option);
                let (username, pass, pass_confirmation) = match Panel::get_str_input(&user_option){

                    Ok((user, pass, pass_confirmation)) => (user, pass, pass_confirmation),

                    Err(_error) => {

                        Panel::clear_panel();
                        continue;

                    }
                };

                if (Validation::is_create_wallet_valid(&username, &pass, &pass_confirmation, &psql_client).await){

                    
                    Panel::clear_panel();
                    println!(" ✅ {}'s WALLET ADDED SUCCESSFULLY!!", username);
                    thread::sleep(Duration::from_secs(2));
                    continue;

                }
                else {
                    
                    continue;

                }
                

            }

            MenuOption::EXIT => {

                Panel::clear_panel();
                println!("[MESSAGE] : PROGRAM FINISHED!");
                program_status = NOT_RUNNING;

            }

            MenuOption::INVALID_OPTION => {

                Panel::clear_panel();
                println!("[ALERT] : INVALID OPTION!");
                thread::sleep(Duration::from_secs(2));
                continue;

            }

            _ => {

                //Other conditions are ignored

            }

        }
         
        program_status = NOT_RUNNING;

    }
    //------------>End of Program<---------------------


}
