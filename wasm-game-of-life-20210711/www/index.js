//import * as wasm from "hello-wasm-pack";
//import * as wasm from "wasm-game-of-life";
//wasm.greet();
import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

function renderLoop() {
	pre.textContent = universe.render();
	universe.tick();
	window.requestAnimationFrame(renderLoop);
}

window.requestAnimationFrame(renderLoop);

