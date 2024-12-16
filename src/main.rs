mod enums;
mod entities;

use enums::{menu_option::MenuOption, prog_status::{ProgramStatus::{RUNNING as RUNNING, NOT_RUNNING as NOT_RUNNING}, LoggedOption}};
use entities::{concrete::{token::TokenInDatFile, wallet, walletoken::Walletoken}, services::{api_interaction::WebInteraction, app_files::ApplicationFiles, db_connection::DbConnection, panel::Panel, validate_registration::Validation}};
use std::{fs, process::exit, thread::{self, sleep}};
use std::time::Duration;
use std::{env, process};
use tokio_postgres::{error, NoTls};
use std::fs::{OpenOptions, create_dir_all};
use entities::services::app_files::ProgramFileCounter;

#[tokio::main]
async fn main() {
    
    let mut program_status = RUNNING;
    let mut db_ip = String::new();
    let mut db_url = String::new();
    
    let args: Vec<String> = env::args().collect();
    if args.len() != 2{
        
        Panel::clear_panel();
        println!("🚨: PROGRAM NEEDS THE DATABASE'S IP ADRESS AS ARGUMENT [xxx.xxx.xxx.xxx]!");
        //process::exit(1);
        return

    }
    else {
        db_ip = args[1].clone();
        db_url = format!("postgres://postgres:natunix.23@{}:5432/walletproject", db_ip);
    }
    
    //thread::sleep(Duration::from_secs(2));
    let counter = ApplicationFiles::update_file_counter().await;
    let mut index_file = match ApplicationFiles::get_index_file(counter).await{

        Ok(f) => f,

        Err(_error) => {

            println!("[ERROR] : GENERATION OF <app_files\\index.dat> HAS FAILED!");
            process::exit(1);

        }

    };
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
                println!("[ ERROR 🚨 ] : ONLY NUMBERS ARE ALLOWED!");
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

                    Err(_error) => {
                        Panel::clear_panel();
                        continue;
                    }
                };

                if DbConnection::exists_user_wallet_relation(&psql_client, &username, &password).await{

                    //USUÁRIO LOGADO...
                    //println!("USUÁRIO LOGADO...");
                    let (owner, mut wallet) = DbConnection::get_owner_wallet_ralation(&psql_client, &username).await;

                    let mut is_logged = true;
                    while is_logged{

                        Panel::clear_panel();
                        Panel::generate_logged_user_panel(&owner, &wallet);
                        let logged_option = Panel::get_logged_user_input();
    
                        match logged_option {
                            
                            LoggedOption::Buy => {
                                Panel::clear_panel();
                                
                                Panel::get_buy_panel();
                                let buyer_option = Panel::get_input_from_search_panel();
                                //println!("input: {}", buyer_option);
                                //thread::sleep(Duration::from_secs(3));

                                if buyer_option != "r".to_string(){

                                    //find in index file... PROBLEM!!!
                                    let token_dat_fmt = match ApplicationFiles::search_in_index_file(&mut index_file, &buyer_option){

                                        Ok(tk) => tk,

                                        Err(_error) => {

                                            println!("[ ALERT ⚠️ ] : TOKEN NOT FOUND!");
                                            continue;

                                        }

                                    };

                                    let mut token = WebInteraction::get_token(&token_dat_fmt.id).await;
                                    //Panel::clear_panel();
                                    //println!("Afetr get token = {:?}", token);
                                    //thread::sleep(Duration::from_secs(4));
                                    if token.name != "".to_string(){
                                        
                                        //println!("Token: {:?}", token);
                                        Panel::clear_panel();
                                        Panel::buy_token_panel(&token, &wallet);
                                        //thread::sleep(Duration::from_secs(3));
                                        let input =Panel::get_input_from_buy_panel();

                                        //teste
                                        /*
                                        Panel::clear_panel();
                                        println!("Input from buy panel: {}", input);
                                        thread::sleep(Duration::from_secs(4));
                                        */

                                        if input != "r".to_string(){
                                            
                                            let order: f64 = match input.trim().parse(){
    
                                                Ok(value) => value,
    
                                                Err(error) => {
    
                                                    Panel::clear_panel();
                                                    println!("[ ALERT ⚠️ ] : THAT'S NOT A NUMBER!");
                                                    thread::sleep(Duration::from_secs(2));
                                                    continue;
                                                }
    
                                            };

                                            //teste
                                            /*
                                            Panel::clear_panel();
                                            println!("Input from buy panel after convertion: {}", order);
                                            thread::sleep(Duration::from_secs(4));
                                            */

                                            if order <= wallet.balance{

                                                let wallettoken = Walletoken::acquisition(order, &mut wallet, &token, &psql_client).await;
                                                Panel::clear_panel();
                                                println!("[ MESSAGE 🎆 ] : THIS IS THE WAY!!");
                                                //println!("{:?}", wallettoken);
                                                thread::sleep(Duration::from_secs(2));
                                            }
                                            else {
                                                
                                            }
                                        }
                                        else{
                                            continue;
                                        }

                                    }
                                    else{

                                        println!("[ ERROR 🚨 ] : UNABLE TO GET TOKEN FROM API!");

                                    }

                                }
                                else{
                                    continue;
                                }

                                thread::sleep(Duration::from_secs(2));


                            }

                            LoggedOption::MyCryptos => {

                                let mut loop_continue = true;
                                while loop_continue{

                                    Panel::clear_panel();
                                    let (walletoken_list, token_list) = DbConnection::get_walletoken_list_from_wallet_id(&wallet, &psql_client).await.unwrap();
                                    let str = Panel::show_your_cryptos_panel(&wallet, &walletoken_list, &token_list);

                                    if str == "continue".to_string(){
                                        loop_continue = false;
                                    }
                                    else{
                                        //
                                    }


                                }


                            }

                            LoggedOption::LogOut => {
                                is_logged = false;
                            }
                            LoggedOption::DeleteAccount => {
    
                                Panel::clear_panel();
                                //println!("Future delete option here...");

                                Panel::get_delete_account_panel();
                                let op = Panel::get_delete_account_panel_input();

                                if op == "c".to_string(){

                                    //excluir no BD
                                    DbConnection::delete_user_wallet_relation(&owner, &wallet, &psql_client).await;
                                    Panel::clear_panel();
                                    println!("[ MESSAGE 💬 ] : ACCOUNT DELETED!");
                                    thread::sleep(Duration::from_secs(2));
                                    break;

                                }
                                else if op == "r".to_string() {

                                    continue;

                                }
                                else{

                                    Panel::clear_panel();
                                    println!("[ ALERT ⚠️ ] : INVALID OPTION!");
                                    thread::sleep(Duration::from_secs(2));
                                    continue;

                                }

                                thread::sleep(Duration::from_secs(2));
                            
                            }
                            LoggedOption::InvalidOption => {
                                Panel::clear_panel();
                                println!("[ ALERT ⚠️ ] INVALID OPTION!");
                                thread::sleep(Duration::from_secs(2));
                            }
    
                        }



                    }

                    //thread::sleep(Duration::from);

                }
                else{
                    Panel::clear_panel();
                    println!("[ ALERT ⚠️ ] : USERNAME OR PASSWORD DOES NOT EXIST!");
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

                if Validation::is_create_wallet_valid(&username, &pass, &pass_confirmation, &psql_client).await{

                    
                    Panel::clear_panel();
                    println!(" [ MESSAGE ✅ ] : {}'s WALLET ADDED SUCCESSFULLY!!", username);
                    thread::sleep(Duration::from_secs(2));
                    continue;

                }
                else {
                    
                    continue;

                }
                

            }

            MenuOption::EXIT => {

                Panel::clear_panel();
                println!("[ MESSAGE 💬 ] : PROGRAM FINISHED!");
                program_status = NOT_RUNNING;

            }

            MenuOption::INVALID_OPTION => {

                Panel::clear_panel();
                println!("[ ALERT ⚠️ ] : INVALID OPTION!");
                thread::sleep(Duration::from_secs(2));
                continue;

            }

            _ => {

                //Other conditions are ignored

            }

        }
         
        //program_status = NOT_RUNNING;

    }
    //------------>End of Program<---------------------
    std::mem::drop(index_file);

}
