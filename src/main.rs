use std::io::Write;
use std::thread;
use std::thread::sleep;

use chrono::Local;
use clap::Clap;
use log::{error, info, warn};
use mysql::*;
use mysql::prelude::*;

use iops_bench::CommandOpts;

fn main() -> Result<()> {
    let opts: CommandOpts = CommandOpts::parse();
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
        .ip_or_hostname(opts.host)
        .user(opts.username)
        .pass(opts.password)
        .db_name(opts.db)
        .tcp_port(opts.port))?;

    info!("Pool ready");

    for i in 0..opts.threads {
        let mut conn = pool.get_conn()?;
        thread::Builder::new().name(format!("wk_{:02}", i)).spawn(move || {
            info!("Initiated");
            if let Ok(()) = conn.query_drop("select 1;") {
                info!("Ok")
            } else {
                warn!("Failed")
            }
        }).unwrap();
    }
    info!("Start sleep");
    let duration = std::time::Duration::from_secs(opts.secs);
    sleep(duration);
    Ok(())
}
