use std::time::Duration;
use std::thread;
use std::path::PathBuf;
use std::fs;
use std::io::{self, ErrorKind};
use std::process;
use dialoguer::{Select, theme::ColorfulTheme};
use indicatif::{ProgressBar, ProgressStyle};
use console::{Term, style};
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};
use dirs::data_dir;
use clap::{Parser, Subcommand};

mod characters;

// States that the Nybbler can be in
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
enum NybblerMood {
    Happy,
    Neutral,
    Sad,
    Sick,
    Sleeping,
    Excited,
    Playful,
}

impl NybblerMood {
    fn to_emoji(&self) -> &str {
        match self {
            NybblerMood::Happy => "😊",
            NybblerMood::Neutral => "😐",
            NybblerMood::Sad => "😢",
            NybblerMood::Sick => "🤒",
            NybblerMood::Sleeping => "😴",
            NybblerMood::Excited => "🤩",
            NybblerMood::Playful => "😋",
        }
    }

#[allow(dead_code)]
    fn get_animation(&self) -> Vec<&str> {
        match self {
            NybblerMood::Happy => vec!["(⌦ᕔ ᕕ ᕔ⌦)", "(⌦ᕔ‿ᕔ⌦)", "(⌦ᕔ ᕕ ᕔ⌦)", "(⌦ᕔ‿ᕔ⌦)"],
            NybblerMood::Neutral => vec!["(・ω・)", "(・ω・)", "(・ω・)", "(・ω・)"],
            NybblerMood::Sad => vec!["(╥_╥)", "(╥︣_╥︭)", "(╥_╥)", "(╥︣_╥︭)"],
            NybblerMood::Sick => vec!["(˘_˘)", "(˘_˘)", "(˘_˘)", "(*￣m￣)"],
            NybblerMood::Sleeping => vec!["(-.-)zzz", "(-_-)zzz", "(-.-)zzz", "(-_-)zzz"],
            NybblerMood::Excited => vec!["(★^O^★)", "(☆^ー^☆)", "(★^O^★)", "(☆^ー^☆)"],
            NybblerMood::Playful => vec!["(◕ᗜ◕✿)", "(◠‿◠✿)", "(◕ᗜ◕✿)", "(◠‿◠✿)"],
        }
    }
}

// The Nybbler struct to hold the game state
#[derive(Serialize, Deserialize)]
struct Nybbler {
    name: String,
    hunger: u8,
    happiness: u8,
    energy: u8,
    health: u8,
    age: u16,
    #[serde(with = "chrono_serde")]
    last_updated: DateTime<Local>,
    mood: NybblerMood,
    #[serde(default = "characters::CharacterType::random")]
    character_type: characters::CharacterType,
}

// Helper module to serialize/deserialize chrono::DateTime
mod chrono_serde {
    use chrono::{DateTime, Local};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = DateTime::parse_from_rfc3339(&s)
            .map_err(serde::de::Error::custom)?
            .with_timezone(&Local);
        Ok(dt)
    }
}

impl Nybbler {
    // Create a new Nybbler with default values
    fn new(name: String) -> Self {
        Nybbler {
            name,
            hunger: 50,
            happiness: 50,
            energy: 100,
            health: 100,
            age: 0,
            last_updated: Local::now(),
            mood: NybblerMood::Happy,
            character_type: characters::CharacterType::random(),
        }
    }

    // Save the Nybbler state to a file
    fn save(&self) -> io::Result<()> {
        let save_dir = get_save_directory()?;
        let save_path = save_dir.join(format!("{}.json", self.name.to_lowercase()));

        let json = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

        fs::write(save_path, json)
    }

    // Load a Nybbler from a file
    fn load(name: &str) -> io::Result<Self> {
        let save_dir = get_save_directory()?;
        let save_path = save_dir.join(format!("{}.json", name.to_lowercase()));

        let data = fs::read_to_string(save_path)?;
        let nybbler: Nybbler = serde_json::from_str(&data)
            .map_err(|e| io::Error::new(ErrorKind::Other, e))?;

        Ok(nybbler)
    }

    // Check if a save file exists for a Nybbler
    fn save_exists(name: &str) -> bool {
        if let Ok(save_dir) = get_save_directory() {
            let save_path = save_dir.join(format!("{}.json", name.to_lowercase()));
            save_path.exists()
        } else {
            false
        }
    }

