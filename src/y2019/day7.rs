use crate::y2019::intcode::{Intcode, MemCell, PauseCause};

use itertools::Itertools;

use std::sync::mpsc;
use std::thread::JoinHandle;

fn load_input<L: IntoIterator<Item = S>, S: AsRef<str>>(line_source: L) -> Intcode {
    Intcode::load_program(line_source.into_iter().next().unwrap().as_ref())
}

fn part1(pre_run: &Intcode) -> isize {
    let mut out_max = 0;

    let phases = [0, 1, 2, 3, 4];

    for phases in phases.iter().copied().permutations(5) {
        let mut input = 0;
        for p in phases.into_iter() {
            input = pre_run.clone().run_until_end(&[p, input])[0];
        }
        if input > out_max {
            out_max = input;
        }
    }

    out_max
}

fn run_amp(
    pre_run: &Intcode,
    input: mpsc::Receiver<MemCell>,
    output: mpsc::Sender<MemCell>,
) -> JoinHandle<()> {
    let mut amp = pre_run.clone();
    std::thread::spawn(move || loop {
        match amp.run() {
            PauseCause::Halt => return,
            PauseCause::Input(mem) => *mem = input.recv().unwrap(),
            PauseCause::Output(out) => output.send(out).unwrap(),
        }
    })
}

fn run_amps(pre_run: &Intcode, phases: Vec<MemCell>) -> MemCell {
    let (input1_tx, mut curr_input_rx) = mpsc::channel();

    let mut running = vec![];
    let mut prev_out_tx = input1_tx.clone();
    for &phase in &phases {
        let (curr_out_tx, next_input) = mpsc::channel();
        prev_out_tx.send(phase).unwrap();
        prev_out_tx = curr_out_tx.clone();
        running.push(run_amp(pre_run, curr_input_rx, curr_out_tx));
        curr_input_rx = next_input;
    }

    // println!("Starting test {:?}", phases);
    input1_tx.send(0).unwrap();
    let mut peak_out = 0;
    while let Ok(val) = curr_input_rx.recv() {
        // println!("Received {} in feedback loop", val);
        if val > peak_out {
            peak_out = val;
        }
        if input1_tx.send(val).is_err() {
            break;
        }
    }
    for amp in running {
        let _ = amp.join();
    }
    peak_out
}

fn part2(pre_run: &Intcode) -> isize {
    let mut max = 0;
    for setting in [5, 6, 7, 8, 9].iter().copied().permutations(5) {
        let x = run_amps(pre_run, setting);
        if x > max {
            max = x;
        }
    }
    max
}

#[test]
fn real_data() {
    let d = load_input(crate::load_strings(crate::data_file!()));
    assert_eq!(part1(&d), 24625);
    //assert_eq!(part2(&d), 36497698);
}

#[test]
fn test_data() {}
