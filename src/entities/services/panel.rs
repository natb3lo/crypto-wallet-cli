use std::{fmt::Error, io::{self, stdout}, num::ParseIntError};

use crate::{entities::concrete::{owner::Owner, token::Token, wallet::Wallet, walletoken::{self, Walletoken}}, enums::menu_option::MenuOption};
use crate::enums::prog_status::LoggedOption;
use crossterm::{cursor::{self, MoveTo}, event::read, execute, terminal::{size, Clear, ClearType}};
use std::thread;
use std::time::Duration;
use rpassword::read_password;

pub struct Panel;

impl Panel{

    pub fn generate_panel(menu_option: &MenuOption){

        let (total_terminal_columns, _) = size().unwrap();

        match menu_option {
            
            MenuOption::MAIN_MENU => {

                Panel::clear_panel();

                let title = String::from("CRYPTO WALLET");

                let mut column: u16 = 0;
                let mut row: u16 = 0;
                while row <= 6 {

                    while column < (total_terminal_columns) {

                        if row == 0 || row == 4{

                            if row == 0{

                                if column == (total_terminal_columns - title.len() as u16) / 2{

                                    print!("{}", title);
                                    column += title.len() as u16 - 1;

                                }
                                else{
                                    print!("-");
                                }

                            }
                            else{

                                print!("-");

                            }

                        }
                        else if row == 1{

                            if column == 0{
                                print!("");
                            }
                            else if column == 1{
                                print!("{:<7}{:<3}{:>13}", "(1)","-","ACCESS WALLET");
                                let adjust = format!("{:<7}{:<3}{:<15}", "(1)","-","ACCESS WALLET");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                
                                print!("");
                            }
                            
                            
                        }
                        else if row == 2{

                            if column == 0{
                                print!("");
                            }
                            else if column == 1{
                                print!("{:<7}{:<3}{:<15}", "(2)","-","CREATE WALLET");
                                let adjust = format!("{:<7}{:<3}{:<15}", "(2)","-","CREATE WALLET");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == total_terminal_columns - 1{
                                print!("");
                            }
                            

                        }
                        else if row == 3{

                            if column == 0{
                                print!("");
                            }
                            else if column == 1{
                                print!("{:<7}{:<3}{:<15}", "(0)","-","EXIT");
                                let adjust = format!("{:<7}{:<3}{:<15}", "(3)","-","EXIT");
                                column += adjust.len() as u16 - 1;
                                
                            }
                            else if column == total_terminal_columns - 1{
                                print!("");
                            }

                        }
                        else if row == 5{

                            if column == 0{
                                print!("");
                            }
                            else if column == 1{
                                print!("{:<7}{:<3}{:<15}", "Option",":","( )");
                                let adjust = format!("{:<7}{:<3}{:<15}", "Option",":","( )");
                                column += adjust.len() as u16 - 1;
                                
                            }
                            else if column == total_terminal_columns - 1{
                                print!("");
                            }

                        }
                        else {
                            print!("-");
                        }
                        
                        column += 1;

                    }

                    row += 1;
                    column = 0;
                    println!("");

                }

            }

            MenuOption::LOGIN => {

                let title = "LOGIN";
                let mut row = 0;
                let mut column = 0;
                
                while row < 8{

                    while column < total_terminal_columns{

                        if row == 0{

                            if column == (total_terminal_columns - title.len() as u16) / 2{

                                print!("{}", title);
                                column += title.len() as u16 - 1;

                            }
                            else{
                                print!("-");
                            }

                        }
                        else if row == 1{

                            if column == 0{
                                print!("");
                            }
                            else if column == 1{
                                print!("{:<10}{:<3}", "USERNAME",":");
                                let adjust = format!("{:<10}{:<3}", "USERNAME",":");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                
                                print!("");
                            }

                        }
                        else if row == 2{

                            if column == 0{
                                print!("");
                            }
                            else if column == 1{
                                print!("{:<10}{:<3}", "PASSWORD",":");
                                let adjust = format!("{:<10}{:<3}", "PASSWORD",":");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                
                                print!("");
                            }

                        }
                        else if row == 3{

                            print!("-");

                        }
                        else if row == 4{

                            if column == 0{
                                
                                print!("{:<4}{:<2}{:<7}", "(S)", "-", "Save");
                                let adjust = format!("{:<4}{:<2}{:<7}", "(S)", "-", "Save");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                //empty
                            }
                            else {
                                    
                                print!("");
                            }

                        }
                        else if row == 5{

                            if column == 0{
                                
                                print!("{:<4}{:<2}{:<7}", "(R)", "-", "Return");
                                let adjust = format!("{:<4}{:<2}{:<7}", "(R)", "-", "Return");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                //empty
                            }
                            else {
                                    
                                print!("");
                            }

                        }
                        else if row == 6{

                            print!("-");
                            
                        }
                        else {
                            
                            if column == 0{
                                print!("{:<7}{:<2}{:<15}", "Option",":","( )");
                                let adjust = format!("{:<7}{:<2}{:<15}", "Option",":","( )");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == total_terminal_columns - 1{
                                //empty   
                            }
                            else {
                                print!("");
                            }


                        }

                        column += 1;

                    }

                    column = 0;
                    row += 1;
                    println!("");

                }

            }

            MenuOption::CREATE_WALLET => {

                Panel::clear_panel();
                let title = String::from("CREATE WALLET");
                let mut row = 0;
                let mut column = 0;
                while row < 9{

                    while column < total_terminal_columns {
                        
                        if  row == 0{
                            
                            if column == (total_terminal_columns - title.len() as u16) / 2{

                                print!("{}", title);
                                column += title.len() as u16 - 1;

                            }
                            else{
                                print!("-");
                            }

                        }
                        else if row == 1{


                            if column == 0{
                                print!("");
                            }
                            else if column == 1{
                                print!("{:<17}{:<3}", "USERNAME",":");
                                let adjust = format!("{:<17}{:<3}", "USERNAME",":");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                
                                print!("");
                            }

                            //print!("{:<20}{:<3}", "USERNAME", ":");

                        }
                        else if row == 2 || row == 3{

                            if row == 2{

                                if column == 0{
                                    print!("");
                                }
                                else if column == 1{
                                    print!("{:<17}{:<3}", "PASSWORD",":");
                                    let adjust = format!("{:<17}{:<3}", "PASSWORD",":");
                                    column += adjust.len() as u16 - 1;
                                }
                                else if column == (total_terminal_columns - 1){
                                    
                                    print!("");
                                }

                            }
                            else {

                                if column == 0{
                                    print!("");
                                }
                                else if column == 1{
                                    print!("{:<17}{:<3}", "CONFIRM PASSWORD",":");
                                    let adjust = format!("{:<17}{:<3}", "CONFIRM PASSWORD",":");
                                    column += adjust.len() as u16 - 1;
                                }
                                else if column == (total_terminal_columns - 1){
                                    
                                    print!("");
                                }

                                //print!("{:<20}{:<3}", "CONFIRM PASSWORD", ":");
                            }

                        }
                        else if row == 4 {
                            print!("-");
                        }
                        else if row == 5{

                            if column == 0{
                                
                                print!("{:<4}{:<2}{:<7}", "(S)", "-", "Save");
                                let adjust = format!("{:<4}{:<2}{:<7}", "(S)", "-", "Save");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                //empty
                            }
                            else {
                                    
                                print!("");
                            }
                        }
                        else if row == 6{

                            if column == 0{
                                
                                print!("{:<4}{:<2}{:<7}", "(R)", "-", "Return");
                                let adjust = format!("{:<4}{:<2}{:<7}", "(R)", "-", "Return");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == (total_terminal_columns - 1){
                                //empty
                            }
                            else {
                                    
                                print!("");
                            }

                        }
                        else if row == 7{

                            print!("-");

                        }
                        else if row == 8{

                            if column == 0{
                                print!("{:<7}{:<2}{:<15}", "Option",":","( )");
                                let adjust = format!("{:<7}{:<2}{:<15}", "Option",":","( )");
                                column += adjust.len() as u16 - 1;
                            }
                            else if column == total_terminal_columns - 1{
                                //empty   
                            }
                            else {
                                print!("");
                            }

                        }

                        column += 1;

                    }
                    column = 0;
                    row += 1;
                    println!("");

                }

            }

            _ => {
                
                //ignored

            }


        }

    }

    pub fn get_integer_input(menu_option: &MenuOption) -> Result<i8, ParseIntError>{

        let mut buffer = String::new();
        let mut output = stdout();
        let mut user_option: i8 = 0;
        let mut x_cordinate = 0;
        let mut y_cordinate = 0;

        match menu_option {

            MenuOption::MAIN_MENU => {
                
                execute!(output, cursor::SavePosition).unwrap();
                x_cordinate = 11;
                y_cordinate = 5;
                execute!(output, cursor::MoveTo(x_cordinate, y_cordinate)).unwrap();
                io::stdin().read_line(&mut buffer).unwrap();
                execute!(output, cursor::RestorePosition).unwrap();

                user_option = match buffer.trim().parse() {
                    
                    Ok(number) => number,

                    Err(error) => return Err(error)

                };


            }
            
            MenuOption::LOGIN => {



            }

            MenuOption::CREATE_WALLET => {

                execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(19, 1)).unwrap();
                io::stdin().read_line(&mut buffer).unwrap();
                execute!(output, cursor::RestorePosition).unwrap();

                if buffer == "r".to_string(){

                    return Ok(-1);

                }
                else{



                }
            }

            _ => {
                //INVALID_OPTION and EXIT are ignored.
            }
            

        }

        Ok(user_option)

    }

    pub fn get_str_input(menu_option: &MenuOption) -> Result<(String, String, String), Error>{

        let mut buffer = String::new();
        let mut output = stdout();

        let mut username = String::new();
        let mut password = String::new();
        let mut password_confirmation = String::new();
        
        match menu_option{

            MenuOption::CREATE_WALLET => {

                let title_username = format!("{:<17}{:<3}", "USERNAME",":");
                let title_pass = format!("{:<17}{:<3}", "PASSWORD",":");
                let title_pass_confirm = format!("{:<17}{:<3}", "CONFIRM PASSWORD",":");


                execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(19, 0)).unwrap();
                io::stdin().read_line(&mut buffer).unwrap();
                //execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();
        
                username = buffer.clone();
                buffer.clear();
                        
                //execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(19, 1)).unwrap();
                //io::stdin().read_line(&mut buffer).unwrap();
                buffer = read_password().unwrap();
                //execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();
                password = buffer.clone();
        
                buffer.clear();
                //execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(19, 2)).unwrap();
                //io::stdin().read_line(&mut buffer).unwrap();
                buffer = read_password().unwrap();
                execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();
                password_confirmation = buffer.clone();
        
                buffer.clear();
                execute!(output, cursor::MoveTo(10, 7)).unwrap();
                io::stdin().read_line(&mut buffer).unwrap();
        
                buffer = buffer.trim().to_string().to_lowercase();
                let option = buffer.clone();
        
                if option.to_lowercase() == "r".to_string(){
        
                    return Err(Error);
        
                }
                else {
                        
                    Ok((username, password, password_confirmation))
                    
                }

            }
            MenuOption::LOGIN => {

                execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(12, 1)).unwrap();
                io::stdin().read_line(&mut buffer).unwrap();
                //execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();

                username = buffer.clone();
                buffer.clear();


                //execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(12, 2)).unwrap();
                //io::stdin().read_line(&mut buffer).unwrap();
                buffer = read_password().unwrap();
                //execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();
                password = buffer.clone();
                buffer.clear();


                execute!(output, cursor::MoveTo(10, 7)).unwrap();
                io::stdin().read_line(&mut buffer).unwrap();
                buffer = buffer.trim().to_string().to_lowercase();
                let option = buffer.clone();

                if option.to_lowercase() == "r".to_string(){
                    return Err(Error);
                }
                else{
                    Ok((username, password, "".to_string()))
                }


            }
            _ => {
                Ok((username, password, "".to_string()))
            }

        }


    }

