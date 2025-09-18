use crate::config;

use std::collections::BTreeSet;

use log::{debug, info, warn};
use nu_ansi_term::Color;

pub struct SuspendState {
    total: u32,
    tried: u32,
    fail_gethandle: u32,
    fail_accessdenied: u32,
    fail_suspendprocess: u32,
    success_suspend: u32,

    record: BTreeSet<String>,
}

impl SuspendState {
    pub fn new() -> Self {
        SuspendState {
            total: 0,
            tried: 0,
            fail_gethandle: 0,
            fail_accessdenied: 0,
            fail_suspendprocess: 0,
            success_suspend: 0,

            record: BTreeSet::new(),
        }
    }

    fn tried_suspend(&mut self) {
        self.total += 1;
        self.tried += 1;
    }

    pub fn no_match(&mut self) {
        self.total += 1;
    }

    pub fn fail_get_handle(&mut self) {
        self.tried_suspend();
        self.fail_gethandle += 1;
    }

    pub fn fail_access_denied(&mut self) {
        self.tried_suspend();
        self.fail_accessdenied += 1;
    }

    pub fn fail_suspend_process(&mut self) {
        self.tried_suspend();
        self.fail_suspendprocess += 1;
    }

    pub fn success_suspend_process(&mut self, proc_name: &String) {
        self.record.insert(proc_name.to_uppercase());
        self.tried_suspend();
        self.success_suspend += 1;
    }

    pub fn display(&self) {
        let body = Color::Fixed(2).paint(format!("[ {} / {} ]", self.success_suspend, self.total,));
        let tail = Color::Fixed(28).paint(format!("( with {} attempts )", self.tried,));

        info!("Done! {body} {tail}");

        debug!(
            "
            \r  - GetHandle Failed : {}
            \r  - Access Denied    : {}
            \r  - Suspend Failed   : {}
            \r",
            self.fail_gethandle, self.fail_accessdenied, self.fail_suspendprocess
        );

        if !self.is_successful_run() {
            warn!(
                "Only {} unique process handled, some process may not handled",
                self.record.len()
            );
        }
    }

    pub fn is_successful_run(&self) -> bool {
        self.record.len() >= config::SUSPEND_UNIQUE_SHOULD
    }
}
