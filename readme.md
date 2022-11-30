<a name="readme-top"></a>

<!-- PROJECT LOGO -->
<br />
<div align="center">

<h2 align="center">Chess Agent in Rust</h2>
  <p align="center">
    An intelligent agent to play chess...with an eye to execution performance.
  </p>
</div>

<!-- TABLE OF CONTENTS -->
<details>
  <summary>Table of Contents</summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#getting-started">Getting Started</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

<!-- ABOUT THE PROJECT -->
## About The Project

This was the final project for a Master's level course that was an introduction to Data Science, Machine Learning, and Aritificial Intelligence at MSOE (Milwaukee School of Engineering).

This project implements a few intelligent agents for chess.  
* The `RandomAgent` is not so intelligent; it just selects a random move.  
* The `SimpleAgent` agent looks at every possible move and picks the best using a scoring function, winning about 85% of the time against the `RandomAgent`.  
* The `DepthAgent` agent looks ahead N number of moves.  So, a depth of 1 is the same as `SimpleAgent`.  It dominates both the `RandomAgent` and `SimpleAgent` at depths of 3 and 4 (usually wins 100% of games).
* The `ThreadAgent` agent does the same as the `DepthAgent`, but uses a small thread pool to improve performance.

To determine the best move, a scoring function is used.  The scoring function is based on tables of values for each type of piece on each square (see: https://www.chessprogramming.org/Piece-Square_Tables).  

<p align="right">(<a href="#readme-top">back to top</a>)</p>


### Built With

The Rust programming language.  I'm sure haters will hate and zealots will rave.  My experience with Rust has been really great.

![](https://foundation.rust-lang.org/img/rust-logo-blk.svg)

(I'm a crusty old C programmer, be kind in your code reviews.)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Getting Started

Want to run it locally?  Do this.

### Prerequisites

Install Rust.  It will make you happy.  https://www.rust-lang.org/tools/install

### Installation

1. Clone the repo.
2. Edit the `main.rs` file to setup any agent matches you want.
3. Open a terminal in the root directory.
4. Build and run the program `cargo run --release`  Note: release mode runs significantly faster.
5. If you are interested in performance tuning, uncomment the `#debug = true` line in `Cargo.toml` and use your favorite profiler.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Usage

Edit `main.rs` to run the `SimpleAgent` against a `RandomAgent` (both sides) as a baseline.  Then, we will run a `ThreadAgent` of depth 3 against a `DepthAgent` of depth 1.

```
play_game(Box::new(SimpleAgent::new()), Box::new(RandomAgent::new()), 100);
play_game(Box::new(RandomAgent::new()), Box::new(SimpleAgent::new()), 100);
play_game(Box::new(ThreadAgent::new(3)), Box::new(DepthAgent::new(1)), 100);
```

Then run it.

`cargo run --release`

Output:

```
SimpleAgent vs RandomAgent
  [93, 0, 7], avg 31.2 full turns, avg 1.71ms, total 171.34ms
RandomAgent vs SimpleAgent
  [0, 96, 4], avg 32.1 full turns, avg 1.68ms, total 167.79ms
ThreadAgent(3) vs DepthAgent(1)
  [100, 0, 0], avg 12.5 full turns, avg 162.87ms, total 16.29s
```

In this run, the `SimpleAgent` won most games, and tied a few.  It was lightening fast, finishing a whole game in ~1.7ms.  The `ThreadAgent(3)` beat the `DepthAgent(1)` 100% of the time (silly DepthAgent).  These games lasted only ~160ms.

Results from:
* AMD Ryzen 9  6900HX with Radeon Graphics 3.30 GHz
* 32 GB RAM
* Windows 11 Pro 64-bit

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Roadmap

In my python implementation, I used Monte Carlo sampling (a fancy way of saying just check 5, 10, or N moves vs every possible move) in order to look more moves ahead (depth) in a reasonable time.  With the performance in the Rust implementation, a depth of 3 ran fast (100 games in 15s) and a depth of 4 ran well (100 games in 8 min), so I did not port that code over.

If I have time, I might work to export the Rust code in a C interface so it can be called from Python.  This would allow for playing the Rust agent against the Python agents that most students created.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

<!-- CONTRIBUTING -->
## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and create a pull request. You can also simply open an issue with the tag "enhancement".
Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

Credit for this text: https://github.com/othneildrew/Best-README-Template/blob/master/BLANK_README.md

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Acknowledgments

A few great humans create very nice libaries for the rest of us.  In the chess agent world, these are normally libraries that manage the board and game state so you don't have to write all that code, and you can focus on the Agent logic.  I used a nice library by Jordan Bray.

* GitHub: https://jordanbray.github.io/chess/
* Documentation: https://docs.rs/chess/latest/chess/

<p align="right">(<a href="#readme-top">back to top</a>)</p>
