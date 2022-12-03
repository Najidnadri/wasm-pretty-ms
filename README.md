
# Pretty-ms, but in wasm.

Wasm version for [pretty-ms](https://github.com/sindresorhus/pretty-ms).



## Example

1. Copy the /pkg file in this [repo](https://github.com/Najidnadri/wasm-pretty-ms) and paste it in your project folder.

2. Load the wasm file and start your web-app.
```
//index.js

import init, { prettyMilliseconds } from "./pkg/wasm_pretty_ms.js";

init().then(() => {
    //Start your application

    let res = prettyMilliseconds(100.400080, {formatSubMilliseconds: true});
    console.log(res); 
    //100ms 400μs 80ns
});
```

3. Or use it anywhere else.
```
//Example.js

import { prettyMilliSeconds } from "./pkg/wasm_pretty_ms.js"

let res = prettyMilliseconds(new Date(2014, 0, 1, 10, 40) - new Date(2014, 0, 1, 10, 5));
console.log(res); 
//35m
```
## Performance

TODO!
*Open for contributions*