    // Update the Nybbler's stats based on elapsed time
    fn update(&mut self) {
        let now = Local::now();
        let diff = now.signed_duration_since(self.last_updated);
        let hours_passed = diff.num_seconds() as f64 / 3600.0;

        // Decrease stats based on time
        let hunger_decrease = (5.0 * hours_passed).min(5.0) as u8;
        let happiness_decrease = (3.0 * hours_passed).min(3.0) as u8;
        let energy_decrease = (2.0 * hours_passed).min(2.0) as u8;

        // Apply decreases, ensuring we don't underflow
        self.hunger = self.hunger.saturating_sub(hunger_decrease);
        self.happiness = self.happiness.saturating_sub(happiness_decrease);
        self.energy = self.energy.saturating_sub(energy_decrease);

        // Update age (1 day every 24 real hours)
        self.age += (hours_passed / 24.0) as u16;

        // Update health based on hunger and happiness
        if self.hunger < 20 || self.happiness < 20 {
            self.health = self.health.saturating_sub(5);
        }

        // Update mood based on stats
        self.update_mood();

        // Update timestamp
        self.last_updated = now;
    }

    // Update the Nybbler's mood based on its stats
    fn update_mood(&mut self) {
        if self.health < 30 {
            self.mood = NybblerMood::Sick;
        } else if self.energy < 20 {
            self.mood = NybblerMood::Sleeping;
        } else if self.hunger < 30 || self.happiness < 30 {
            self.mood = NybblerMood::Sad;
        } else if self.hunger > 70 && self.happiness > 70 && self.energy > 70 {
            self.mood = NybblerMood::Excited;
        } else if self.hunger > 70 && self.happiness > 70 {
            self.mood = NybblerMood::Happy;
        } else if self.happiness > 80 {
            self.mood = NybblerMood::Playful;
        } else {
            self.mood = NybblerMood::Neutral;
        }
    }

    // Feed the Nybbler
    fn feed(&mut self) {
        self.hunger = (self.hunger + 30).min(100);
        self.energy = (self.energy + 5).min(100);
        self.update_mood();
    }

    // Play with the Nybbler
    fn play(&mut self) {
        self.happiness = (self.happiness + 20).min(100);
        self.hunger = self.hunger.saturating_sub(10);
        self.energy = self.energy.saturating_sub(15);
        self.update_mood();
    }

    // Put the Nybbler to sleep
    fn sleep(&mut self) {
        self.energy = 100;
        self.happiness = (self.happiness + 5).min(100);
        self.update_mood();
    }

    // Heal the Nybbler
    fn heal(&mut self) {
        self.health = 100;
        self.update_mood();
    }

    // Check if the Nybbler is alive
    fn is_alive(&self) -> bool {
        self.health > 0
    }
}

// Helper function to get the save directory
fn get_save_directory() -> io::Result<PathBuf> {
    let mut save_dir = data_dir()
        .ok_or_else(|| io::Error::new(ErrorKind::NotFound, "Could not find data directory"))?;

    save_dir.push("nybbler");

    if !save_dir.exists() {
        fs::create_dir_all(&save_dir)?;
    }

    Ok(save_dir)
}

// Delete all Nybbler save files
fn delete_all_nybblers() -> io::Result<usize> {
    let save_dir = get_save_directory()?;

    // Make sure the directory exists
    if !save_dir.exists() {
        return Ok(0);
    }

    let mut count = 0;
    for entry in fs::read_dir(save_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Only delete JSON files
        if path.extension().map_or(false, |ext| ext == "json") {
            fs::remove_file(path)?;
            count += 1;
        }
    }

    Ok(count)
}

// Command line arguments structure
#[derive(Parser)]
#[command(name = "nybbler")]
#[command(about = "🎮 Nybbler: The Terminal Virtual Pet 🐙", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Delete all Nybbler pets
    DeleteAll,
}

