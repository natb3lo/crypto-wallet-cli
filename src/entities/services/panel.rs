use std::{fmt::Error, io::{self, stdout}, num::ParseIntError};

use crate::enums::menu_option::MenuOption;
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

            MenuOption::EXIT => {



            }

            MenuOption::INVALID_OPTION => {



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

                execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(19, 1)).unwrap();
                io::stdin().read_line(&mut buffer).unwrap();
                //execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();
        
                username = buffer.clone();
                buffer.clear();
                        
                //execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(19, 2)).unwrap();
                //io::stdin().read_line(&mut buffer).unwrap();
                buffer = read_password().unwrap();
                //execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();
                password = buffer.clone();
        
                buffer.clear();
                //execute!(output, cursor::SavePosition).unwrap();
                execute!(output, cursor::MoveTo(19, 3)).unwrap();
                //io::stdin().read_line(&mut buffer).unwrap();
                buffer = read_password().unwrap();
                execute!(output, cursor::RestorePosition).unwrap();
                buffer = buffer.trim().to_string();
                password_confirmation = buffer.clone();
        
                buffer.clear();
                execute!(output, cursor::MoveTo(10, 8)).unwrap();
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

    pub fn clear_panel(){
        
        print!("\x1B[2J\x1B[1;1H");
        /*
        let mut output = stdout();
        execute!(output, Clear(ClearType::All)).unwrap();
        */

    }


}