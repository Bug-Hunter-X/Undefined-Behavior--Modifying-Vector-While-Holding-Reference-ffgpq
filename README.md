# Undefined Behavior in Rust Vector

This repository demonstrates a common pitfall in Rust: modifying a vector while holding a reference to one of its elements. This can lead to undefined behavior, resulting in unexpected program crashes or incorrect outputs.

The `bug.rs` file contains the erroneous code. The `bugSolution.rs` file provides a corrected version that avoids the undefined behavior.

## How to reproduce

1. Clone this repository.
2. Navigate to the repository's directory in your terminal.
3. Compile and run `bug.rs` using `rustc bug.rs && ./bug` (you'll likely get a panic or unexpected output).
4. Compile and run `bugSolution.rs` using `rustc bugSolution.rs && ./bugSolution` (this will print '1' correctly).

## Understanding the issue

When you push an element to a vector, the vector may need to resize its internal buffer to accommodate the new element. This resizing involves allocating new memory and copying the existing elements to the new location.  If you hold a reference to an element of the vector while resizing occurs, the reference becomes invalid because it's pointing to memory that has been deallocated. This is what leads to undefined behavior.

## Solution

The solution is to avoid holding references to elements of a vector while modifying it.  Proper use of cloning, ownership and borrowing is crucial to avoid this.