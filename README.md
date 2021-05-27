# Overview

I wanted to implement an random key generator for an api that I wrote. 
This project taught me about how we generate such keys, and some basic ways
to hash information for use on the internet. I was exposed to base64,
random number seeds, and random number generators made for cryptography.

In this project, I use a number cipher called Hc128. It was designed by
Hongjun Wu. Essentially, what happened is that it took a bunch of
randomly generated seed in a u8 array and then generated 4 u64 numbers
that would be encoded in base64. It then exported it to a .txt file that
would save these keys for verfication for later. I then exported the
keys to be saved to a database later. 


[Software Demo Video](https://youtu.be/Cqx4-DbuuS0)
* Length: 10:45 
  * A lot of information in this video, I wanted to make sure it was comprehensive enough that people could follow the code within it.    

# Development Environment

## Tools
* Visual Studio Code
* Cargo
* repl.it (this was used for testing my rust)

## Libraries (see .toml) 
* base64 = "0.13.0" (we want to encode our u64 numbers)
* rand_hc = "0.3.0"
* rand_core = "0.6.2" (this was for our seeding generator we want to possibly want to make in the future)
* rand_chacha = "0.3.0"
* rand = "0.8.3" 

I programmed in RUST, a language based off of C/C++. It was super interesting, the errors were
cryptic, but familiar enough to build straight off way if you had experience with a C-type
of programming language. 

# Useful Websites / Sources
### general resources 
* [rust doc on struct](https://doc.rust-lang.org/rust-by-example/std/vec.html)
* [tutorial point on struct](https://www.tutorialspoint.com/rust/rust_structure.htm)
* [writing to file](https://doc.rust-lang.org/std/fs/struct.File.html)
* [making a thread sleep](https://stackoverflow.com/questions/28952938/how-can-i-put-the-current-thread-to-sleep)
* [Overview of file operations](https://www.tutorialspoint.com/rust/rust_file_input_output.htm)
* [How to append api keys to end of a file](https://stackoverflow.com/questions/30684624/what-is-the-best-variant-for-appending-a-new-line-in-a-text-file)

### resources on api key generation
* [general process for api key](https://stackoverflow.com/questions/14412132/whats-the-best-approach-for-generating-a-new-api-key)
* [investigation into what public vs private keys](https://stackoverflow.com/questions/55009503/how-services-generate-and-use-public-and-secret-api-keys/61301438#61301438)
* [generating numbers from Hc128](https://stackoverflow.com/questions/62923895/how-do-i-generate-random-numbers-using-hc-128-in-rust)
* [how to base64 data](https://crates.io/crates/base64)
* [how can i convert array Vec u8 nto string?](https://stackoverflow.com/questions/58051863/convert-u8-array-to-base64-string-in-rust)
* [how to map across vector -> turn element into string](https://users.rust-lang.org/t/how-do-i-convert-vec-of-i32-to-string/18669/10)
* [methods/impl of HC128](https://rust-random.github.io/rand/rand_hc/struct.Hc128Rng.html)

### how to slice a vector (conversion from vector -> array)
* [slice array](https://stackoverflow.com/questions/25428920/how-to-get-a-slice-as-an-array-in-rust)
* [template form of any type vector -> same array type](https://users.rust-lang.org/t/convert-vec-to-array-with-tryfrom/50727)

### how to return a vector -> getting time in ms
* [method signature for returning an array](https://users.rust-lang.org/t/how-can-i-return-vec-from-function/34405)
* [how can i get the time in ms](https://stackoverflow.com/questions/26593387/how-can-i-get-the-current-time-in-milliseconds)
* [seeding our random number generator to return a random seed each time](https://stackoverflow.com/questions/59020767/how-can-i-input-an-integer-seed-for-producing-random-numbers-using-the-rand-crat)

### Hc128 impl (we needed to implement these function in order for it to work)

* [example of HC128 implmentation from rust doc](https://docs.rs/rand/0.5.0/src/rand/prng/hc128.rs.html#70)
# Future Work

* Look into public keys and private keys.
* Find a better random seed generator. (u64 is fine for our current purposes though)
* Upload the api keys as json file in the future.
