extern crate rand; // Crate for Random Number Generation
extern crate termion; // Crate for console output with colors
use std::{env, thread, time}; // Grabs environment variables and threading
use std::fs::File; // Used for managing files
use std::io::{BufRead, BufReader}; // Buffered readers handle I/O management and deliver only the data that is needed
use termion::{clear, color}; // Clears terminal screen for each generation. Changes colors for better readability

fn census (_world: [[u8; 75]; 75]) -> u16 // counts world population
{
    let mut count = 0; // Initialize Population Count

    for i in 0..74 { // Loop through each row
        for j in 0..74 { // Loop through each column
            if _world[i][j] == 1
            {
                count += 1; // Increment population count if cell is alive
            }
        }
    }
    count // return world population count
}
fn generation (_world: [[u8; 75]; 75]) -> [[u8; 75]; 75] // creates next gen. previous gen gets replaced by new gen
{
    let mut newworld = [[0u8; 75]; 75]; // creates newworld array. 
    for i in 0..74 {
        for j in 0..74 {
            let mut count = 0;
            if i>0 {
                count = count + _world[i-1][j]; // count cell above
            }
            if i>0 && j>0 {
                count = count + _world[i-1][j-1]; // count cell above and to the left
            }
            if i>0 && j<74 {
                count = count + _world[i-1][j+1]; // count cell above and to the right
            }
            if i<74 && j>0 {
                count = count + _world[i+1][j-1]; // count cell below and to the left
            }
            if i<74 {
                count = count + _world[i+1][j]; // count cell below
            }
            if i<74 && j<74 {
                count = count + _world[i+1][j+1]; // count cell below and to the right
            }
            if j>0 {
                count = count + _world[i][j-1]; // count cell to the left
            }
            if j<74 {
                count = count + _world[i][j+1]; // count cell to the right
            }

            newworld[i][j] = 0; 

            if (count <2) && (_world[i][j] == 1) {
                newworld[i][j] = 0; // represents underpopulation
            }
            if _world[i][j] == 1 && (count == 2 || count == 3) {
                newworld[i][j] = 1; // represents survival
            }
            if (_world[i][j] == 0) && (count == 3) {
                newworld[i][j] = 1; // represents reproduction
            } 
        }
    }
    newworld // return final newworld array
}

fn main () {
    let mut world = [[0u8; 75]; 75]; // creates world array
    let mut generations = 0; // number of generations

    let args: Vec<String> = env::args().collect(); // collect CLI inputs and store in vector as a collection.

    if args.len() < 2 { // Determines if no filename is found (dead if no file name)
        for i in 0..74 { // loop through each row
            for j in 0..74 { // loop through each column
                if rand::random() { 
                    world[i][j] = 1; // set cell to alive
                } else {
                    world[i][j] = 0; // set cell to dead
                }
            }
        }
    } else {
        let filename = env::args().nth(1).unwrap(); // unwraps the CLI input option into a string and stores it in a variable
        world = populate_from_file(filename); // populate world array from filename variable
    }

    println!("Population at generation {} is {}", generations, census(world));
    for _gens in 0..100 { // loop 100 times
        let temp = generation(world); // create new generation and store results in variable temp
        world = temp; // sets world variable to memory location of the new array created in generation function
        generations += 1; // increment generations
        println! ("{}", clear::All); // clear screen for readability. uses print macro for escape character sequence
        displayworld(world); // visualize world
        println!("{blue}Population at generation {g} is {c}", 
        blue = color::Fg(color::Blue), g = generations, c = census(world)); // prints statistics of new world
        thread::sleep(time::Duration::from_secs(2)); // sleep for 2 seconds
    }
}

fn populate_from_file(filename: String) -> [[u8; 75]; 75] // returns a populated grid (only living cells have file)
{
    let mut newworld = [[0u8; 75]; 75]; // creates newworld array
    let file = File::open(filename).unwrap(); // open file (name is living cell coordinates)
    let reader = BufReader::new(file); // creates a reader for the file
    let mut pairs: Vec<(usize, usize)> = Vec::new(); // create vector of tuples. usize bc the +int values are unknown
    for (index, line) in reader.lines().enumerate() { // reads each line
        let l = line.unwrap(); // unwrap the Result datatype from filename into a string
        let mut words = l.split_whitespace(); // split line into words
        let left = words.next().unwrap(); // get left side of tuple
        let right = words.next().unwrap(); // get right side of tuple
        pairs.push((left.parse::<usize>().unwrap(), right.parse::<usize>().unwrap())); // push tuple to vector
    }

    for i in 0..74 {
        for j in 0..74 {
            newworld[i][j] = 0; // initially sets all cells to dead
        }
    }

    for (x,y) in pairs { // all coordinates with a pair are alive
        newworld[x][y] = 1; // sets empty cells to alive based on tuple coordinates in filename
    }
    newworld // returns newworld array
}

fn displayworld(world: [[u8; 75]; 75])
{ 
    for i in 0..74 {
        for j in 0..74 { 
            if world[i][j] == 1  
            {
                print!("{magenta}$", magenta = color::Fg(color::Magenta)); // print purple "$" for alive cells
            }
            else
            {
                print!("{white} ", white = color::Fg(color::White)); // print white space for dead cells
            }
        }
        println!("");
    }
}