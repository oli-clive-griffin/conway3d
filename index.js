const canvas = document.getElementById('gameCanvas');
const ctx = canvas.getContext('2d');
const width = canvas.width;
const height = canvas.height;

// const cellSize = 10; // Size of the cell in pixels
// const gridWidth = width / cellSize;
// const gridHeight = height / cellSize;

// // Function to draw the grid
// function drawGrid(state) {
//     ctx.clearRect(0, 0, width, height);
//     for (let x = 0; x < gridWidth; x++) {
//         for (let y = 0; y < gridHeight; y++) {
//             ctx.fillStyle = state[x][y] ? 'black' : 'white';
//             ctx.fillRect(x * cellSize, y * cellSize, cellSize, cellSize);
//         }
//     }
// }

async function main() {
    // Adding control to start and stop the game
    const conwayJs = await import('./pkg/conway3d.js')
    const { GameOfLife } = conwayJs
    const res = await fetch('./pkg/conway3d_bg.wasm')
    
    conwayJs.initSync(await res.arrayBuffer())
    const game = GameOfLife.new_rand(10, 10, 10)
    // console.log(game.width)

    let gameInterval;
    document.getElementById('startBtn').addEventListener('click', () => {
        console.log('starting')
        if (!gameInterval) {
            gameInterval = setInterval(() => {
                console.log('render')
                console.log(game.render())
                console.log('tick')
                game.tick()
                // console.log(res.toString().slice(0, 100))
            }, 100); // Update every 100ms
        }
    });
    document.getElementById('tickBtn').addEventListener('click', () => {
        console.log('tick')
        game.tick()
    });
    document.getElementById('stopBtn').addEventListener('click', () => {
        console.log('stop')
        clearInterval(gameInterval);
        gameInterval = null;
    });
}

main()

// let conwayJs;
// import('./pkg/conway3d.js').then((module) => { conwayJs = module
//     fetch('./pkg/conway3d_bg.wasm').then((res) => {
//         res.arrayBuffer().then((bytes) => {
//             // WebAssembly.instantiate(bytes, {}).then((results) => {
//                 // wasmModule = results.instance.exports;
//                 conwayJs.initSync(bytes)
//                 const { GameOfLife } = conwayJs
//                 const game = GameOfLife.new(10, 10, 10)
//                 let gameInterval;
//                 document.getElementById('startBtn').addEventListener('click', () => {
//                     if (!gameInterval) {
//                         gameInterval = setInterval(() => {
//                             game.tick()
//                             const res = game.render()
//                             // console.log(res.toString().slice(0, 100))
//                         }, 100); // Update every 100ms
//                     }
//                 });
//                 document.getElementById('tickBtn').addEventListener('click', () => {
//                     game.tick()
//                 });
//                 document.getElementById('stopBtn').addEventListener('click', () => {
//                     clearInterval(gameInterval);
//                     gameInterval = null;
//                 });
//             // });
//         });
//     })
// })

// Initialize the game or draw initial state
// drawGrid(wasmModule.getInitialState());



// Initialize the scene, camera, and renderer
const scene = new THREE.Scene();
const camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
const renderer = new THREE.WebGLRenderer();
console.log(THREE.OrbitControls)
const controls = new THREE.OrbitControls(camera, renderer.domElement);
controls.enableDamping = true; // Optional: this enables damping (inertia), which can provide a smoother control experience.

renderer.setSize(window.innerWidth, window.innerHeight);
document.body.appendChild(renderer.domElement);

// Define the 3D array of booleans
const grid = [
    [[true, false, true], [false, true, false], [true, false, true]],
    [[false, true, false], [true, false, true], [false, true, false]],
    [[true, false, true], [false, true, false], [true, false, true]]
];

// Define cube geometry and material
const cubeGeometry = new THREE.BoxGeometry();
const cubeMaterial = new THREE.MeshBasicMaterial({ color: 0x00ff00, wireframe: true });

// Loop through the array and add cubes where true
for (let x = 0; x < grid.length; x++) {
    for (let y = 0; y < grid[x].length; y++) {
        for (let z = 0; z < grid[x][y].length; z++) {
            if (grid[x][y][z]) {
                const cube = new THREE.Mesh(cubeGeometry, cubeMaterial);
                cube.position.set(x - grid.length / 2, y - grid[x].length / 2, z - grid[x][y].length / 2);
                scene.add(cube);
            }
        }
    }
}

// Set the camera position
camera.position.z = 5;

// Create the animation loop
function animate() {
    requestAnimationFrame(animate);
    renderer.render(scene, camera);
}

// Start the animation loop
animate();