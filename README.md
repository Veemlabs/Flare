

# Flare Compiler Language  (fusion + scheduling)


### Fusion and Scheduling

`@fusion` or `@schedule` blocks control how Flare merges operations into one GPU kernel.

- `elementwise: true` — allow elementwise ops to fuse.
- `barriers` — list ops that break fusion (e.g. matmul, conv).
- `tile`, `vectorize`, `unroll`, `threads`, `memory` — hardware tuning hints.


```flare
@fusion {
  elementwise: true
  barriers: [matmul]
  tile = [BLOCK_M=128, BLOCK_N=128, BLOCK_K=32]
  vectorize = 4
  unroll = 2
  threads = [x=16, y=16]
  memory = { a: shared, b: shared, c: global, x: register }
}
kernel mm_add_relu(a: tensor<f32,[M,K]>, b: tensor<f32,[K,N]>, c: tensor<f32,[M,N]>) -> tensor<f32,[M,N]> {
    let mm = matmul(a, b)
    let y  = mm + c
    return relu(y)
}
```
