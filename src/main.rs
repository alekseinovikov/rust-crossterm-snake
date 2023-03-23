use std::io::{stdout, Write};

use crossterm::{cursor, event, ExecutableCommand, execute, QueueableCommand, Result, style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor}, style, terminal};
use crossterm::style::Stylize;

fn main() -> Result<()> {
    //enable raw mode
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    let (size_x, size_y) = terminal::size()?;

    //clear terminal
    execute!(stdout, terminal::Clear(terminal::ClearType::All))?;
    //set cursor to 0,0
    execute!(stdout, cursor::MoveTo(0, 0))?;
    //draw horizontal border around terminal
    for i in 0..size_x - 5 {
        execute!(stdout, cursor::MoveTo(i, 0), Print("-"))?;
        execute!(stdout, cursor::MoveTo(i, size_y - 5 - 1), Print("-"))?;
    }
    //draw vertical border around terminal
    for i in 0..size_y - 5 {
        execute!(stdout, cursor::MoveTo(0, i), Print("|"))?;
        execute!(stdout, cursor::MoveTo(size_x - 5 - 1, i), Print("|"))?;
    }
    //set cursor in the center
    execute!(stdout, cursor::MoveTo(size_x / 2 - 5, size_y / 2 - 5))?;

    //wait for key press


    Ok(())
}
