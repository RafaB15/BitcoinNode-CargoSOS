use super::{error_tui::ErrorTUI, menu_option::MenuOption};

use cargosos_bitcoin::logs::logger_sender::LoggerSender;

use std::io::stdin;

/// Get the option from the user via terminal
///
/// ### Error
///  * `ErrorTUI::TerminalReadFail`: It will appear when the terminal read fails
pub fn select_option(logger: LoggerSender) -> Result<MenuOption, ErrorTUI> {
    println!("Select an option:");
    MenuOption::print_all();

    let mut option: String = String::new();
    if stdin().read_line(&mut option).is_err() {
        return Err(ErrorTUI::TerminalReadFail);
    }

    loop {
        let _: MenuOption = match MenuOption::try_from(option.trim()) {
            Ok(result) => return Ok(result),
            Err(error) => {
                let _ =
                    logger.log_wallet(format!("Put an invalid option, with error: {:?}", error));

                option.clear();
                println!("Error, please enter a valid option:");
                MenuOption::print_all();
                if stdin().read_line(&mut option).is_err() {
                    return Err(ErrorTUI::TerminalReadFail);
                }
                continue;
            }
        };
    }
}
