# Fearless Concurrency

Initially the Rust team thought that memory safety and preventing concurrency problems are 2 separate challenges. But gradually they learnt that because of the ownership rules and the strong type-system of Rust, many concurrency errors can be caught at compile time rather than at runtime.