use std::process::{Command, Output};
use std::sync::mpsc;
use std::thread;

use crate::util::notify;

pub fn ctrlc(file_name: String) {
    thread::spawn(move || -> anyhow::Result<()> {
        let (tx, rx) = mpsc::channel();

        ctrlc::set_handler(move || tx.send(()).unwrap())?;

        rx.recv()?;

        println!();

        if let Ok(Output { .. }) = Command::new("killall").arg(&file_name).output() {
            notify(&format!("Kill request received for {}.", &file_name), None);

            return Ok(());
        }

        notify(
            &format!(
                "Error while trying to kill {}. Try killing from htop.",
                &file_name
            ),
            None,
        );

        Ok(())
    });
}
