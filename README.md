

# sum17

**Live Demo**: [graphictin.github.io/sum17/](https://graphictin.github.io/sum17/)

A technical prototype and interaction study 
built with <span style="color:orange">**Rust**</span> and <span style="color:red">**Macroquad**</span>. <br>
This project explores `circular constraint` logic and `bi-directional data management` in a 2D game environment.

$\color{Orange}{\textsf{**Rust**}}$

<span style="color:red"></span>
<span style="color:orange"></span>

## 🚀 Core Features
* **Circular Constraint Engine**: A 10-slot array using modulo arithmetic to calculate adjacent triplet sums ($n-1, n, n+1$).
* **Visual-Logic Sync**: Implementation of a **Half-Slot Offset** $-\left(\frac{2\pi}{10} / 2\right)$ to perfectly center textures under mathematical hitboxes.
* **Bi-Directional Interaction**: Seamlessly moving `Number` entities between a linear hand `Vec` and a fixed circular `Option` array.
* **Reactive Feedback**: Real-time color tinting to signal "Overload" states (Sum > 17).

## 🛠 Technical Stack
* **Language**: Rust
* **Framework**: Macroquad / Miniquad
* **Target**: WebAssembly (`wasm32-unknown-unknown`)
* **Host**: GitHub Pages

## 📁 Assets & Licensing
* **Fonts**: Downloaded via **Google Fonts**. Licensed under the **SIL Open Font License (OFL)**.
* **Graphics**: Custom-generated `.png` assets for 10-slice circle geometry.
* **Engine**: Macroquad (MIT/Apache 2.0).

## 🔨 Build Instructions
To compile for the web:
```bash
cargo build --release --target wasm32-unknown-unknown
```
