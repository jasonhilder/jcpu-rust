/*
    This file is largly just for UI rendering so that we can see
    what's happening under the hood. All interesting stuff happens in Sim
    and the related XCPU lib
*/

pub mod sim;

use jcpu::{peripheral::{Keyboard, Screen, get_key_code, Peripheral}, motherboard::{SCREEN_WIDTH, SCREEN_HEIGHT}};
use sim::Sim;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture, Event, KeyCode, read, poll, MouseEventKind, MouseButton},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
// use xcpu::cpu::register;
use std::{error::Error, io, time::Duration};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Paragraph, Wrap, Table, Row, Cell},
    Terminal,
};

pub const FULL: &str = "█";

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
    // create peripherals
    const SCREEN_SIZE: u8 = SCREEN_WIDTH * SCREEN_HEIGHT;
    let screen = Screen {buffer: [0; SCREEN_SIZE as usize]};
    let kb = Keyboard {keys_pressed: vec![]}; 

    let mut sim: Sim = Sim::new();
    
    sim.mb.peripherals.insert("screen", Peripheral::Screen(screen));
    sim.mb.peripherals.insert("keyboad", Peripheral::Keyboard(kb));

    sim.start();

    loop {
        terminal.draw(|f| {
            // Wrapping block for a group
            // Just draw the block and the group on the same area and build the group
            // with at least a margin of 1
            let size = f.size();

            let info_cpu = sim.get_cpu_info();
            let info_cpu_data = sim.get_cpu_details();
            let info_alu_data = sim.get_alu_details();
            let info_ram = sim.get_ram_info();
            let info_mb = sim.get_mb_info();
            let info_dbg = sim.get_dbg_info();
            let info_instructions = sim.get_cpu_instructions_text();

            // -----------------------------------------------------------------
            // Surrounding block
            let block = Block::default()
                .title("CPU SIM")
                .title_alignment(Alignment::Center);
            f.render_widget(block, size);

            let wrapper = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .horizontal_margin(2)
                .vertical_margin(1)
                .split(f.size());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(30)].as_ref())
                .split(wrapper[0]);

            let instruction_container = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(100)])
                .split(wrapper[1]);

            // -----------------------------------------------------------------
            // Top two inner blocks
            let top_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(50)].as_ref())
                .split(chunks[0]);

             // split cpu info block
            let cpu_info_blocks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(top_chunks[0]);

            // split cpu block
            let cpu_blocks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(top_chunks[1]);

            // -----------------------------------------------------------------
            // Top left inner block
            let info_block = Block::default().title("CPU INFO").borders(Borders::ALL);

            let mut text = Vec::new();
            for d in info_cpu.iter() {
                if d.0.contains("mouse") || d.0.contains("UI") {
                    text.push(Spans::from(Span::styled(format!("{}: {}", d.0, d.1), Style::default().fg(Color::White))));
                } else {
                    text.push(Spans::from(Span::styled(format!("{}: {}", d.0, d.1), Style::default().fg(Color::Red))));
                }
            }

            let paragraph = Paragraph::new(text)
                .block(info_block)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, cpu_info_blocks[0]);

            // VGA BUFFER block
            let vga_block = Block::default().title("VGA BUFFER").borders(Borders::ALL);
            let mut text = vec![];

            // for c in 0..screen.buffer.clone().len() {}

            if let Some(Peripheral::Screen(scr)) = sim.mb.peripherals.get("screen") {
                // Do buffer stuff here 
                for h in 0..SCREEN_HEIGHT {
                    let mut vga_b:Vec <Span> = Vec::new();
                
                    for i in 0..SCREEN_WIDTH {
                        let pos = i + ( SCREEN_HEIGHT * h );
                        let c_value = scr.buffer[pos as usize];
                        let color = Color::Rgb(c_value as u8, c_value as u8, c_value as u8);
                        vga_b.push(Span::styled(FULL, Style::default().fg(color)));
                    }
                
                    text.push(Spans::from(vga_b));
                }
            }

            let v_paragraph = Paragraph::new(text).block(vga_block).alignment(Alignment::Left).wrap(Wrap {trim: false});

            f.render_widget(v_paragraph, cpu_info_blocks[1]);

            // -----------------------------------------------------------------
            // Top right inner block
            let table_block = Block::default().title("CPU TABLE").borders(Borders::ALL);

            // table
            let selected_style = Style::default().add_modifier(Modifier::REVERSED);
            let normal_style = Style::default().bg(Color::Gray);
            let header_cells = ["Register", "Value"]
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

            f.render_widget(t, cpu_blocks[0]);

            // Top right inner block
            let table_block_alu = Block::default().title("ALU TABLE").borders(Borders::ALL);

            // table
            let alu_header_cells = ["ALU Flag", "Value"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
            let alu_header = Row::new(alu_header_cells)
                .style(normal_style)
                .height(1)
                .bottom_margin(1);
            let rows = info_alu_data.iter().map(|item| {
                let cells = vec![ Cell::from(item.0.clone()), Cell::from(item.1.clone()) ];

                Row::new(cells).height(1 as u16).bottom_margin(0)
            });
            let t_a = Table::new(rows)
                .header(alu_header)
                .block(table_block_alu)
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);


            f.render_widget(t_a, cpu_blocks[1]);


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

                if i == (sim.mb.cpu.reg_mar as usize) && i == (sim.mb.cpu.reg_iar as usize) {
                    color = Color::Cyan;
                } else if i == (sim.mb.cpu.reg_mar as usize) {
                    color = Color::Red;
                } else if i == (sim.mb.cpu.reg_mar as usize) + 1 {
                    color = Color::Green;
                } else if i == (sim.mb.cpu.reg_iar as usize) {
                    color = Color::Yellow;
                } else if i == (sim.mb.cpu.reg_sp as usize) {
                    color = Color::Magenta;
                }

                x.push(Span::styled(format!("{:02x} ", bin_data[i]), Style::default().fg(color)));
            });

            let r_paragraph = Paragraph::new(Spans::from(x)).block(ram_block).alignment(Alignment::Left).wrap(Wrap { trim : true});

            f.render_widget(r_paragraph, bottom_chunks[0]);



            // -----------------------------------------------------------------
            // Bottom right block (MB INFO)

            // split mb block
            let mb_blocks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(bottom_chunks[1]);

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
            let t_mb = Table::new(rows)
                .header(header)
                .block(mb_block)
                .highlight_style(selected_style)
                .highlight_symbol(">> ")
                .widths(&[
                    Constraint::Percentage(50),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);

            f.render_widget(t_mb, mb_blocks[0]);

            // debug bar
            let dbg_block = Block::default()
                .title(Span::styled(
                    "DEBUG INFO", Style::default().fg(Color::White)
                ))
                .title_alignment(Alignment::Left)
                .borders(Borders::ALL);

            let dbg_paragraph = Paragraph::new(Span::from(info_dbg))
                .block(dbg_block)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });


            f.render_widget(dbg_paragraph, mb_blocks[1]);
            // -----------------------------------------------------------------
            // Bottom Debug bar


            // --------------------------------- --------------------------------
            // Instructions block
            let instruction_block = Block::default().title("CPU INSTRUCTIONS").borders(Borders::ALL);

            let mut text = Vec::new();
            for d in info_instructions.iter() {
                let address = d.split(":").collect::<Vec<&str>>();
                let mut color = Color::White;

                // 15 is the peripheral ram
                if address[0].parse::<usize>().unwrap() == sim.mb.cpu.reg_iar as usize - 15 {
                    color = Color::Cyan;
                }

                text.push(Spans::from(Span::styled(d, Style::default().fg(color))));
            }

            let paragraph = Paragraph::new(text)
                .block(instruction_block)
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, instruction_container[0]);
        })?;

        sim.mb.process_peripherals();

        if poll(Duration::from_millis(16)).unwrap() {
            match read().unwrap() {
                Event::Mouse(me) => {
                    match me.kind {
                        MouseEventKind::Down(btn) => {
                            match btn {
                                MouseButton::Middle => return Ok(()),
                                MouseButton::Right => sim.reset(),
                                MouseButton::Left => {
                                    sim.cycle();
                                    if sim.mb.cpu.clearing {
                                        sim.mb.reset_peripherals()
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                },
                Event::Key(key)  => {
                    // send any key presses to the peripherals
                    // write the keycode pressed to ram
                    match key.code {
                        KeyCode::Backspace => sim.mb.pass_to_peripheral("keyboard", 19), //9
                        KeyCode::Enter => sim.mb.pass_to_peripheral("keyboard", 13), //13
                        KeyCode::Esc => sim.mb.pass_to_peripheral("keyboard", 46), // 46
                        KeyCode::Char(c) => {
                            // get key code ascii
                            let ascii_c = get_key_code(c);
                            sim.mb.pass_to_peripheral("keyboard", ascii_c)
                        }, // pass to peripheral
                        //
                        _ => { }
                    }
                }
                _ => {}
            }
        }
    }
}
