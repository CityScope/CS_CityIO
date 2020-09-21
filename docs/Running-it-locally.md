The current version is developed using rust.

1. install [rust](https://www.rust-lang.org/tools/install)

2. clone this repository

3. clone [sha256 repo](https://github.com/yasushisakai/sha256)

    this repo should live in the same directory as the main repo

4. get db credentials from yasushi, save it in the root of this repo

5. ```cargo run --bin server --release```

  this will compile the program and run it in port 8080

the server will now run locally