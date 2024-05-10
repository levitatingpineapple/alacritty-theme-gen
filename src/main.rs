use clap::Parser;
use colored::*;
use palette::{FromColor, Oklch, Srgb};

#[derive(Parser, Debug)]
#[command(about)]
struct Theme {
    /// Foreground luminosity
    #[arg(long, value_name = "f32", default_value_t = 1.00)]
    fg: f32,

    /// Background luminosity
    #[arg(long, value_name = "f32", default_value_t = 0.00)]
    bg: f32,

    /// Natural colors luminosity
    #[arg(long, value_name = "f32", default_value_t = 0.75)]
    nl: f32,

    /// Natural colors chroma
    #[arg(long, value_name = "f32", default_value_t = 0.22)]
    nc: f32,

    /// Natural color hue offset in degrees
    #[arg(long, value_name = "f32", default_value_t = 0.00)]
    nho: f32,

    /// Bright colors luminosity
    #[arg(long, value_name = "f32", default_value_t = 0.90)]
    bl: f32,

    /// Bright colors chroma
    #[arg(long, value_name = "f32", default_value_t = 0.15)]
    bc: f32,

    /// Bright colors hue offset in degrees
    #[arg(long, value_name = "f32", default_value_t = 0.00)]
    bho: f32,
}

impl Theme {
    fn print(self) {
        #[rustfmt::skip]
        let colors: [&str; 8] = [
            "red    ",
            "yellow ",
            "green  ",
            "cyan   ",
            "blue   ",
            "magenta",
            "black  ",
            "white  ",
        ];

        // Primary grayscale foregrund
        let pfg = gray(self.fg);

        // Primary grayscale background
        let pbg = gray(self.bg);

        // Secondary foreground matches the luminosity of normal colors
        let sfg = gray(self.nl);

        // Secondary background requires contrast with both bg and nl
        let sbg = gray((self.bg + self.nl) / 2.0);

        // Print normal colors
        println!("[colors.normal]");
        print_color(pbg, colors[6]);
        for index in 0..6 {
            let hue: f32 = 30.0 + self.nho + 60.0 * index as f32;
            print_color(Oklch::new(self.nl, self.nc, hue), colors[index]);
        }
        print_color(sfg, colors[7]);

        // Print bright colors
        println!("\n[colors.bright]");
        print_color(sbg, colors[6]);
        for index in 0..6 {
            let hue: f32 = 30.0 + self.bho + 60.0 * index as f32;
            print_color(Oklch::new(self.bl, self.bc, hue), colors[index]);
        }
        print_color(pfg, colors[7]);
    }
}

fn gray(l: f32) -> Oklch {
    Oklch::new(l, 0.0, 0.0)
}

fn print_color(oklch: Oklch, name: &str) {
    let srgb: Srgb<u8> = Srgb::from_color(oklch).into_format::<u8>();
    let (r, g, b) = srgb.into_components();
    print!("{}", name.truecolor(r, g, b,));
    print!(" = ");
    let t: u8 = if oklch.l > 0.5 { 0x00 } else { 0xFF };
    println!(
        "{}",
        format!("\"#{:x}\"", srgb)
            .on_truecolor(r, g, b)
            .truecolor(t, t, t)
    );
}

fn main() {
    let theme = Theme::parse();
    dbg!(&theme);
    theme.print();
}
