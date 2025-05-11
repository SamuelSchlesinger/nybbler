// Pixelated Nybbler character variants in Tamagotchi style
// Each character has different states for the various actions

use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Serialize, Deserialize};

// Character type for identifying different character designs
#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
pub enum CharacterType {
    Blob,
    Square,
    Ghost,
    Cat,
    Robo,
}

impl CharacterType {
    // Get a random character type
    pub fn random() -> Self {
        let types = [
            CharacterType::Blob,
            CharacterType::Square,
            CharacterType::Ghost,
            CharacterType::Cat,
            CharacterType::Robo,
        ];
        
        let mut rng = thread_rng();
        *types.choose(&mut rng).unwrap()
    }
    
    // Get the neutral state for this character type
    pub fn neutral(&self) -> &'static str {
        match self {
            CharacterType::Blob => r#"
  ████████  
 ██        ██
██  ○    ○  ██
██          ██
██    ◡     ██
 ██        ██
  ████████  
"#,
            CharacterType::Square => r#"
 ▄▄▄▄▄▄▄▄▄▄
 █ ▓  ▓    █
 █         █
 █    ‿    █
 ▀▀▀▀▀▀▀▀▀▀
"#,
            CharacterType::Ghost => r#"
   ▄████▄   
  █ ◕  ◕ █  
  █      █  
  █  ▿   █  
  █▀▀▀▀▀▀█  
 ▀ ▀  ▀▀ ▀ ▀
"#,
            CharacterType::Cat => r#"
 /\_/\  
( o.o ) 
 > ᴥ <  
"#,
            CharacterType::Robo => r#"
  ▄███▄  
 █[□ □]█ 
 █  ▼  █ 
 ▀▀█ █▀▀ 
   ▀▀▀   
"#,
        }
    }

    // Get the eating animation for this character type
    pub fn eating(&self) -> &'static str {
        match self {
            CharacterType::Blob => r#"
  ████████  
 ██        ██
██  ○    ○  ██
██          ██
██    O     ██
 ██        ██
  ████████  
"#,
            CharacterType::Square => r#"
 ▄▄▄▄▄▄▄▄▄▄
 █ ▓  ▓    █
 █         █
 █    O    █
 ▀▀▀▀▀▀▀▀▀▀
"#,
            CharacterType::Ghost => r#"
   ▄████▄   
  █ ◕  ◕ █  
  █      █  
  █  O   █  
  █▀▀▀▀▀▀█  
 ▀ ▀  ▀▀ ▀ ▀
"#,
            CharacterType::Cat => r#"
 /\_/\  
( o.o ) 
 > O <  
"#,
            CharacterType::Robo => r#"
  ▄███▄  
 █[□ □]█ 
 █  O  █ 
 ▀▀█ █▀▀ 
   ▀▀▀   
"#,
        }
    }

    // Get the sleeping animation for this character type
    pub fn sleeping(&self) -> &'static str {
        match self {
            CharacterType::Blob => r#"
  ████████  
 ██        ██
██  -    -  ██
██          ██
██    ◡     ██
 ██        ██
  ████████  
"#,
            CharacterType::Square => r#"
 ▄▄▄▄▄▄▄▄▄▄
 █ -  -    █
 █         █
 █    ‿    █
 ▀▀▀▀▀▀▀▀▀▀
"#,
            CharacterType::Ghost => r#"
   ▄████▄   
  █ -  - █  
  █      █  
  █  ▿   █  
  █▀▀▀▀▀▀█  
 ▀ ▀  ▀▀ ▀ ▀
"#,
            CharacterType::Cat => r#"
 /\_/\  
( -.-)z 
 > ᴥ <  
"#,
            CharacterType::Robo => r#"
  ▄███▄  
 █[- -]█ 
 █  ▼  █ 
 ▀▀█ █▀▀ 
   ▀▀▀   
"#,
        }
    }

    // Get the playing animation for this character type
    pub fn playing(&self) -> &'static str {
        match self {
            CharacterType::Blob => r#"
  ████████  
 ██        ██
██  ◕    ◕  ██
██          ██
██    ◠     ██
 ██        ██
  ████████  
"#,
            CharacterType::Square => r#"
 ▄▄▄▄▄▄▄▄▄▄
 █ ♥  ♥    █
 █         █
 █    ◡    █
 ▀▀▀▀▀▀▀▀▀▀
"#,
            CharacterType::Ghost => r#"
   ▄████▄   
  █ ★  ★ █  
  █      █  
  █  ▽   █  
  █▀▀▀▀▀▀█  
 ▀ ▀  ▀▀ ▀ ▀
"#,
            CharacterType::Cat => r#"
 /\_/\  
(^o.o^)~
 > ᴥ <  
"#,
            CharacterType::Robo => r#"
  ▄███▄  
 █[! !]█ 
 █  ▲  █ 
 ▀▀█ █▀▀ 
   ▀▀▀   
"#,
        }
    }

    // Get the healing animation for this character type
    pub fn healing(&self) -> &'static str {
        match self {
            CharacterType::Blob => r#"
  ████████  
 ██        ██
██  +    +  ██
██          ██
██    ◡     ██
 ██        ██
  ████████  
"#,
            CharacterType::Square => r#"
 ▄▄▄▄▄▄▄▄▄▄
 █ +  +    █
 █         █
 █    ‿    █
 ▀▀▀▀▀▀▀▀▀▀
"#,
            CharacterType::Ghost => r#"
   ▄████▄   
  █ +  + █  
  █      █  
  █  ▿   █  
  █▀▀▀▀▀▀█  
 ▀ ▀  ▀▀ ▀ ▀
"#,
            CharacterType::Cat => r#"
 /\_/\  
( +.+ ) 
 > ᴥ <  
"#,
            CharacterType::Robo => r#"
  ▄███▄  
 █[+ +]█ 
 █  ▼  █ 
 ▀▀█ █▀▀ 
   ▀▀▀   
"#,
        }
    }
}