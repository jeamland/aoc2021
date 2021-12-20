use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum Pixel {
    Light = 1,
    Dark = 0,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Light,
            '.' => Self::Dark,
            _ => panic!("eek"),
        }
    }
}

impl std::fmt::Display for Pixel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Light => write!(f, "#"),
            Self::Dark => write!(f, "."),
        }
    }
}

struct Image {
    enhancer: Vec<Pixel>,
    image: Vec<Vec<Pixel>>,
    default_pixel: Pixel,
}

impl Image {
    fn new(enhancer: Vec<Pixel>, image: Vec<Vec<Pixel>>) -> Self {
        Self {
            enhancer,
            image,
            default_pixel: Pixel::Dark,
        }
    }

    fn enhance(&mut self) {
        let row_len = self.image[0].len();
        let mut sample_image = vec![vec![self.default_pixel; row_len + 6]; 3];
        for row in &self.image {
            let mut sample_row = vec![self.default_pixel; 3];
            sample_row.extend(row.iter());
            sample_row.push(self.default_pixel);
            sample_row.push(self.default_pixel);
            sample_row.push(self.default_pixel);
            sample_image.push(sample_row);
        }
        sample_image.push(vec![self.default_pixel; row_len + 6]);
        sample_image.push(vec![self.default_pixel; row_len + 6]);
        sample_image.push(vec![self.default_pixel; row_len + 6]);

        let mut new_image = Vec::new();

        for y in 0..sample_image.len() {
            let mut new_row = Vec::new();

            for x in 0..sample_image[y].len() {
                let mut value = 0;
                for py in y as isize - 1..=y as isize + 1 {
                    for px in x as isize - 1..=x as isize + 1 {
                        value <<= 1;
                        if px != -1
                            && py != -1
                            && py < sample_image.len() as isize - 1
                            && px < sample_image[py as usize].len() as isize - 1
                        {
                            value |= sample_image[py as usize][px as usize] as usize;
                        } else {
                            value |= self.default_pixel as usize;
                        }
                    }
                }
                new_row.push(self.enhancer[value]);
            }

            new_image.push(new_row);
        }

        self.default_pixel = match self.default_pixel {
            Pixel::Light => self.enhancer[511],
            Pixel::Dark => self.enhancer[0],
        };
        self.image = new_image;
    }

    fn print(&self) {
        for row in &self.image {
            for p in row {
                print!("{}", p);
            }
            println!();
        }
    }

    fn lit_pixels(&self) -> usize {
        self.image
            .iter()
            .map(|r| r.iter().map(|p| *p as usize).sum::<usize>())
            .sum::<usize>()
    }
}

fn main() {
    let file = File::open(std::env::args_os().nth(1).unwrap()).unwrap();
    let mut lines = BufReader::new(file).lines();

    let enhancer: Vec<Pixel> = lines
        .next()
        .unwrap()
        .unwrap()
        .chars()
        .map(Pixel::from)
        .collect();

    let mut image: Vec<Vec<Pixel>> = Vec::new();

    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }

        image.push(line.chars().map(Pixel::from).collect());
    }

    let mut image = Image::new(enhancer, image);
    image.print();
    println!();
    image.enhance();
    image.print();
    println!();
    image.enhance();
    image.print();

    println!("Pixels lit: {}", image.lit_pixels());
}
