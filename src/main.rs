// Config Vars
pub mod config;
pub mod error;

mod decode;
mod privilige;
mod process;
mod state;

use crate::process::{
    get_process_handle, is_target_process, process_iter, santinize, suspend_process_handle,
};

use std::sync::{Arc, RwLock};
use std::thread::sleep;
use std::time::Duration;

use log::{debug, error, info, trace};
use log::{Level, LevelFilter};
use nu_ansi_term::Color;

use ansi_escapes::{
    ClearScreen,
    //CursorSavePosition,
    //CursorRestorePosition,
    EnterAlternativeScreen,
    ExitAlternativeScreen,
};

fn main() -> eyre::Result<()> {
    let running = Arc::new(RwLock::new(true));

    ctrlc::set_handler({
        let running = running.clone();
        move || {
            *running.write().unwrap() = false;
        }
    })
    .expect("Failed to bind handler on `Ctrl+C`");

    // Initialize Jobs : Set Up
    init();

    // Initialize Jobs : Build Target Vector
    let targets = decode::get_prebuilt();
    debug!("Built Target Vector = {:?}", &targets);

    // Main Termination Loop
    let estimated_runs = config::SUSPEND_UNTIL / config::SUSPEND_EACH;
    for iteration in 1..=estimated_runs {
        iteration_init();

        info!("Progress: '{}' out of '{}'", iteration, estimated_runs);

        match do_suspend_targets(&targets).is_successful_run() {
            false => {
                let mut exit = false;

                for _ in 0..config::SUSPEND_EACH {
                    sleep(Duration::new(1, 0));

                    if !*running.read().unwrap() {
                        println!();
                        error!("Got `Ctrl+C` signal!");
                        info!("Proceed to Early Exit...");

                        exit = true;
                        break;
                    };
                }

                if exit {
                    break;
                };
            }
            true => {
                println!();
                info!(
                    "'{}' unique process(es) handled successfully!",
                    config::SUSPEND_UNIQUE_SHOULD
                );
                info!("Proceed to Early Exit...");

                break;
            }
        }
    }

    println!();
    info!(
        "This window will automatically closed after {} second(s)",
        config::IDLE_AFTER_FINISH
    );
    info!("You can close this window manually");
    sleep(Duration::new(config::IDLE_AFTER_FINISH, 0));

    cleanup();
    Ok(())
}

fn init() {
    privilige::elevate();
    let _ = enable_ansi_support::enable_ansi_support();

    env_logger::Builder::new()
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            use std::io::Write;

            let body = match record.level() {
                Level::Error => Color::Fixed(11).paint(record.args().to_string()),
                Level::Warn => Color::Fixed(11).paint(record.args().to_string()),
                Level::Info => Color::Fixed(14).paint(record.args().to_string()),
                Level::Debug => Color::Fixed(8).paint(record.args().to_string()),
                Level::Trace => Color::Fixed(8).paint(record.args().to_string()),
            };
            let level_style = buf.default_level_style(record.level());

            writeln!(
                buf,
                "  [{level_style}{:^7}{level_style:#}] {body}",
                record.level(),
            )
        })
        .init();

    print!("{}", EnterAlternativeScreen,);
}

fn cleanup() {
    print!("{}", ExitAlternativeScreen,);
}

fn iteration_init() {
    print!("{}", ClearScreen,);
    print!("\n");
}

fn do_suspend_targets(targets: &Vec<String>) -> state::SuspendState {
    println!();
    info!("Start scan");
    debug!("");

    let mut suspend_state = state::SuspendState::new();

    for proc in process_iter() {
        // - HELP
        // process_id   : proc.get_pid()
        // process_name : proc.get_pname()
        // process_user : proc.get_user()
        //
        trace!(
            "Process - {} // {} // {}",
            proc.get_pid(),
            proc.get_pname(),
            proc.get_user().unwrap_or("nobody".to_string())
        );

        let proc_name = santinize(&proc);

        if is_target_process(targets, &proc_name) {
            debug!("ProcName = {}", &proc_name);

            if proc.get_user().unwrap_or("nobody".to_string()).as_str() == "access denied:OpenProcess failed" {
                suspend_state.fail_access_denied();
                error!("Access Denied Error");
                continue;
            }

            let proc_handle_result = get_process_handle(proc.get_pid());
            if let Err(e) = proc_handle_result {
                suspend_state.fail_get_handle();
                error!("Error get handle");
                error!("Err: {:?}", e);
                continue;
            }

            let proc_handle = proc_handle_result.unwrap();
            if !suspend_process_handle(proc_handle) {
                suspend_state.fail_suspend_process();
                error!("Handle Error");
                continue;
            }

            suspend_state.success_suspend_process(&proc_name);
            debug!("Handle Success");
        } else {
            suspend_state.no_match();
        }
    }

    // print: current suspend iteration's result status
    suspend_state.display();

    // return: suspended process count exceeded `config::SUSPEND_UNIQUE_SHOULD` threshold or not
    suspend_state
}
