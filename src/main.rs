use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::env;
use std::error::Error;

struct TVShow {
    start: u32,
    end: u32,
}

impl TVShow {
    fn new(start: u32, end: u32) -> Self {
        TVShow { start: start, end: end }
    }
}

fn main() {
    let file_name = match env::args().nth(1) {
        Some(file_name) => file_name,
        None => panic!("You need to pass in a commandline argument!"),
    };

    let mut shows = read_file_input(&file_name);

    shows.sort_by(|fst, snd| fst.end.cmp(&snd.end));

    let num_shows = schedule_vhs(shows);

    println!("Number of shows you can record: {}", num_shows);
}

fn schedule_vhs(shows: Vec<TVShow>) -> u16 {
    let mut recorded_shows = vec![];

    for show in shows.iter().clone() {
        if recorded_shows.len() < 1 {
            recorded_shows.push(show);
        }

        if recorded_shows.last().unwrap().end <= show.start {
            recorded_shows.push(show);
        }
    }

    recorded_shows.len() as u16
}

fn read_file_input(file_name: &str) -> Vec<TVShow> {
    let mut shows = vec![];

    let file = match File::open(file_name) {
        Ok(file) => file,
        Err(what) => panic!("{}", Error::description(&what)),
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let times: Vec<String> = line.unwrap().split_whitespace().map(|token| token.to_string()).collect();
        shows.push(TVShow::new(times[0].parse().unwrap(), times[1].parse().unwrap()));
    }

    shows
}
