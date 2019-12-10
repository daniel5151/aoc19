use crate::prelude::*;

use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Barrier};
use std::thread;

#[derive(Debug)]
enum AmpCommand {
    Run,
    Reset { phase: isize },
}

enum AmpResponse {
    Halted(isize),
}

struct Amp {
    id: usize,
    intcode: Intcode,

    loopback: Sender<isize>,
    input: Receiver<isize>,
    output: Sender<isize>,

    cmd_chan: Receiver<AmpCommand>,
    res_chan: Sender<AmpResponse>,

    sync: Arc<Barrier>,
}

impl Amp {
    fn run(mut self) -> DynResult<()> {
        let input = self.input;
        let output = self.output;

        let _ = self.id;

        loop {
            let msg = self.cmd_chan.recv()?;
            match msg {
                AmpCommand::Run => {
                    let mut last_output = 0;
                    while self.intcode.step(
                        || {
                            let i = input.recv()?;
                            // eprintln!("[{}] <---- {}", id, i);
                            Ok(i)
                        },
                        |i| {
                            // eprintln!("[{}] ----> {}", id, i);
                            last_output = i;
                            output.send(i)?;
                            Ok(())
                        },
                    )? {}

                    // eprintln!("[{}] Halted", id);

                    self.res_chan.send(AmpResponse::Halted(last_output))?;
                }
                AmpCommand::Reset { phase } => {
                    // drain any input that might be left over from the last run
                    input.try_iter().for_each(drop);
                    // .for_each(|e| eprintln!("[{}] > {:?}", id, e));

                    self.intcode.reset();
                    self.loopback.send(phase)?;

                    // eprintln!("[{}] Reset", id);

                    self.sync.wait();
                }
            }
        }
    }
}

pub fn q2(input: String, _args: &[String]) -> DynResult<(isize, Vec<isize>)> {
    let base_intcode = Intcode::new(input)?;

    // Setup

    let mut threads = Vec::new();

    let sync = Arc::new(Barrier::new(6));
    let mut cmd_chans = Vec::new();
    let mut res_chans = Vec::new();

    let (input_chans, mut output_chans): (Vec<_>, Vec<_>) = (0..5)
        .map(|_| {
            let (tx, rx) = mpsc::channel::<isize>();
            (tx, Some(rx))
        })
        .unzip();

    for id in 0..5 {
        let (cmd_tx, cmd_rx) = mpsc::channel::<AmpCommand>();
        let (res_tx, res_rx) = mpsc::channel::<AmpResponse>();
        cmd_chans.push(cmd_tx);
        res_chans.push(res_rx);

        let intcode = base_intcode.clone();
        let sync = sync.clone();

        let loopback = input_chans[id].clone();
        let input = output_chans[id].take().unwrap();
        let output = input_chans[(id + 1) % 5].clone();

        threads.push(thread::spawn(move || -> Result<(), String> {
            Amp {
                id,
                intcode,

                loopback,
                input,
                output,

                cmd_chan: cmd_rx,
                res_chan: res_tx,

                sync,
            }
            .run()
            .map_err(|e| format!("[Amp {}]: {}", id, e))
        }))
    }

    // Start the tests
    let mut max_out = (std::isize::MIN, Vec::new());
    for phases in (5..10).permutations(5) {
        // seed the amps with their phase
        for (c, &phase) in cmd_chans.iter().zip(phases.iter()) {
            c.send(AmpCommand::Reset { phase })?;
        }

        sync.wait();

        // start the amps
        for c in &cmd_chans {
            c.send(AmpCommand::Run)?;
        }

        input_chans[0].send(0)?;

        // Run until all amps are halted
        let final_outputs = res_chans
            .iter()
            .map(|c| c.recv())
            .collect::<Result<Vec<_>, _>>()?;

        // we only care about the final amp in the series
        let AmpResponse::Halted(out) = final_outputs[4];

        // calculate the new maximum
        if max_out.0 < out {
            max_out = (out, phases);
        }
    }

    Ok(max_out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn q2_e1_threaded() {
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,
            28,-1,28,1005,28,6,99,0,0,5";
        let output = q2(input.to_string(), &[]).unwrap();
        assert!(output == (139629729, vec![9, 8, 7, 6, 5]));
    }

    #[test]
    fn q2_e2_threaded() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,
            1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,
            55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let output = q2(input.to_string(), &[]).unwrap();
        assert!(output == (18216, vec![9, 7, 8, 5, 6]));
    }
}
