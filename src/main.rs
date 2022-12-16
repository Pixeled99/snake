use rand::Rng;
use colored::Colorize;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::{thread, time::Duration};

const BOARD_Y : usize = 25;
const BOARD_X : usize = 50;
const MID_Y : usize = BOARD_Y/2;

fn spawn_stdin_channel() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap();
    });
    rx
}

fn print_ready_board(board: &Vec<Vec<String>>) -> Vec<String>{
    let mut print_board = Vec::new();
    for vec in board.iter(){
        print_board.push(vec.join("").to_string());
    }
    print_board
}

fn add_food(board: &mut Vec<Vec<String>>) -> Vec<usize>{
    let mut rng = rand::thread_rng();
    let mut y = rng.gen_range(0..BOARD_Y);
    let mut x = rng.gen_range(0..BOARD_X);
    while board[y][x] != "¤"{
        y = rng.gen_range(0..BOARD_Y);
        x = rng.gen_range(0..BOARD_X);
    }
    board[y][x] = "X".red().to_string();
    vec![x, y]
}

struct Snake {
    direction : String,
    head : String,
    cords : Vec<Vec<usize>>,
    food_pos : Vec<usize>,
}

impl Snake{
    fn update(&mut self){
        let mut cords = Vec::new();
        let mut x = self.cords.first().unwrap()[0];
        let mut y = self.cords.first().unwrap()[1];
        if self.direction == "up"{
            y -= 1;
        }
        if self.direction == "down"{
            y += 1;
        }
        if self.direction == "right"{
            x += 1;
        }
        if self.direction == "left"{
            x -= 1;
        }
        cords.push(x);
        cords.push(y);
        self.cords.insert(0, cords);
        self.cords.pop();
    }

    fn board_render(&mut self) -> Vec<Vec<String>>{
      let mut ate_food = false;
        let mut board = vec![vec!["¤".to_string() ; BOARD_X]; BOARD_Y];
        for (index, cord) in self.cords.iter().enumerate(){
            let mut piece = "=".green().to_string();
            if index == 0{
                piece = self.head.green().to_string();
            }
            board[cord[1]][cord[0]] = piece;
            if self.food_pos[1] == cord[1] && self.food_pos[0] == cord[0]{
              ate_food = true;
            }
        }
      if ate_food{
        let mut cords = Vec::new();
        let mut x = self.cords.first().unwrap()[0];
        let mut y = self.cords.first().unwrap()[1];
        if self.direction == "up"{
            y -= 1;
        }
        if self.direction == "down"{
            y += 1;
        }
        if self.direction == "right"{
            x += 1;
        }
        if self.direction == "left"{
            x -= 1;
        }
        cords.push(x);
        cords.push(y);
        self.cords.push(cords);
        self.food_pos = add_food(&mut board);
      }
      board[self.food_pos[1]][self.food_pos[0]] = "X".red().to_string();
      board
}
}

fn main() {
    let stdin_channel = spawn_stdin_channel();
    let mut board = vec![vec!["¤".to_string() ; BOARD_X]; BOARD_Y];
    let food_pos = add_food(&mut board);
    let game = true;
    let mut snake = Snake{direction: "right".to_string(), head: "►".to_string(), cords: vec![vec![8,MID_Y],vec![7,MID_Y],vec![6,MID_Y]], food_pos: food_pos};
    while game{
        print!("{}[2J", 27 as char);
        let board = snake.board_render(); 
        snake.update(); 
        let print_board = print_ready_board(&board);  
        for vec in print_board.iter(){
            println!("{}",vec)
        }
        let result = stdin_channel.try_recv();
        if result == Ok("w\n".to_string()){
            snake.direction = "up".to_string(); 
            snake.head = "▲".to_string()
        }
        if result == Ok("a\n".to_string()){
            snake.direction = "left".to_string();
            snake.head = "◄".to_string()
        }
        if result == Ok("s\n".to_string()){
            snake.direction = "down".to_string();
            snake.head = "▼".to_string()
        }
        if result == Ok("d\n".to_string()){
            snake.direction = "right".to_string();
            snake.head = "►".to_string()
        }
        thread::sleep(Duration::from_millis(100));
        
    }
}