fn display_stats(nybbler: &Nybbler, term: &Term) -> Result<(), std::io::Error> {
    term.clear_screen()?;

    // Display fancy header with border
    let header = format!("✨ {} the Nybbler ✨  Age: {} days 🎂", nybbler.name, nybbler.age);
    let border = "•*´¨`*•.¸¸.•*´¨`*•.¸¸.•*´¨`*•.¸¸.•*´¨`*•.¸¸.•";

    println!("{}", style(border).cyan());
    println!("{}", style(header).bold().magenta());
    println!("{}", style(border).cyan());

    // Display animated mood
    let mood_text = match nybbler.mood {
        NybblerMood::Happy => "💖 I'm happy! 💖",
        NybblerMood::Neutral => "🌱 I'm doing okay. 🌱",
        NybblerMood::Sad => "💧 I'm feeling sad... 💧",
        NybblerMood::Sick => "🌡️ I don't feel well... 💊",
        NybblerMood::Sleeping => "💤 Zzz... 💤",
        NybblerMood::Excited => "✨ I'm super excited! ✨",
        NybblerMood::Playful => "🎮 Let's play! 🎮",
    };

    println!("{} {}", style(nybbler.mood.to_emoji()).bold(), style(mood_text).italic());

    // Display the pixelated character
    let character_display = match nybbler.mood {
        NybblerMood::Sleeping => nybbler.character_type.sleeping(),
        _ => nybbler.character_type.neutral(),
    };
    println!("{}", style(character_display).bold().yellow());

    println!();

    // Display stats bars with cute emojis
    let get_bar_style = |emoji: &str| {
        ProgressStyle::with_template(&format!("{}  [{{bar:20.green/red}}] {{pos}}/{{len}}", emoji))
            .unwrap()
            .progress_chars("█▉▊▋▌▍▎▏ ")
    };

    // Hunger
    let hunger_bar = ProgressBar::new(100);
    hunger_bar.set_style(get_bar_style("🍔"));
    hunger_bar.set_position(nybbler.hunger as u64);
    println!("{}:", style("Hunger").bold().blue());
    hunger_bar.tick();

    // Happiness
    let happiness_bar = ProgressBar::new(100);
    happiness_bar.set_style(get_bar_style("🎈"));
    happiness_bar.set_position(nybbler.happiness as u64);
    println!("{}:", style("Happiness").bold().magenta());
    happiness_bar.tick();

    // Energy
    let energy_bar = ProgressBar::new(100);
    energy_bar.set_style(get_bar_style("⚡"));
    energy_bar.set_position(nybbler.energy as u64);
    println!("{}:", style("Energy").bold().yellow());
    energy_bar.tick();

    // Health
    let health_bar = ProgressBar::new(100);
    health_bar.set_style(get_bar_style("💖"));
    health_bar.set_position(nybbler.health as u64);
    println!("{}:", style("Health").bold().red());
    health_bar.tick();

    println!();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse command line arguments
    let cli = Cli::parse();

    // Handle commands
    if let Some(Commands::DeleteAll) = cli.command {
        match delete_all_nybblers() {
            Ok(count) => {
                println!("🗑️ Successfully deleted {} Nybbler pets!", count);
                println!("🎮 Run the game without arguments to create a new pet.");
                return Ok(());
            },
            Err(e) => {
                eprintln!("Error deleting Nybblers: {}", e);
                process::exit(1);
            }
        }
    }

    // Regular game flow
    let term = Term::stdout();
    term.clear_screen()?;

    // Welcome message with cute ASCII art
    println!("{}", style("
     /\\_/\\
    ( o.o )
     > ^ <
  ✨ NYBBLER ✨").bold().yellow());
    println!("{}", style("Welcome to Terminal Nybbler!").bold().green());
    println!("{}", style("🌈 Take care of your virtual pet and keep it happy! 🌈").cyan());
    println!("{}", style("✨ You can create a new pet or load an existing one by name! ✨").italic().magenta());
    println!();

    // Ask for a name (or to load an existing Nybbler)
    let name = dialoguer::Input::<String>::new()
        .with_prompt("Enter your Nybbler's name (new or existing)")
        .interact_text()?;

    // Check if a save exists and ask if we should load it
    let mut nybbler = if Nybbler::save_exists(&name) {
        let load_save = dialoguer::Confirm::new()
            .with_prompt(format!("A Nybbler named {} already exists! Would you like to load it?", name))
            .default(true)
            .interact()?;

        if load_save {
            match Nybbler::load(&name) {
                Ok(loaded) => {
                    println!("{} {} has been loaded! {}", style("🎉").bold(), style(&name).bold().yellow(), style("🎉").bold());
                    println!("{} Time has passed since you last played... {}", style("⏰").bold(), style("⏰").bold());
                    thread::sleep(Duration::from_millis(1500));
                    loaded
                },
                Err(e) => {
                    println!("Error loading save: {}", e);
                    println!("Creating a new Nybbler instead...");
                    thread::sleep(Duration::from_millis(1500));
                    Nybbler::new(name)
                }
            }
        } else {
            println!("Creating a new Nybbler named {}...", name);
            Nybbler::new(name)
        }
    } else {
        // Create new Nybbler
        Nybbler::new(name)
    };

    // Main game loop
    loop {
        // Update nybbler state
        nybbler.update();

        // Check if nybbler is alive
        if !nybbler.is_alive() {
            term.clear_screen()?;
            println!("{}", style("
      .======.
      | RIP |
      |      |
      |      |
      |      |
      '======'").bold());
            println!("{}", style("💔 Oh no! Your Nybbler has passed away! 💔").bold().red());
            println!("🌈 {} lived for {} wonderful days with you. 🌈", nybbler.name, nybbler.age);
            println!("🌟 Thank you for taking care of your Nybbler! 🌟");
            break;
        }

        // Display stats
        display_stats(&nybbler, &term)?;

        // Show available actions with cute emojis
        let options = vec!["🍔 Feed", "🎮 Play", "💤 Sleep", "💊 Heal", "👋 Exit"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("✨ What would you like to do? ✨")
            .items(&options)
            .default(0)
            .interact_on(&term)?;

        // Process selection with cute responses
        match selection {
            0 => {
                nybbler.feed();
                println!("{} You fed {} a delicious meal! 🍔 Yum yum! {}", style("🎉").bold(), style(&nybbler.name).bold().yellow(), style("🎉").bold());
                for _ in 0..3 {
                    thread::sleep(Duration::from_millis(300));
                    term.clear_last_lines(1)?;
                    println!("{} Nom nom nom... {} is eating! {}", style("🍽️").bold(), style(&nybbler.name).bold().yellow(), style("🍽️").bold());
                    println!("{}", style(nybbler.character_type.eating()).bold().yellow());
                    thread::sleep(Duration::from_millis(300));
                    term.clear_last_lines(2)?;
                    println!("{} Yummy! That was delicious! {}", style("😋").bold(), style("😋").bold());
                    println!("{}", style(nybbler.character_type.neutral()).bold().yellow());
                }
            },
            1 => {
                nybbler.play();
                println!("{} You played with {}! So much fun! {}", style("🎮").bold(), style(&nybbler.name).bold().yellow(), style("🎮").bold());
                for _ in 0..3 {
                    thread::sleep(Duration::from_millis(300));
                    term.clear_last_lines(1)?;
                    println!("{} Wheee! {} is having fun! {}", style("🎯").bold(), style(&nybbler.name).bold().yellow(), style("🎯").bold());
                    println!("{}", style(nybbler.character_type.playing()).bold().yellow());
                    thread::sleep(Duration::from_millis(300));
                    term.clear_last_lines(2)?;
                    println!("{} Bouncing around with joy! {}", style("🏀").bold(), style("🏀").bold());
                    println!("{}", style(nybbler.character_type.neutral()).bold().yellow());
                }
            },
            2 => {
                nybbler.sleep();
                println!("{} {} took a nap and feels refreshed! {}", style("💤").bold(), style(&nybbler.name).bold().yellow(), style("💤").bold());
                for _ in 0..3 {
                    thread::sleep(Duration::from_millis(400));
                    term.clear_last_lines(1)?;
                    println!("{} Zzz... {} is sleeping soundly... {}", style("😴").bold(), style(&nybbler.name).bold().yellow(), style("😴").bold());
                    println!("{}", style(nybbler.character_type.sleeping()).bold().yellow());
                    thread::sleep(Duration::from_millis(400));
                    term.clear_last_lines(2)?;
                    println!("{} Dreaming of treats and toys... {}", style("💭").bold(), style("💭").bold());
                    println!("{}", style(nybbler.character_type.sleeping()).bold().yellow());
                }
            },
            3 => {
                nybbler.heal();
                println!("{} You gave {} medicine and they're feeling better! {}", style("💊").bold(), style(&nybbler.name).bold().yellow(), style("💊").bold());
                for _ in 0..3 {
                    thread::sleep(Duration::from_millis(300));
                    term.clear_last_lines(1)?;
                    println!("{} {} is recovering... {}", style("🌡️").bold(), style(&nybbler.name).bold().yellow(), style("🌡️").bold());
                    println!("{}", style(nybbler.character_type.healing()).bold().yellow());
                    thread::sleep(Duration::from_millis(300));
                    term.clear_last_lines(2)?;
                    println!("{} All better now! Healthy and strong! {}", style("💪").bold(), style("💪").bold());
                    println!("{}", style(nybbler.character_type.neutral()).bold().yellow());
                }
            },
            4 => {
                if confirm_exit()? {
                    // Save the nybbler before exiting
                    match nybbler.save() {
                        Ok(_) => {
                            println!("{} {} has been saved successfully! {}", style("💾").bold(), style(&nybbler.name).bold().yellow(), style("💾").bold());
                            thread::sleep(Duration::from_millis(1000));
                        },
                        Err(e) => {
                            println!("Error saving nybbler: {}", e);
                            thread::sleep(Duration::from_millis(1000));
                        }
                    }
                    term.clear_screen()?;
                    println!("{}", style("
      /\\_/\\
     ( ^ω^ )
     / >👋< \\
    Goodbye!").bold().yellow());
                    println!("{}", style("👋 Goodbye! See you soon! 👋").bold().green());
                    println!("🌈 {} will be waiting for your return! 🌈", nybbler.name);
                    break;
                }
            },
            _ => unreachable!(),
        }

        // Short delay to see the action result
        thread::sleep(Duration::from_millis(1000));
    }

    Ok(())
}

fn confirm_exit() -> Result<bool, std::io::Error> {
    let confirm = dialoguer::Confirm::new()
        .with_prompt("🥺 Are you really sure you want to leave? Your Nybbler will miss you! 🥺")
        .default(false)
        .interact()?;

    Ok(confirm)
}