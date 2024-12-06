mod enums;
mod entities;

use enums::{menu_option::MenuOption, prog_status::ProgramStatus::{RUNNING as RUNNING, NOT_RUNNING as NOT_RUNNING}};
use entities::services::panel::Panel;


fn main() {
    
    //----------->Main program loop<-----------------
    let mut program_status = RUNNING;
    while program_status == RUNNING {

        Panel::generate_panel(MenuOption::MAIN_MENU);
        program_status = NOT_RUNNING;

    }
    //------------>End of Program<---------------------


}
