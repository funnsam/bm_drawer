use std::{io::{*, Write as IoWrite}, fmt::Write};

use serde::{Serialize, Deserialize};
use serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize, Default)]
struct Image {
    w: usize, h: usize, n: usize,
    canvas: Vec<Vec<Vec<bool>>>
}

fn main() {
    let mut args = std::env::args().skip(1);
    let stdin = stdin();
    let stdout = stdout();
    let (mut filename, mut image) = if let Some(filename) = args.next() {
        let a = std::fs::read_to_string(&filename).unwrap();
        (filename, from_str(&a).unwrap())
    } else {
        (format!("a.json"), Image::default())
    };
    // let w = args.next().unwrap().parse::<usize>().unwrap();
    // let h = args.next().unwrap().parse::<usize>().unwrap();
    // let n = args.next().unwrap().parse::<usize>().unwrap();

    // let mut canvas = vec![vec![vec![false; w]; h]; n];
    let mut active = 0;

    loop {
        print!(":");
        stdout.lock().flush().unwrap();

        let cmd = stdin.lock().lines().next().unwrap().unwrap();
        let mut cmd = cmd.split_whitespace();
        if let Some(operation) = cmd.next() {
            if let Ok(n) = operation.parse() {
                active = n;
            } else {
                fn plot(cmd: &mut std::str::SplitWhitespace, image: &mut Image, active: usize, action: impl Fn(&mut bool)) {
                    while let Some(x) = cmd.next() {
                        let x = x.parse::<usize>().unwrap();
                        let y = cmd.next().unwrap().parse::<usize>().unwrap();
                        if x > image.w || y > image.h {
                            continue;
                        }
                        action(&mut image.canvas[active][y-1][x-1]);
                    }
                }
                match operation {
                    "new" => {
                        image.w = cmd.next().unwrap().parse::<usize>().unwrap();
                        image.h = cmd.next().unwrap().parse::<usize>().unwrap();
                        image.n = cmd.next().unwrap().parse::<usize>().unwrap();

                        image.canvas = vec![vec![vec![false; image.w]; image.h]; image.n];
                    },
                    "w" => {
                        if let Some(f) = cmd.next() {
                            filename = f.to_string();
                        }
                        std::fs::write(&filename, to_string(&image).unwrap()).unwrap();
                    },
                    "s" => plot(&mut cmd, &mut image, active, |a| *a = true),
                    "c" => plot(&mut cmd, &mut image, active, |a| *a = false),
                    "t" => plot(&mut cmd, &mut image, active, |a| *a ^= true),
                    "export" => {
                        let filename = cmd.next().unwrap();
                        let mut file = format!("const BITMAPS: [u8; {}] = [\n", image.w.div_ceil(8) * image.h * image.n);

                        for bm in image.canvas.iter() {
                            write!(file, "    ").unwrap();
                            for y in bm.iter() {
                                for x in 0..image.w.div_ceil(8) {
                                    let row = &y[x*8..((x+1)*8).min(image.w)];
                                    let mut cur = 0;
                                    for (i, d) in row.iter().enumerate() {
                                        cur |= (*d as u8) << (7-i);
                                    }
                                    write!(file, "0x{cur:02X}, ").unwrap();
                                }
                            }
                            writeln!(file).unwrap();
                        }

                        write!(file, "];").unwrap();

                        std::fs::write(filename, &file).unwrap();
                    },
                    _ => todo!("{operation}")
                }
            }
        }
        print!("#{active}\t");
        for x in 1..=image.w {
            print!("{} ", x % 10);
        }
        println!();
        for (i, y) in image.canvas[active].iter().enumerate() {
            print!("{}\t", i+1);
            for x in y.iter() {
                if *x {
                    print!("\u{2588}\u{2588}");
                } else {
                    print!("\u{2500}\u{2500}");
                }
            }
            println!();
        }
    }
}
