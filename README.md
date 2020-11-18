# life.rs

[game of life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life) implementation in rust, using the standard grid algorithm (as opposed to something like [this](https://www.refsmmat.com/posts/2016-01-25-conway-game-of-life.html)).

```
USAGE:
    life-rs [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --alive-chance <alive_chance>    Chance for cells to be alive in the first generation [default: 0.2]
    -c, --columns <columns>              The number of columns that will be printed to the terminal (same as the number
                                         of columns in the game grid) [default: 80]
    -d, --delay <delay>                  Duration to wait between each generation (milliseconds) [default: 60]
        --iter-count <iter_count>        BENCHMARKING TOOL: num iterations of game
    -r, --rows <rows>                    The number of rows that will be printed to the terminal. Note that the number
                                         of rows in the game grid is twice this amount, as each printed row is two game
                                         rows. [default: 20]
```

![random grid gif](https://user-images.githubusercontent.com/15344581/52524646-2e4d1c00-2c97-11e9-9fd0-188a5013478e.gif)

![glider gun gif](https://user-images.githubusercontent.com/15344581/52524579-5d16c280-2c96-11e9-91c0-8dad3fe36852.gif)
