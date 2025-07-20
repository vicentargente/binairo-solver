# Binairo+ Solver 🦓

This is a solver for the logic puzzle game **Binairo**, also known as **Tango**. The core solver is written in **Rust** and is compiled to **WebAssembly (WASM)**, allowing it to run directly and efficiently in any modern web browser.

The frontend provides an interactive grid where you can set up any Binairo puzzle, including special wall constraints, and watch the final solved result.

-----

## 🚀 Getting Started

To run this project locally, you will need a few tools.

### Prerequisites

  * **npm**: The project uses npm to manage dependencies and run scripts. It comes bundled with [Node.js](https://nodejs.org/).
  * **wasm-pack**: This is a tool for building and packaging Rust-generated WebAssembly. You can find installation instructions on the official [wasm-pack website](https://rustwasm.github.io/wasm-pack/installer/).
  * **http-server**: A simple, zero-configuration command-line HTTP server. You can install it globally via npm:
    ```bash
    npm install -g http-server
    ```
    You are free to use another one and set it up in the scripts at package.json.

### Building the Project

The build process compiles the Rust solver into a WebAssembly package that the web application can use.

1.  **Clone the repository**:
    ```bash
    git clone <your-repository-url>
    cd binairo-solver
    ```
2.  **Build the WebAssembly package**:
    This command compiles the Rust code located in the `rust-solver` directory and places the output in `docs/pkg`.
    ```bash
    npm run build
    ```

### Serving the Application

Once the project is built, you can start a local server to view the application in your browser.

1.  **Start the server**:
    This command serves the static files from the `docs` directory.
    ```bash
    npm run serve
    ```
2.  **Open your browser**:
    Navigate to the local address provided by `http-server` (usually `http://127.0.0.1:8080`).

-----

## 🎮 How to Use the Solver

The interface is designed to be simple and intuitive.

1.  **Set the Board Size**: Use the "Board Size" input to define the grid dimensions. The size must be an even number.
2.  **Set Initial Pieces**: Click on any cell to cycle through its state:
      * **Empty** ➡️ **Black** ⚫ ➡️ **White** ⚪ ➡️ **Empty**
3.  **Set Wall Constraints**: Click on the walls between cells to define a constraint between them:
      * **Normal** (no line) ➡️ **Equals** (=) ➡️ **Not-Equals** (×) ➡️ **Normal**
4.  **Solve\!**: Once you have set up the initial board state, just click the **"Solve"** button. The solver will immediately get to work, and the board will update with the final solution once found.

If the solver finds a solution, the board will be filled in, and the initially placed pieces will be highlighted for clarity. If no solution exists, an error message will be displayed.

<div style="display: flex; flex-wrap: wrap; gap: 10px; justify-content: center;">
    <img alt="Initial puzzle state" src="https://i.imgur.com/sjaVk9s.jpeg" style="max-width: 35%; height: auto;">
    <img alt="Initial puzzle state" src="https://i.imgur.com/0Ogen1Z.jpeg" style="max-width: 35%; height: auto;">
</div>

-----

## ⚖️ License

This project is licensed under the MIT License. See the `LICENSE` file for details.