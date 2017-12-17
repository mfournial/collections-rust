# Collections and algorithms for Rust-lang

[![Crates version](https://img.shields.io/crates/v/collections-more.svg)](https://crates.io/crates/collections-more)
[![Documentation](https://docs.rs/collections-more/badge.svg)](https://docs.rs/collections-more/)
[![Build Status](https://travis-ci.org/mfournial/collections-rust.svg?branch=master)](https://travis-ci.org/mfournial/collections-rust)
[![Pipeline Status](https://gitlab.doc.ic.ac.uk/mmf115/collections-rust/badges/master/build.svg)](https://gitlab.doc.ic.ac.uk/mmf115/collections-rust/commits/master)
[![codecov](https://codecov.io/gh/mfournial/collections-rust/branch/master/graph/badge.svg)](https://codecov.io/gh/mfournial/collections-rust)
[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)

<!-- Backup docs: [![Backup docs](https://img.shields.io/readthedocs/pip.svg)](https://gitlab.doc.ic.ac.uk/mmf115/collections-rust/-/jobs/artifacts/master/download?job=doc) -->

This crate is an extension of the rust collections crate. If you're used to C++ 
algorithms and the various Java collections you'll feel just at home.  
You should use this crate if you want to use some general purpose data 
structures very easily. Each of them have been tested, and the concurrent 
versions are thread safe. So scale your project and use lighting fast DS with 
this crate!  

**Celebs talking about this project:**
> The best open source project yet (Linus Torvalds)  
> I wish I had this idea for Microsoft (Bill Gates)  
> Collections-more is a true revolution (Steve Jobs at RustConf 2017)  

## Table of contents

- [Install](#install)
- [Usage](#usage)
- [Contribute](#contribute)
- [License](#license)

## Install

Add this line to your `cargo.toml` in your `[dependencies]` section:  

```toml
[dependencies]
# ...
collections_more = "X.X.X"
```
replacing `X.X.X` with the version number with the one on the badge above and 
you should be good to go!

## Usage

Usage is still being decided and shall for now be marked as a work in progress. 
We have to evaluate what's the Rust way of packaging easily before committing 
ourselves to a design.  
As for example of how to use the different data structures, please refer to the 
integration tests in the tests folder that show exactly how everything will be 
holding on in time. The [documentation](https://docs.rs/collections-more/) 
should be very useful to guide you on how to use the various functionalities of 
this project.  
The full details of what is currently available in this library can be found in 
the documentation, this document being only a facade to the project. 
Nevertheless it is important to give a few example to understand the purpose of 
this library so here they are:

```rust
extern crate collections_more;

// snip

use collections_more::priority_queue::PriorityQueue;

fn main() {
	let pq = PriorityQueue::pqueue!(1, 5, 3); // Creates a priority_queue using the macro
	asserteq!(Some(5), pq.poll()); // Gets the biggest element of the queue O(1)
	asserteq!(2, pq.len()); // Size of the queue after removing biggest element
}
```

## Contribute

We most welcome contributions from you. Issue is generally a good safe way to
go. For more active participation, forking the project and opening a pull 
request is also acceptable. For security related issues, a more direct email is 
recommended (email in `cargo.toml` file).  
You may take a look at the code of conduct that apply as some sort of by-law 
between participants. If you have anything to report please send an email at 
the address in the `cargo.toml` file.

## License

MIT © 2017 - mfournial Mayeul (Mike) Fournial
