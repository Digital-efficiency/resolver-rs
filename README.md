# resolver-rs [![CI](https://github.com/Digital-efficiency/resolver-rs/actions/workflows/CI.yml/badge.svg?branch=master)](https://github.com/Digital-efficiency/resolver-rs/actions/workflows/CI.yml)

resolver-rs 是一个高效的本地模块解析器，可以根据包名快速获取本地模块的信息。

## 功能特点

- 快速解析：根据包名迅速定位本地模块
- 支持嵌套结构：可处理复杂的 node_modules 目录结构
- 详细信息：提供模块的版本、描述、关键词等完整信息
- Rust 实现：高性能，可靠性强

## 本地测试方法

```js
import { resolve } from "./index.js";

const packageNames = ["lodash", "@types/react", "@babel/core", "vue"];

for (const packageName of packageNames) {
  const result = resolve(packageName);
  console.log(result);
}
```
