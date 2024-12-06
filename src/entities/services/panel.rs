use crate::enums::menu_option::MenuOption;
use crossterm::terminal::size;

pub struct Panel;

impl Panel{

    pub fn generate_panel(menu_option: MenuOption){

        let (total_terminal_columns, _) = size().unwrap();

        match menu_option {
            
            MenuOption::MAIN_MENU => {

                print!("\x1B[2J\x1B[1;1H");

                let title = String::from("CRYPTO WALLET");

                let mut column: u16 = 0;
                let mut row: u16 = 0;
                while row <= 4 {

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
                                print!("(1) - {:<15}", "ACCESS WALLET");
                                let adjust = format!("(1) - {:<15}", "ACCESS WALLET");
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
                                print!("(2) - {:<15}", "CREATE WALLET");
                                let adjust = format!("(2) - {:<15}", "CREATE WALLET");
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
                                print!("(0) - {:<15}", "EXIT");
                                let adjust = format!("(3) - {:<15}", "QUIT");
                                column += adjust.len() as u16 - 1;
                                
                            }
                            else if column == total_terminal_columns - 1{
                                print!("");
                            }

                        }
                        
                        column += 1;

                    }

                    row += 1;
                    column = 0;
                    println!("");

                }

                //stdout().flush().unwrap();

            }

            MenuOption::LOGIN => {



            }

            MenuOption::CREATE_WALLET => {



            }

            MenuOption::INVALID_OPTION => {



            }

            MenuOption::QUIT => {



            }


        }

    }

    /*
    
    pub fn get_user_input(menu_option: &MenuOption, ) -> String{
        
        let mut user_input = String::new();

        match menu_option {
            
            MenuOption::LOGIN => {
                
            
            
        }
        
        MenuOption::CREATE_WALLET => {
            
        
    }

    MenuOption::INVALID_OPTION => {
        

            }

            MenuOption::QUIT => {
                
        }
    }
    
}

*/


}