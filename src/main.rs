slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_LoginClick(move ||{
        println!("TEst")
    });

    ui.run()?;

    Ok(())
}
