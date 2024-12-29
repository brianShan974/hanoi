#[derive(Clone)]
struct HanoiStackFrame {
    pub pc: usize,
    pub n: usize,
    pub from: char,
    pub to: char,
    pub via: char,
}

pub fn hanoi(n: usize, from: char, to: char, via: char) {
    let mut frames = vec![HanoiStackFrame {
        pc: 0,
        n,
        from,
        to,
        via,
    }];

    while let Some(top_frame) = frames.last() {
        let current_top_frame = top_frame.clone();
        let current_top_index = frames.len() - 1;

        match current_top_frame.pc {
            0 => {
                if current_top_frame.n == 1 {
                    println!("{} -> {}", current_top_frame.from, current_top_frame.to);
                    frames.pop();
                }
            }
            1 => frames.push(HanoiStackFrame {
                pc: 0,
                n: current_top_frame.n - 1,
                from: current_top_frame.from,
                to: current_top_frame.via,
                via: current_top_frame.to,
            }),
            2 => {
                println!("{} -> {}", current_top_frame.from, current_top_frame.to);
            }
            3 => frames.push(HanoiStackFrame {
                pc: 0,
                n: current_top_frame.n - 1,
                from: current_top_frame.via,
                to: current_top_frame.to,
                via: current_top_frame.from,
            }),
            _ => {
                frames.pop();
            }
        }

        if let Some(last_top_frame) = frames.get_mut(current_top_index) {
            last_top_frame.pc += 1;
        }
    }
}
