# Island Coloring Game

This is a simple game written in Rust where you aim to color an island with a single color. The game randomly generates an island with various colors, and your objective is to change all cells of the island to a single color within a limited number of tries.

## Instructions

1. **Choose Difficulty:** When you run the game, you'll be prompted to choose a difficulty level (easy, medium, hard). This determines the size of the island grid.

2. **Gameplay:** You have a limited number of tries to switch the color of the island to a single color. Each cell of the island has a different color, and by choosing a color, you will infect adjacent cells of the same color until the entire island becomes a single color.

3. **Choose Color:** You can choose the color to switch the island by entering the first letter of the color name (e.g., "r" for Red, "g" for Green, etc.).

4. **Win Condition:** The game ends when either you successfully change the entire island to a single color or you exhaust all your tries.

## Screenshots

### Initial Grid

![Initial Grid](https://i.postimg.cc/LsDbKH5w/Screenshot-2024-03-13-at-12-35-38.png)

### Gameplay

![Gameplay](https://i.postimg.cc/TYSy0mcH/Screenshot-2024-03-13-at-12-40-39.png)

### Game Over

![Game Over](https://i.postimg.cc/9XDCpkNC/Screenshot-2024-03-13-at-12-36-36.png)

## How to Run

To run the game, make sure you have Rust installed on your system. Then, clone this repository and navigate to the project directory in your terminal. Run the following command:

```sh
cargo run -q
```
