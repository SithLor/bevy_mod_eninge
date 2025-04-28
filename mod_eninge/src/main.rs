use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use rand::Rng; 

//exaple for send data to mod  and return data from mod

enum ButtonState {
    Up,
    Down,
}
struct GameData {
    mouse_x: i32,
    mouse_y: i32,
    // mouse click postion can be 
    mouse_1_state: ButtonState,
    mouse_2_state: ButtonState,

    key_w_state: ButtonState,
}
impl GameData {
    fn new() -> Self {
        GameData {
            mouse_x: 0,
            mouse_y: 0,
            mouse_1_state: ButtonState::Up,
            mouse_2_state: ButtonState::Up,
            key_w_state: ButtonState::Up,
        }
    }
}




struct Mod {
    exit_val: Arc<Mutex<bool>>,
    game_data: Arc<Mutex<GameData>>,
    extern_thread_pool: Vec<thread::JoinHandle<()>>,
}

impl Mod {
    fn new(exit_val: bool, game_data: GameData, extern_thread_pool: Vec<thread::JoinHandle<()>>) -> Mod {
        Mod {
            exit_val: Arc::new(Mutex::new(exit_val)),
            game_data: Arc::new(Mutex::new(game_data)),
            extern_thread_pool,
        }
    }
    
    fn init(&mut self) -> thread::JoinHandle<()> {
        // Create a thread to run the mod
        let exit_val_clone = Arc::clone(&self.exit_val);
        let game_data_clone = Arc::clone(&self.game_data);
        
        let handle = thread::spawn(move || {
            loop {
                let should_exit = *exit_val_clone.lock().unwrap();
                if should_exit {
                    break;
                }
                
                // Access game data
                let data = game_data_clone.lock().unwrap();
                
                // Run the mod
                println!("Running mod: x={}, y={}", data.mouse_x, data.mouse_y);
                
                // Drop the lock before sleeping
                drop(data);
                
                // Sleep for 1 second
                thread::sleep(Duration::from_secs(1));
            }
        });
        
        handle
    }
    
    fn set_exit(&mut self, val: bool) {
        let mut exit = self.exit_val.lock().unwrap();
        *exit = val;
    }
    
    fn update_game_data(&self, new_x: i32, new_y: i32) {
        let mut data = self.game_data.lock().unwrap();
        data.mouse_x = new_x;
        data.mouse_y = new_y;
    }
}





// help func
fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    // Loop until user presses q
    let mut stop_val = false;

    // Create game data
    let game_data = GameData::new();
    
    // Create thread pool
    let mut handles = vec![];



    let mut mod_1 = Mod::new(false, game_data, handles);
    let mod_thread = mod_1.init();

    let mut rng = rand::rng();

    while !stop_val {
        let input = get_user_input();

        if input == "q" {
            stop_val = true;
            mod_1.set_exit(true);
        } else {
            // Create random values using a rng instance
            
            // Random coordinates between 0 and 99
            let new_x = rng.gen_range(0..100);
            let new_y = rng.gen_range(0..100);
            
            // Randomly decide button states
            let random_state = || {
                if rng.gen_bool(0.5) {
                    ButtonState::Down
                } else {
                    ButtonState::Up
                }
            };
            
            // Update game data with random values
            mod_1.update_game_data(new_x, new_y);

            println!("You pressed: {}", input);
            println!("Generated random position: ({}, {})", new_x, new_y);
        }
    }
    
    mod_thread.join().unwrap();
}