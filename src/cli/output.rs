use termal::formatmc;

use crate::{com_measure::ComMeasure, err::Result, measurement::Measurement};

use super::{ColorMode, OutputType};

#[derive(Debug)]
pub struct Output {
    pub color: bool,
    pub out: OutputType,
}

impl Output {
    pub fn new(out: OutputType, color: ColorMode) -> Self {
        Self {
            color: out.color(color),
            out,
        }
    }

    pub fn print_measurement(
        &mut self,
        measurement: &Measurement,
    ) -> Result<()> {
        let color = self.color as usize;
        let s = format!("{measurement:-.color$}\n");
        self.out.print(s)
    }

    pub fn print_res_with(&mut self, n: usize, r: Result<()>) -> Result<()> {
        let s = match r {
            Ok(()) => String::new(),
            Err(e) => formatmc!(
                self.color,
                "\nmproc: {'r}Failed to measure {n}: {'_}{e}\n"
            ),
        };
        self.out.print(s)
    }

    pub fn print_com_measure(&mut self, cm: &ComMeasure) -> Result<()> {
        let color = self.color as usize;
        let s = format!("{cm:-.color$}");
        self.out.print(s)
    }
}