    pub fn get_logged_user_input() -> LoggedOption{
        
        let opt_str = format!("{:<7}{:<2}{:<2}", "Option",":","( )");
        let mut buffer = String::new();
        let mut output = stdout();
       
        execute!(output, cursor::SavePosition).unwrap();
        execute!(output, cursor::MoveTo((opt_str.len() as u16 - 2), 7)).unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        execute!(output, cursor::RestorePosition).unwrap();
        
        buffer = buffer.trim().to_lowercase();
        
        if "b".to_string() == buffer{
            return LoggedOption::Buy;
        }
        else if "l".to_string() == buffer{
            return LoggedOption::LogOut
        }
        else if "d".to_string() == buffer{
            return LoggedOption::DeleteAccount
        }
        else if "m".to_string() == buffer{
            return LoggedOption::MyCryptos
        }
        else{
            LoggedOption::InvalidOption
        }


    }

    pub fn clear_panel(){
        
        print!("\x1B[2J\x1B[1;1H");
        /*
        let mut output = stdout();
        execute!(output, Clear(ClearType::All)).unwrap();
        */

    }

    pub fn generate_logged_user_panel(owner: &Owner, wallet: &Wallet){

        let (total_terminal_columns, _) = size().unwrap();

        Panel::clear_panel();
        let title = format!("{}", wallet.name);
        let title_budget = format!("{}{:.2}", "$", wallet.balance);
        let title_date = format!("{:<5} {:<20}UTC", "Since", wallet.creation_date.format("%Y-%m-%d %H:%M:%S"));
        let mut row = 0;
        let mut column = 0;

        while row < 10{

            while column < total_terminal_columns {

                if  row == 0{
                            
                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title);
                        column += title.len() as u16 - 1;

                    }
                    else{
                        print!("-");
                    }

                }
                else if row == 1 {

                    if column == (total_terminal_columns - title_budget.len() as u16) / 2{

                        print!("{}", title_budget);
                        column += title_budget.len() as u16 - 1;

                    }
                    else{
                        print!(" ");
                    }


                }
                else if row == 2{

                    if column == (total_terminal_columns - title_date.len() as u16) / 2{

                        print!("{}", title_date);
                        column += title_date.len() as u16 - 1;

                    }
                    else{
                        print!(" ");
                    }


                }
                else if row == 3{

                    print!("-");

                }
                else if row == 4{

                    if column == 0{
                                
                        print!("{:<4}{:<2}{:<11}", "(B)", "-", "Buy Cryptos");
                        let adjust = format!("{:<4}{:<2}{:<11}", "(B)", "-", "Buy Cryptos");
                        column += adjust.len() as u16 - 1;
                    }
                    else if column == (total_terminal_columns - 1){
                        //empty
                    }
                    else {
                            
                        print!("");
                    }


                }
                else if row == 5{

                    if column == 0{
                                
                        print!("{:<4}{:<2}{:<11}", "(M)", "-", "My Cryptos");
                        let adjust = format!("{:<4}{:<2}{:<11}", "(M)", "-", "My Cryptos");
                        column += adjust.len() as u16 - 1;
                    }
                    else if column == (total_terminal_columns - 1){
                        //empty
                    }
                    else {
                            
                        print!("");
                    }

                }
                else if row == 6 {

                    if column == 0{
                                
                        print!("{:<4}{:<2}{:<11}", "(L)", "-", "Log Out");
                        let adjust = format!("{:<4}{:<2}{:<11}", "(L)", "-", "Log Out");
                        column += adjust.len() as u16 - 1;
                    }
                    else if column == (total_terminal_columns - 1){
                        //empty
                    }
                    else {
                            
                        print!("");
                    }


                }
                else if row == 7{


                    if column == 0{
                                
                        print!("{:<4}{:<2}{:<14}", "(D)", "-", "Delete Account");
                        let adjust = format!("{:<4}{:<2}{:<14}", "(D)", "-", "Delete Account");
                        column += adjust.len() as u16 - 1;
                    }
                    else if column == (total_terminal_columns - 1){
                        //empty
                    }
                    else {
                            
                        print!("");
                    }



                }
                else if row == 8{

                    print!("-");

                }
                else{

                    if column == 0{
                        print!("{:<7}{:<2}{:<15}", "Option",":","( )");
                        let adjust = format!("{:<7}{:<2}{:<15}", "Option",":","( )");
                        column += adjust.len() as u16 - 1;
                    }
                    else if column == total_terminal_columns - 1{
                        //empty   
                    }
                    else {
                        print!("");
                    }

                }
                column += 1;
            }

