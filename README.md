# 🏴‍☠️ Minesweeper

A classic **Minesweeper** game playable directly in the **command line**, built in **Rust**.

## 🎯 Features

✔ Classic Minesweeper mechanics (flagging, revealing cells, checking for mines)  
✔ Randomly generated minefield with adjustable grid size and mine count  
✔ Fully interactive CLI with simple text input  
✔ Clear and aligned board display for better readability  
✔ Win and loss detection

## 🛠 Installation & Setup

### 1️⃣ Prerequisites

- [**Rust**](https://rust-lang.org/tools/install/) installed
- A terminal supporting standard input/output

### 2️⃣ Clone the Repository

```sh
git clone https://github.com/thomasbtho/Minesweeper.git
cd Minesweeper
```

### 3️⃣ Run the Game

```sh
cargo run
```

## 🎮 How to Play

1️⃣ Enter **three values**: `x y action` to play, or type `quit` or `exit` to exit.

- `x` and `y` are the **coordinates** (starting from 1).
- `action` can be:
    - `free` or `reveal` → Reveal the cell
    - `flag` or `mine` → Mark a possible mine

2️⃣ Example input:

```sh
3 5 free
```

3️⃣ **Win condition**: Reveal all non-mine cells.  
4️⃣ **Lose condition**: Reveal a mine.

---
**Author:** [thomasbtho](https://github.com/thomasbtho)  
🎮 Happy mining! 🏴‍☠️
