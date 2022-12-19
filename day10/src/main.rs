use std::fs;

#[derive(Debug)]
struct Cpu {
    register_x: i32,
    cycles: i32,
    total_signal_strength: i32,
    screen: Vec<Vec<char>>
}

impl Cpu {
    fn new() -> Cpu {
        Cpu { register_x: 1, cycles: 0, total_signal_strength: 0, screen: Vec::new()}
    }

    fn process_instruction(&mut self, instruction: Vec<&str>) {
        match instruction[0] {
            "noop" => {
                self.run_cycle();
            },
            "addx" => {
                self.run_cycle();
                self.run_cycle();
                self.register_x += instruction[1].parse::<i32>().unwrap();
            },
            _ => panic!("invalid instruction")
        }
    }

    fn run_cycle(&mut self) {
        let screen_line_num: usize = (self.cycles / 40) as usize;
        let screen_line_pos: i32 = self.cycles % 40;
        self.cycles += 1;
        match self.cycles {
            20 | 60 | 100 | 140 | 180 | 220 => {
                println!("signal strength = {}", self.cycles * self.register_x);
                self.total_signal_strength += self.cycles * self.register_x
            },
            _ => ()
        }
        dbg!(screen_line_num);
        dbg!(self.screen.len());
        if self.screen.len() < (screen_line_num + 1) as usize {
            self.screen.push(Vec::new());
            println!("adding new line to screen");
            dbg!(self.screen.len());
        }
        let pixel: char = if (screen_line_pos <= self.register_x + 1) &&
                             (screen_line_pos >= self.register_x - 1)
            {
                '#'
            }
            else {
                '.'
            };
        self.screen[screen_line_num].push(pixel);
        println!("line: {} char: {}", screen_line_num, self.screen[screen_line_num].len());
    }
}

fn main() {
    // let file_path = "input_sample.txt";
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("could not open file");
    let mut cpu = Cpu::new();

    for line in contents.lines() {
        let instruction: Vec<&str> = line.split(" ").collect();
        cpu.process_instruction(instruction);
    }
    for line in cpu.screen {
        println!("{}", line.iter().collect::<String>());
    }
}
