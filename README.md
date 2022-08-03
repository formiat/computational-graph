# Computational graph task

## Description

Computational graph is a direct acyclic graph, where each node takes one or multiple inputs
and produces one or more outputs. All inputs and outputs are floating point values (`f32`).

For example:
`y = x1 + x2 * sin(x2 + pow(x3, 3))`

where `x1`, `x2`, `x3` are computation graph inputs and `y` is a graph output. `3` is the exponent for
power function.

## Task

The task is to create a computational graph structure which can:

- Take an arbitrary number of inputs
- Can hold arbitrary graph of operations
- Can compute a graph with `compute()` method
- Each node should have its own cache (the result of `compute()` method) and should
   invalidate caches of dependent nodes. For example, if `x1` changes in the above
   example only the last `sum(a,b)` has to be recomputed.
