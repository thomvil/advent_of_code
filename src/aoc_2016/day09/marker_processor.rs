lazy_static! {
    static ref MARKER_RE: Regex = Regex::new(r"([.]*)\((\d+)x(\d+)\)(.+)").unwrap();
}

use regex::*;
use std::ops::Range;

#[derive(Debug)]
struct MarkerInfo {
    pub repeat_number: usize,
    pub head_range: Range<usize>,
    pub repeat_range: Range<usize>,
    pub tail_range: Range<usize>,
}

impl MarkerInfo {
    fn process_capture(cap: Captures) -> MarkerInfo {
        let tail_start = cap.get(4).unwrap().start();
        let repeat_length: usize = cap.get(2).unwrap().as_str().parse().unwrap();
        MarkerInfo {
            repeat_number: cap.get(3).unwrap().as_str().parse().unwrap(),
            head_range: (0..cap.get(1).unwrap().end()),
            repeat_range: (tail_start..tail_start + repeat_length),
            tail_range: (tail_start + repeat_length..cap.get(0).unwrap().end()),
        }
    }
}

pub fn decompress(instr: &str) -> String {
    match MARKER_RE.captures(instr) {
        None => instr.to_string(),
        Some(cap) => {
            let mi = MarkerInfo::process_capture(cap);
            let head = &instr[mi.head_range];
            let repeat = &instr[mi.repeat_range].repeat(mi.repeat_number);
            let tail = &instr[mi.tail_range];
            format!("{}{}{}", head, repeat, decompress(tail))
        }
    }
}

pub fn decompressed_length_full(instr: &str) -> usize {
    match MARKER_RE.captures(instr) {
        None => instr.len(),
        Some(cap) => {
            let mi = MarkerInfo::process_capture(cap);
            let head = mi.head_range.end;
            let repeat = &instr[mi.repeat_range].repeat(mi.repeat_number);
            let tail = &instr[mi.tail_range];
            head + decompressed_length_full(repeat) + decompressed_length_full(tail)
        }
    }
}

pub fn decompressed_length(instr: &str) -> usize {
    match MARKER_RE.captures(instr) {
        None => instr.len(),
        Some(cap) => {
            let mi = MarkerInfo::process_capture(cap);
            let head = mi.head_range.end;
            let repeat = mi.repeat_range.len() * mi.repeat_number;
            let tail = &instr[mi.tail_range];
            head + repeat + decompressed_length(tail)
        }
    }
}
