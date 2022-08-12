# draw-julia

Draw julia set

## Build

```bash
wasm-pack build --target web --scope toriyama
cd pkg
npm publish --access=public
```

## Usage

```javascript
import init, { JuliaSet } from "@toriyama/draw-julia";

await init();
const juliaSet = JuliaSet.new({
	re: -0.15,
	im: 0.65,
});
const canvas = document.getElementById("julia-set");
juliaSet.draw(canvas, {
	south: -2.0,
	north: 2.0,
	west: -2.0,
	east: 2.0,
});
```
