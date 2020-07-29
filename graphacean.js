// Rust wasm-bindgen exports all functions at the top-level, making functions
// prone to name collisions. This file re-exports most of the functions except
// for certain ones that are grouped into a frozen object, as is done in
// Grapheme.

import init, * as module from './pkg/graphacean.js'

export default init

export const { Complex } = module

function getPrefixedExports (prefix) {
  const obj = Object.create(null)
  for (const name in module) {
    if (name.startsWith(prefix)) {
      obj[name.slice(prefix.length)] = module[name]
    }
  }
  return Object.freeze(obj)
}

export const ComplexFunctions = getPrefixedExports('ComplexFunctions_')
