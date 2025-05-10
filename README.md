# 🎮 Nybbler: The Terminal Virtual Pet 🐙

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## 🌟 Welcome to Nybbler! 🌟

Nybbler is a charming virtual pet that lives in your terminal! Keep it happy, healthy, and watch it grow. Inspired by the classic virtual pets of the 90s, but with a modern terminal twist!

## 🚀 Features

- 🍔 **Feed your Nybbler** - Keep hunger at bay!
- 🎯 **Play with your Nybbler** - Boost happiness levels!
- 💤 **Let your Nybbler sleep** - Restore energy!
- 💊 **Heal your Nybbler** - Keep sickness away!
- 💾 **Save system** - Your Nybbler persists between sessions!
- 📊 **Dynamic stats** - Watch as hunger, happiness, energy, and health change over time!
- 😊 **Mood system** - Your Nybbler's mood changes based on its stats!

## 🔧 Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/nybbler.git

# Navigate to the project directory
cd nybbler

# Build the project
cargo build --release

# Run the game
cargo run --release
```

## 🎮 How to Play

1. 🆕 **Start the game** - Run `cargo run --release`
2. 👶 **Create a new Nybbler** - Enter a name for your new pet
3. 🎯 **Take care of your Nybbler** - Feed it, play with it, let it sleep, and heal it
4. 💾 **Exit and save** - Your Nybbler will be waiting for you when you return!

## 📝 Game Mechanics

### Mood States
- 😊 **Happy** - High hunger and happiness levels
- 😐 **Neutral** - Average stats
- 😢 **Sad** - Low hunger or happiness
- 🤒 **Sick** - Low health
- 😴 **Sleeping** - Low energy

### Stats
- 🍔 **Hunger** - Decreases over time, increases when fed
- 😄 **Happiness** - Decreases over time, increases when played with
- ⚡ **Energy** - Decreases over time, increases when sleeping
- ❤️ **Health** - Decreases when hunger or happiness are low, restored when healed

## 🌈 Technical Details

Nybbler is built with Rust and uses:
- 📊 `indicatif` - For progress bars
- 💬 `dialoguer` - For interactive prompts
- 🎨 `console` - For terminal styling
- 📅 `chrono` - For time tracking
- 📁 `dirs` - For cross-platform data directory handling

## 📜 License

This project is open source and available under the [MIT License](LICENSE).

## 🙏 Acknowledgements

- The Rust community for awesome libraries
- Classic virtual pets for the inspiration
- You, for taking care of a Nybbler! 🐙

---

Made with ❤️ and Rust