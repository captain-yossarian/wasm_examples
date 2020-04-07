import * as wasm from "webassembly";

const calculatePerf = (callback) => {
  const start = performance.now();
  callback();
  return performance.now() - start;
}

const arrayGenerator = () =>
  Array(100000).fill(0).map((_, id) => ({
    id: `${id}`,
    name: `Duck#${id}`,
  }))

const hugeArray = arrayGenerator();
const js = () => hugeArray.reduce((acc, _, index) => (index % 42 === 0 ? [...acc, index] : acc), []);
const assembly = () => wasm.filter_array_of_objects(hugeArray);



const js_perf = calculatePerf(js)

const wasm_perf = calculatePerf(assembly)


console.log({ js_perf, wasm_perf });


