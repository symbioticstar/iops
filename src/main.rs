use std::io::Write;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use chrono::Local;
use clap::Clap;
use log::{info, warn};
use mysql::*;
use mysql::prelude::*;

use bat::CommandOpts;

fn main() -> Result<()> {
    let opts: CommandOpts = CommandOpts::parse();
    let opts = Arc::new(opts);

    env_logger::builder()
        .format(|buf, record| {
            writeln!(buf, "[{}] [{}] {}: {}",
                     Local::now(),
                     thread::current().name().unwrap_or("anonymous".into()),
                     record.level(), record.args())
        })
        .init();

    info!("MySQL test iops");

    let pool = Pool::new(OptsBuilder::new()
        .ip_or_hostname(opts.host.as_ref())
        .user(opts.username.as_ref())
        .pass(opts.password.as_ref())
        .db_name(opts.db.as_ref())
        .tcp_port(opts.port))?;

    info!("Pool ready");

    let mut join_handles: Vec<JoinHandle<()>> = vec![];
    for i in 0..opts.threads {
        let mut conn = pool.get_conn()?;
        let opts = opts.clone();
        let join_handle = thread::Builder::new().name(format!("wk_{:03}", i)).spawn(move || {
            info!("Initiated");
            let mut seq = 1;
            loop {
                for command in opts.commands.iter() {
                    let command = command.replace("{}", i.to_string().as_str());
                    let command = command.replace("{+}", (i + 1).to_string().as_str());
                    let command = command.replace("{-}", (i - 1).to_string().as_str());
                    let start = Instant::now();
                    match conn.query_drop(&command) {
                        Ok(_) => {
                            let span = start.elapsed();
                            println!("{} {}", i, span.as_micros());
                            info!("{}", command)
                        }
                        Err(e) => {
                            println!("{} {}", i, -1);
                            warn!("{}: {:?}", command, e)
                        }
                    }
                }
                if let Some(count) = opts.count {
                    if count == seq {
                        info!("Done");
                        break;
                    }
                }
                seq = seq + 1;
            }
        }).unwrap();
        join_handles.push(join_handle);
    }
    for jh in join_handles {
        jh.join().unwrap()
    }
    Ok(())
}
