use std::fmt::{self, Write};
// use crate::Opt;

use libamdgpu_top::NpuMetrics;
use libamdgpu_top::xdna::XdnaFdInfoStat;

const PROC_NAME_LEN: usize = 16;
const PID_MAX_LEN: usize = 7; // 2^22

const MEMORY_LABEL: &str = "MEM";
const NPU_LABEL: &str = "NPU";

use crate::AppTextView;

impl AppTextView {
    // pub const XDNA_FDINFO_TITLE: &str = "XDNA fdinfo";

    pub fn print_xdna_fdinfo(
        &mut self,
        stat: &mut XdnaFdInfoStat,
        npu_metrics: &Option<NpuMetrics>,
    ) -> Result<(), fmt::Error> {
        self.text.clear();

        if let Some(npu_metrics) = npu_metrics {
            write!(
                self.text.buf,
                " MPNPU:{:4}MHz NPU:{:4}MHz {:5}mW Read:{:5}MB/s Write:{:5}MB/s",
                npu_metrics.mpnpuclk_freq,
                npu_metrics.npuclk_freq,
                npu_metrics.npu_power,
                npu_metrics.npu_reads,
                npu_metrics.npu_writes,
            )?;

            write!(self.text.buf, " [")?;

            for busy in npu_metrics.npu_busy.iter() {
                write!(self.text.buf, "{busy:3}%,")?;
            }

            let _ = self.text.buf.pop(); // remove ','
            writeln!(self.text.buf, "]")?;
        }

        writeln!(
            self.text.buf,
            " {proc_name:<PROC_NAME_LEN$}|{pid:^PID_MAX_LEN$}|{MEMORY_LABEL:^6}|{NPU_LABEL:^4}|",
            proc_name = "Name",
            pid = "PID",
        )?;

        self.print_xdna_fdinfo_usage(stat)?;

        Ok(())
    }

    pub fn print_xdna_fdinfo_usage(&mut self, stat: &XdnaFdInfoStat) -> Result<(), fmt::Error> {
        for pu in &stat.proc_usage {
            let utf16_count = pu.name.encode_utf16().count();
            let name_len = if pu.name.len() != utf16_count {
                PROC_NAME_LEN - utf16_count
            } else {
                PROC_NAME_LEN
            };
            write!(
                self.text.buf,
                " {name:name_len$}|{pid:>PID_MAX_LEN$}|{total:>5}M|",
                name = pu.name,
                pid = pu.pid,
                total = pu.usage.total_memory >> 10,
            )?;

            // write!(self.text.buf, "{:>3}%|", pu.cpu_usage)?;

            for (usage, label_len) in [
                (pu.usage.npu, NPU_LABEL.len()),
            ] {
                write!(self.text.buf, "{usage:>label_len$}%|")?;
            }

            writeln!(self.text.buf)?;
        }

        Ok(())
    }
/*
    pub fn xdna_fdinfo_name(index: usize) -> String {
        format!("{} {index}", Self::XDNA_FDINFO_TITLE)
    }
*/
    // TODO: cb
    // No one has tested the functionality for XDNA
}
