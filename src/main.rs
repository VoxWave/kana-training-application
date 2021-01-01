// Kana training application
//     Copyright (C) 2021  Victor "VoxWave" Bankowski

//     This program is free software: you can redistribute it and/or modify
//     it under the terms of the GNU General Public License as published by
//     the Free Software Foundation, either version 3 of the License, or
//     (at your option) any later version.

//     This program is distributed in the hope that it will be useful,
//     but WITHOUT ANY WARRANTY; without even the implied warranty of
//     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//     GNU General Public License for more details.

//     You should have received a copy of the GNU General Public License
//     along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{fs::File, io::{BufRead, BufReader}};

use rand::{prelude::SliceRandom, thread_rng};

 
fn main() {
    let file = File::open("kana.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    let mut pairs = Vec::new();
    while buf_reader.read_line(&mut line).unwrap() != 0 {
        let mut pair = line.trim().split_whitespace();
        let kana = pair.next().unwrap();
        let romanji = pair.next().unwrap();
        pairs.push((kana.to_string(), romanji.to_string()));
        line.clear();
    }
    let mut rng = thread_rng();
    pairs.shuffle(&mut rng);
    let mut round = 1;
    loop {
        println!("Round {}:", round);
        let mut missed = Vec::new();
        while let Some(pair) = pairs.pop() {
            println!("{}", pair.0);
            std::io::stdin().read_line(&mut line).unwrap();
            if line.trim() == pair.1 {
                println!("Correct");
            } else {
                if !line.trim().is_empty() {
                    print!("Incorrect. ")
                }
                println!("{} is {}\n", pair.0, pair.1);
                missed.push(pair);
            }
            line.clear();
        }
        if missed.is_empty() {
            break;
        } else {
            missed.shuffle(&mut rng);
            pairs = missed;
        }
        round += 1;
    }
    println!("Practice completed. Congratulations!");
}
