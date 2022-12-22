mod pattern;
mod position_calc;
mod range;

use range::{parse_range, Range};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = "Generage gcode test pattern for laser")]
struct Cli {
    /// Square len & wigth [mm]
    #[structopt(short = "s", long, default_value = "1")]
    pattern_size: f32,

    /// pattern offset [mm]
    #[structopt(short = "o", long, default_value = "0.2")]
    pattern_offset: f32,

    /// Laset beam width [mm]
    #[structopt(short = "l", long, default_value = "0.01")]
    heigth: f32,

    /// A
    #[structopt(short = "A", default_value = "100")]
    a: f32,

    /// S
    #[structopt(short = "S", default_value="1:10:1", parse(try_from_str = parse_range))]
    s: Range,

    /// F [mm/min]
    #[structopt(short = "F", default_value="10000:20000:1000", parse(try_from_str = parse_range))]
    f: Range,
}

fn main() {
    let args = Cli::from_args();

    let pattern_count = args.f.count() * args.s.count();

    println!(
        "(Test pattern: size: {}, offset: {}, count: {} (s: {} x f: {}))",
        args.pattern_size,
        args.pattern_offset,
        pattern_count,
        args.s.count(),
        args.f.count()
    );

    // генерируем указанное количество паттернов вокруг точки 0:0
    // S - вертикаль
    // F - горизонталь
    let pc = position_calc::PositionCalc::new(
        args.pattern_size,
        args.pattern_offset,
        args.s.count() as u32,
        args.f.count() as u32,
    );

    println!("G94\nG17\nG90");

    for (i, s) in args.s.into_iter().enumerate() {
        for (j, f) in args.f.into_iter().enumerate() {
            let pos = pc.position(i, j);
            let pattern = pattern::SquarePattern::new(pos, args.pattern_size, s, f, args.a, args.heigth);

            println!("{}", pattern.generate());
        }
    }
}