            column = 0;
            row += 1;
            println!("");


        }

    }

    pub fn get_buy_panel(){

        let (total_terminal_columns, _) = size().unwrap();

        let title = format!("{} ", "🔎");
        let title_msg = format!("[ TIP💡] : \'r\' TO RETURN...");
        let mut row = 0;
        let mut column = 0;

        while row < 4{

            while column < total_terminal_columns {

                if row == 0 || row == 2{
                    print!("-");
                }
                else if row == 1{

                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title);
                        column += title.len() as u16 - 1;

                    }
                    else{
                        print!(" ");
                    }
                    


                }
                else{

                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title_msg);
                        column += title_msg.len() as u16 - 1;

                    }
                    else{
                        print!(" ");
                    }

                }

                column += 1;

            }

            column = 0;
            row += 1;
            println!("");

        }



    }

    pub fn get_input_from_search_panel() -> String{

        let (total_terminal_columns, _) = size().unwrap();
        let title = format!("{} : ", "🔎");



        let mut buffer = String::new();
        let mut output = stdout();

        execute!(output, cursor::SavePosition).unwrap();
        execute!(output, cursor::MoveTo((total_terminal_columns/2 as u16) + 1, 1)).unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        execute!(output, cursor::RestorePosition).unwrap();
        
        buffer = buffer.trim().to_lowercase();

        buffer


    }

    pub fn buy_token_panel(token: &Token, wallet: &Wallet){

        let (total_terminal_columns, _) = size().unwrap();

        let title = format!("{}🪙 ", token.name);
        let title_mkt_value = format!("💹 ${:.2}", token.mkt_value);
        let title_your_balance = format!("{:<13}{:<2}${:.2}", "Balance", ":",wallet.balance);
        let mut row = 0;
        let mut column = 0;

        while row < 6{

            while column < total_terminal_columns {

                if  row == 0{
                            
                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title);
                        column += title.len() as u16 - 1;

                    }
                    else{
                        print!("-");
                    }
                }
                else if row == 1{

                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title_mkt_value);
                        column += title_mkt_value.len() as u16 - 1;

                    }
                    else{
                        print!(" ");
                    }

                }
                else if row == 2{

                    print!("-");

                }
                else if row == 3{

                    if column == 0{
                        print!("{}", title_your_balance);
                        let adjust = format!("{}", title_your_balance);
                        column += adjust.len() as u16 - 1;
                    }
                    else if column == total_terminal_columns - 1{
                        //empty   
                    }
                    else {
                        print!("");
                    }
                }
                else if row == 4{

                    if column == 0{

                        println!("{:<13}{:<2}$", "Amount to Buy", ":")

                    }

                }
                else if row == 5{

                    if column == 0{
                        println!("");
                        println!("[MESSAGE]: \'r\' TO RETURN...")
                    }

                }
                column += 1;

            }
            column = 0;
            row += 1;
            println!("");


        }


    }

    pub fn get_input_from_buy_panel() -> String{

        let str_size = format!("{:<13}{:<2}$", "Amount to Buy", ":");

        let mut buffer = String::new();
        let mut output = stdout();

        execute!(output, cursor::SavePosition).unwrap();
        execute!(output, cursor::MoveTo((str_size.len() as u16), 3)).unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        execute!(output, cursor::RestorePosition).unwrap();

        buffer

    }

    pub fn get_delete_account_panel(){

        let (total_terminal_columns, _) = size().unwrap();

        let title = format!("{}", "DELETE ACCOUNT");
        let title_confirm = format!("{:<4}{:<2}{:<8}", "(C)", "-","CONFIRM");
        let title_return = format!("{:<4}{:<2}{:<8}", "(R)", "-","RETURN");
        let mut row = 0;
        let mut column = 0;

        while row < 5{

            while column < total_terminal_columns {

                if  row == 0{
                            
                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title);
                        column += title.len() as u16 - 1;

                    }
                    else{
                        print!("-");
                    }
                }
                else if row == 1{

                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title_confirm);
                        column += title_confirm.len() as u16 - 1;

                    }
                    else{
                        print!(" ");
                    }

                }
                else if row == 2{

                    if column == (total_terminal_columns - title.len() as u16) / 2{

                        print!("{}", title_return);
                        column += title_return.len() as u16 - 1;

                    }
                    else{
                        print!(" ");
                    }

                }
                else if row == 3{

                    print!("-");

                }
                else{
                    if column == 0{
                        
                        println!("{:<7}{:<2}", "Option", ":");
                    }
                    else{
                        break;
                    }

                }
                column += 1;
            }
            column = 0;
            row += 1;
            println!("");

        }

    }

    pub fn get_delete_account_panel_input() -> String{

        let str_size = format!("{:<7}{:<2}", "Option", ":");

        let mut buffer = String::new();
        let mut output = stdout();

        execute!(output, cursor::SavePosition).unwrap();
        execute!(output, cursor::MoveTo((str_size.len() as u16), 4)).unwrap();
        io::stdin().read_line(&mut buffer).unwrap();
        execute!(output, cursor::RestorePosition).unwrap();

        buffer.trim().to_string().to_lowercase()


    }

    pub fn show_your_cryptos_panel(wallet: &Wallet, walletoken_list: &Vec<Walletoken>, token_list: &Vec<Token>) -> String{

        let (total_terminal_columns, _) = size().unwrap();
        let mut total_balance = 0.0;

        for walletoken in walletoken_list{

            //println!("{}\n{}", walletoken.token_symbol, walletoken.amount);
            for token in token_list{

                if walletoken.token_symbol == token.symbol{

                    total_balance += walletoken.amount * token.mkt_value;

                }

            }
            

        }
        let total_balance_str = format!("{:.2}", total_balance);
        let mut row = 0;
        let mut column = 0;

        while row < 1{
            while  column < total_terminal_columns {
                if row == 0{
                    if column == (total_terminal_columns - total_balance_str.len() as u16) / 2{

                        print!("${}", total_balance_str);
                        column += total_balance_str.len() as u16 - 1;

                    }
                    else{
                        print!("-");
                    }
                }
                else{
                    print!(" ");
                }
                column += 1;
            
            }
            column = 0;
            row += 1;
            println!("");
        }

        for walletoken in walletoken_list{

            println!("{} - {} tokens", walletoken.token_symbol, walletoken.amount);

        }

        println!("\nType any key to continue...");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        return "continue".to_string();

    }


}