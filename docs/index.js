import init, { solve_from_js } from './pkg/binairo.js';

// DOM Elements
const sizeInput = document.getElementById('size-input');
const solveButton = document.getElementById('solve-button');
const boardContainer = document.getElementById('board-container');
const status = document.getElementById('status');

// Constants
const wallNameDict = {
    "normal": 'Normal',
    "equals": 'Equals',
    "not-equals": 'NotEquals'
};

// Globals
let boardSize = 6;
let editable = true;

// Validate and adjust board size
function validateSize(size) {
    // Ensure it's even
    if (size % 2 !== 0) {
        size = size + 1;
    }
    // Clamp between min and max
    size = Math.max(2, Math.min(256, size));
    return size;
}

// Function to generate the HTML grid
function generateBoard(size) {
    boardContainer.innerHTML = '';

    // Grid has cells and walls arranged in a pattern
    // For a size x size board, we need (2*size - 1) x (2*size - 1) grid
    const gridSize = size * 2 - 1;
    boardContainer.style.gridTemplateColumns = `repeat(${gridSize}, auto)`;
    boardContainer.style.gridTemplateRows = `repeat(${gridSize}, auto)`;

    for (let r = 0; r < gridSize; r++) {
        for (let c = 0; c < gridSize; c++) {
            const div = document.createElement('div');
            const isCell = r % 2 === 0 && c % 2 === 0;
            const isVWall = r % 2 === 0 && c % 2 !== 0;
            const isHWall = r % 2 !== 0 && c % 2 === 0;
            const isCorner = r % 2 !== 0 && c % 2 !== 0;

            if (isCell) {
                div.className = 'cell';
                div.dataset.row = r / 2;
                div.dataset.col = c / 2;

                // Add piece element
                const piece = document.createElement('div');
                piece.className = 'piece empty';
                piece.dataset.state = 'empty';
                div.appendChild(piece);

            } else if (isVWall) {
                div.className = 'wall vertical';
                div.dataset.state = 'normal';
                div.dataset.row = r / 2;
                div.dataset.col = (c - 1) / 2;

                const symbol = document.createElement('div');
                symbol.className = 'wall-symbol';
                div.appendChild(symbol);

            } else if (isHWall) {
                div.className = 'wall horizontal';
                div.dataset.state = 'normal';
                div.dataset.row = (r - 1) / 2;
                div.dataset.col = c / 2;

                const symbol = document.createElement('div');
                symbol.className = 'wall-symbol';
                div.appendChild(symbol);

            } else if (isCorner) {
                div.className = 'corner';
            }

            boardContainer.appendChild(div);
        }
    }
}

function setNonEditable() {
    editable = false;
    boardContainer.classList.add('non-editable');
    sizeInput.disabled = true;
    solveButton.disabled = true;
}

function setEditable() {
    editable = true;
    boardContainer.classList.remove('non-editable');
    sizeInput.disabled = false;
    solveButton.disabled = false;
}

// Handle clicks using event delegation
boardContainer.addEventListener('click', (e) => {
    if (!editable) return;

    const target = e.target.closest('.cell, .wall');
    if (!target) return;

    if (target.classList.contains('cell')) {
        const piece = target.querySelector('.piece');
        const states = ['empty', 'black', 'white'];
        const current = states.indexOf(piece.dataset.state);
        const next = (current + 1) % states.length;

        piece.dataset.state = states[next];
        piece.className = `piece ${states[next]}`;

    } else if (target.classList.contains('wall')) {
        const states = ['normal', 'equals', 'not-equals'];
        const current = states.indexOf(target.dataset.state);
        const next = (current + 1) % states.length;

        target.dataset.state = states[next];
        target.className = target.className.replace(/(normal|equals|not-equals)/g, '').trim() + ` ${states[next]}`;

        const symbol = target.querySelector('.wall-symbol');
        if (states[next] === 'equals') {
            symbol.textContent = '=';
        } else if (states[next] === 'not-equals') {
            symbol.textContent = '×';
        } else {
            symbol.textContent = '';
        }
    }
});

// Handle size input changes
sizeInput.addEventListener('input', () => {
    if (!editable) return;

    const newSize = validateSize(parseInt(sizeInput.value) || 6);
    if (newSize !== parseInt(sizeInput.value)) {
        sizeInput.value = newSize;
    }
    boardSize = newSize;
    generateBoard(boardSize);
    status.textContent = '';
});

sizeInput.addEventListener('blur', () => {
    if (!editable) return;

    const newSize = validateSize(parseInt(sizeInput.value) || 6);
    sizeInput.value = newSize;
    if (newSize !== boardSize) {
        boardSize = newSize;
        generateBoard(boardSize);
        status.textContent = '';
    }
});

// Solve button functionality
solveButton.addEventListener('click', () => {
    if (!editable) return;

    setNonEditable();

    // 1. Collect data from the DOM
    const initial_cells = [];
    const fixed_cells = [];
    document.querySelectorAll('.cell').forEach(cell => {
        const piece = cell.querySelector('.piece');
        if (piece.dataset.state !== 'empty') {
            initial_cells.push({
                coords: { x: parseInt(cell.dataset.col), y: parseInt(cell.dataset.row) },
                cell: piece.dataset.state.charAt(0).toUpperCase() + piece.dataset.state.slice(1)
            });

            fixed_cells.push(cell);
        }
    });
    console.log(initial_cells);

    const initial_walls = [];
    document.querySelectorAll('.wall').forEach(wall => {
        if (wall.dataset.state !== 'normal') {
            const type = wall.classList.contains('horizontal') ? 'Up' : 'Left';
            const wallState = wallNameDict[wall.dataset.state] || 'Normal';

            const coords = type === 'Up'
                ? { x: parseInt(wall.dataset.col), y: parseInt(wall.dataset.row) + 1 }
                : { x: parseInt(wall.dataset.col) + 1, y: parseInt(wall.dataset.row) };

            initial_walls.push({ coords: { [type]: coords }, wall: wallState });
            // initial_walls.push([{[type]: coords}, wallState]);
        }
    });
    console.log(initial_walls);

    // 2. Call the Rust function (when available)
    status.textContent = 'Solving...';
    try {
        // This is the real call to your Wasm function!
        const result = solve_from_js(boardSize, initial_cells, initial_walls);

        // Update the DOM with the solution
        result.forEach((r, i) => {
            const x = i % boardSize;
            const y = Math.floor(i / boardSize);

            const cell = document.querySelector(`.cell[data-row='${y}'][data-col='${x}']`);
            const piece = cell.querySelector('.piece');
            const state = r.toLowerCase();
            piece.dataset.state = state;
            piece.className = `piece ${state}`;
        });

        for (const cell of fixed_cells) {
            cell.classList.add('initially-selected');
        }

        status.textContent = 'Solved!';
    } catch (error) {
        status.textContent = `Error: ${error}`;
        status.classList.add('error-message');
        setEditable();
    }
});



// Initialize
await init();
generateBoard(boardSize);
