use std::time::{Duration, Instant};

pub struct Registers {
    pub v: [u8; 0x10],
    pub pc: u16,
    pub sp: u8,
    pub i: u16,
    pub dt: u8,
    pub st: u8,
    dt_instant: Instant,
    st_instant: Instant,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v: [0x0; 0x10],
            pc: 0x200,
            sp: 0x0,
            i: 0x0,dt: 0x0,
            st: 0x0,
            dt_instant: Instant::now(),
            st_instant: Instant::now(),
        }
    }

    pub fn get_elapsed_time_since_last_dt(&self) -> Duration {
        self.dt_instant.elapsed()
    }

    pub fn get_elapsed_time_since_last_st(&self) -> Duration {
        self.st_instant.elapsed()
    }

    pub fn reset_dt_time(&mut self) {
        self.dt_instant = Instant::now();
    }

    pub fn reset_st_time(&mut self) {
        self.st_instant = Instant::now();
    }
}