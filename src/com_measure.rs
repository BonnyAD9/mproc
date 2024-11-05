use std::{
    fmt::Display,
    io::{self, IsTerminal},
    process::Command,
    time::Duration,
};

use termal::{formatmc, writemcln};

use crate::{err::Result, get_mem_string, measurement::Measurement};

pub struct ComMeasure {
    cmd: Command,
    atempts: usize,
    measured: usize,
    total_time: Duration,
    best_time: Duration,
    worst_time: Duration,
    success: usize,
    failure: usize,
    memory_cnt: usize,
    total_memory: usize,
    best_memory: usize,
    worst_memory: usize,
}

impl ComMeasure {
    pub fn new(cmd: Command) -> Self {
        Self {
            cmd,
            atempts: 0,
            measured: 0,
            total_time: Duration::ZERO,
            best_time: Duration::MAX,
            worst_time: Duration::ZERO,
            success: 0,
            failure: 0,
            memory_cnt: 0,
            total_memory: 0,
            best_memory: usize::MAX,
            worst_memory: 0,
        }
    }

    pub fn measure(&mut self) -> Result<()> {
        self.atempts += 1;

        let m = Measurement::measure(&mut self.cmd)?;

        self.measured += 1;
        self.total_time += m.time;
        self.best_time = self.best_time.min(m.time);
        self.worst_time = self.worst_time.max(m.time);

        match m.exit_code {
            Some(0) => self.success += 1,
            Some(_) => self.failure += 1,
            _ => {}
        }

        let m = m.memory?;

        self.memory_cnt += 1;
        self.total_memory += m;
        self.best_memory = self.best_memory.min(m);
        self.worst_memory = self.worst_memory.max(m);

        Ok(())
    }
}

impl Display for ComMeasure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color = f
            .precision()
            .map(|p| p != 0)
            .unwrap_or_else(|| io::stderr().is_terminal());

        if f.sign_minus() {
            writemcln!(
                f,
                color,
                "
{'gr}===============<< {'y}mproc results {'gr}>>==============={'_}"
            )?;
        }

        let (ds, bs) = if self.failure == 0 {
            (formatmc!(color, "{'dg}"), formatmc!(color, "{'g bold}"))
        } else {
            (formatmc!(color, "{'dr}"), formatmc!(color, "{'r bold}"))
        };

        let dmeasured = self.measured.max(1) as u32;
        let dsc = (self.success + self.failure).max(1) as f32;
        let dmem = self.memory_cnt.max(1);

        writemcln!(
            f,
            color,
            "Runs   : {'w bold}{measured}{'_}/{atempts}
{ds}Success: {bs}{'bold}{sc_rate} {'_}{ds}({success}/{sc_total})

{'dm}Avg time  : {'m bold}{avg_time:?} {'_ dm}({total_time:?}/{measured}){'_}
{'dm}Best time : {'m bold}{best_time:?}{'_}
{'dm}Worst time: {'m bold}{worst_time:?}{'_}

{'dc}Avg memory  : {'c bold}{avg_memory} {'_ dc}({total_memory}/{measured}){'_}
{'dc}Best memory : {'c bold}{best_memory}{'_}
{'dc}Worst memory: {'c bold}{worst_memory}{'_}
",
            measured = self.measured,
            atempts = self.atempts,
            sc_rate = self.success as f32 / dsc,
            success = self.success,
            sc_total = self.success + self.failure,
            avg_time = self.total_time / dmeasured,
            total_time = self.total_time,
            best_time = self.best_time,
            worst_time = self.worst_time,
            avg_memory = get_mem_string(self.total_memory / dmem),
            total_memory = get_mem_string(self.total_memory),
            best_memory = get_mem_string(self.best_memory),
            worst_memory = get_mem_string(self.worst_memory),
        )?;

        Ok(())
    }
}
