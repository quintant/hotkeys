use device_query::{DeviceQuery, DeviceState, Keycode};
use serde_json::Value;
use std::{collections::HashMap, fs, thread, time, fmt::Debug};

// Function that spawns the desired program
fn spawn_program(program: &str) {
    let mut command = std::process::Command::new("cmd");
    command.arg("/c").arg(&(&program.strip_prefix("\"")).unwrap().strip_suffix("\"").unwrap());
    command.spawn().expect("Failed to spawn program");
}

fn main() {
    let device_state = DeviceState::new();
    let sleep_time = time::Duration::from_millis(50);

    let path: String = "C:/Users/sigur/Desktop/Projects/hotkeys/src/config.json".to_string();
    let data = fs::read_to_string(path).expect("Unable to read file");

    let keybinds: Value = serde_json::from_str(data.as_str()).unwrap();
    let mut workingkeys: HashMap<String, Value> = HashMap::new();
    let mut last: Keycode = Keycode::Meta;
    let mut cnt = 15;
    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        for key in keys {
            if key == last {
                last = key;
                continue;
            }
            if workingkeys.contains_key(&key.to_string())
                && workingkeys[&key.to_string()].is_string()
            {
                let mut keybind: String = String::new();
                workingkeys[&key.to_string()]
                    .to_string()
                    .clone_into(&mut keybind);
                thread::spawn(move || {
                    spawn_program(keybind.as_str());
                });
                workingkeys.clear();
                thread::sleep(sleep_time);
            } else if keybinds.as_object().unwrap().contains_key(&key.to_string())
                && keybinds[key.to_string()].is_string()
            {
                let mut keybind: String = String::new();
                workingkeys[&key.to_string()]
                    .to_string()
                    .clone_into(&mut keybind);
                thread::spawn(move || {
                    spawn_program(keybind.as_str());
                });
                workingkeys.clear();
                thread::sleep(sleep_time);
            } else {
                let mut tmpmap: HashMap<String, Value> = HashMap::new();

                // Check if workingkeys is an object and then add the value to the hashmap if it is
                if workingkeys.contains_key(&key.to_string())
                    && workingkeys[&key.to_string()].is_object()
                {
                    let dat = workingkeys[&key.to_string()].as_object().unwrap();
                    for (key, value) in dat {
                        // Append to workingkeys
                        tmpmap.insert(key.to_string(), value.to_owned());
                    }
                    cnt = 15;
                }
                if keybinds.as_object().unwrap().contains_key(&key.to_string())
                    && keybinds[&key.to_string()].is_object()
                {
                    let dat = keybinds[&key.to_string()].as_object().unwrap();
                    for (key, value) in dat {
                        tmpmap.insert(key.to_string(), value.to_owned());
                    }
                    cnt = 15;
                }
                for (key, value) in tmpmap {
                    workingkeys.insert(key, value);
                }
            }
        }
        cnt -= 1;
        if cnt <= 0 {
            cnt = 15;
            workingkeys.clear();
        }
        thread::sleep(sleep_time);
    }
}
