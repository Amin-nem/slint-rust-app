// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use std::fmt::format;
use slint::Weak;

slint::include_modules!();

const TAX_PERCENTAGE: f64 = 0.30;
const OWNER_PERCENTAGE: f64 = 0.55;
const PROFIT_PERCENTAGE: f64 = 0.5;
const OPERATING_PERCENTAGE: f64 = 0.10;


fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;
    let ui_handle: Weak<AppWindow> = ui.as_weak();

    ui.on_divide_income(move |string| {
        let ui = ui_handle.unwrap();
        let num = string.trim().parse::<f64>().unwrap();
        let tax:f64 = num * TAX_PERCENTAGE;
        let owner:f64 = num * OWNER_PERCENTAGE;
        let profit:f64 = num * PROFIT_PERCENTAGE;
        let opx:f64 = num * OPERATING_PERCENTAGE;
        let result = format!("Taxes: {:.2}\nOwner: {:.2}\nProfit: {:.2}\nOperation expenses: {:.2}",tax,owner,profit,opx);
        ui.set_results(result.into())


    });

    ui.run()?;

    Ok(())
}
