Final Year Project - GA to create & optimise Stock Screeners
===============================================

Initialisation
--------------

To set up the project, and run it for the first time:

```console
$ python scripts/intrinio_pull.py
$ ./scripts/clean_merge.sh
$ ./scripts/build_latex.sh
$ cd game
$ cargo run
```

Usage - GA
----------

To run the Genetic Algorithm standalone from the root directory:

```console
$ cd game
$ cargo run
```

Usage - Pull Intrinio Data
--------------------------

To pull data from Intrinio:

```console
$ python scripts/intrinio_pull.py
```

Usage - Convert Intrinio data to Formatted data
-----------------------------------------------

To format the data from Intrinio:

```console
$ ./scripts/clean_merge.sh
```
