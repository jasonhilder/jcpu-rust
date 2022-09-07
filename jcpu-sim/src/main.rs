/*


    This file is largly just for UI rendering so that we can see
    what's happening under the hood. All interesting stuff happens in Sim
    and the related XCPU lib


*/


pub mod sim;

use sim::Sim;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// use xcpu::cpu::register;
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap, Table, Row, Cell},
    Terminal,
};  


fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {

    let mut sim: Sim = Sim::new();
 
    sim.start();

    loop {
        terminal.draw(|f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();

            let info_cpu = sim.get_cpu_info();
            let info_cpu_data = sim.get_cpu_details();
            let info_ram = sim.get_ram_info();
            let info_mb = sim.get_mb_info();

            // -----------------------------------------------------------------
            // Surrounding block
            let block = Block::default()
                .title("CPU SIM")
                .title_alignment(Alignment::Center);
            f.render_widget(block, size);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .horizontal_margin(2)
                .vertical_margin(1)
                .split(f.size());



            // -----------------------------------------------------------------
            // Top two inner blocks
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[0]);


            // -----------------------------------------------------------------
            // Top left inner block
            let info_block = Block::default().title("CPU INFO").borders(Borders::ALL);

            let mut text = Vec::new();
            for d in info_cpu.iter() {
                text.push(Spans::from(Span::styled(format!("{}: {}", d.0, d.1), Style::default().fg(Color::Red))));
            } 

            let paragraph = Paragraph::new(text)
                .block(info_block)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, top_chunks[0]);



            // -----------------------------------------------------------------
            // Top right inner block
            let table_block = Block::default().title("CPU TABLE").borders(Borders::ALL);
 
            // table
            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let normal_style = Style::default().bg(Color::Gray);
            let header_cells = ["Regiser", "Value"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
            let header = Row::new(header_cells)
                .style(normal_style)
                .height(1)
                .bottom_margin(1);
            let rows = info_cpu_data.iter().map(|item| {  
                let cells = vec![ Cell::from(item.0.clone()), Cell::from(item.1.clone()) ];

                Row::new(cells).height(1 as u16).bottom_margin(0)
            });
            let t = Table::new(rows)
                .header(header)
                .block(table_block)
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);

            f.render_widget(t, top_chunks[1]);



            // -----------------------------------------------------------------
            // Bottom two inner blocks
            let bottom_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
                .split(chunks[1]);



            // -----------------------------------------------------------------
            // Bottom left block (RAM)
            let ram_block = Block::default().title("RAM INFO").borders(Borders::ALL);

            // ram text input
            let bin_data = info_ram;

            let mut x:Vec <Span> = Vec::new();
            (0..bin_data.len()).for_each(|i| {
                // TUI strips newlines in spans...
                // if i % 15 == 0 && i != 0 {
                //     x.push(Span::raw("\r\n"));
                // }
                let mut color = Color::White;

                if i == (sim.mb.cpu.reg_mar as usize) {
                    color = Color::Red;
                } else if i == (sim.mb.cpu.reg_mar as usize) + 1 {
                    color = Color::Green;
                } else if i == (sim.mb.cpu.reg_iar as usize) { 
                    color = Color::Yellow;
                }

                x.push(Span::styled(format!("{:02x} ", bin_data[i]), Style::default().fg(color)));
            });

            let r_paragraph = Paragraph::new(Spans::from(x)).block(ram_block).alignment(Alignment::Left).wrap(Wrap { trim : true});

            f.render_widget(r_paragraph, bottom_chunks[0]);



            // -----------------------------------------------------------------
            // Bottom right block (MB INFO)
            let mb_block = Block::default()
                .title(Span::styled(
                    "MB INFO", Style::default().fg(Color::White)
                ))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL);
 
            // table
            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let normal_style = Style::default().bg(Color::Gray);
            let header_cells = ["Key", "Value"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
            let header = Row::new(header_cells)
                .style(normal_style)
                .height(1)
                .bottom_margin(1);
            let rows = info_mb.iter().map(|item| {
                let cells = vec![ Cell::from(item.0.clone()), Cell::from(item.1.clone()) ];

                Row::new(cells).height(1 as u16).bottom_margin(0)
            });
            let t = Table::new(rows)
                .header(header)
                .block(mb_block)
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);

            f.render_widget(t, bottom_chunks[1]);

        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Char('r') => sim.reset(), 
                KeyCode::Char('c') => {
                    sim.cycle();
                },
                _ => println!("[App] Unknown key command: {:?}", key.code),
            }
        }
    }
} 