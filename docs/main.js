import init, { JuliaSet } from "../pkg/draw_julia.js";

await init();
const juliaSet = JuliaSet.new({
    re: -0.15,
    im: 0.65
});
const canvas = document.getElementById("julia-set");
juliaSet.draw(canvas, {
    south: -2.0,
    north: 2.0,
    west: -2.0,
    east: 2.0
});