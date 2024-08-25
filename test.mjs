import { resolve } from "./index.js";

const packageNames = ["lodash", "@types/react", "@babel/core", "vue"];

for (const packageName of packageNames) {
  const result = resolve(packageName);
  console.log(result);
}