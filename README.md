Final Year Project - GA to create & optimise Stock Screeners
===============================================

If you don't mind reading this file in it's raw unformatted form, then simply open it in any text editor. However, I recommend using a program that can display .md (Markdown) files as HTML. For example:

* In the Atom editor you can right click the file and select "Markdown Preview".
* In Chrome there is an appropriate [plugin](https://chrome.google.com/webstore/detail/markdown-preview-plus/febilkbfcbhebfnokafefeacimjdckgl).
* In Python, there is a program called [grip](https://pypi.org/project/grip/) available from the pip package manager.

Program Dependencies
------------
* Cargo, the rust package manager.
* A selection of public rust crates (libraries). These will be pulled the first time that cargo is run.

Cargo has its own dependencies (a rust compiler, a C linker, etc). I have provided a bash script to install everything that is necessary (this script requires apt, and sudo permissions, and has only been tested on Ubuntu). This script may need to be made executable before running it:

```console
$ chmod +x ./scripts/dependency_get.sh
$ ./scripts/dependency_get.sh
```

You may need to give the occassional input whilst this script runs, for example when getting packages from apt you may need to press "y" to confirm the install. When rustup installs, you have to select option "1" and press "Enter".

If this script doesn't work to install the dependencies, you may need to look up how to install cargo on the platform you are on.

Directory Structure
--------------
<pre>
../
├── CSProjectReport  
│   ├── 0_abstract.tex  
│   ├── 10_beforeRatioRemove.tex  
│   ├── 11_afterRatioRemove.tex  
│   ├── 12_evaluativeTesting.tex  
│   ├── 13_evaluativeTestingElitism.tex  
│   ├── 14_conclusions.tex  
│   ├── 15_appendix.tex  
│   ├── 1_introduction.tex  
│   ├── -1_titlepage.tex  
│   ├── 2_proposal.tex  
│   ├── 3_literatureRev.tex  
│   ├── 4_prerequisiteKnowledge.tex  
│   ├── 5_implementation.tex  
│   ├── 6_trainingData.tex  
│   ├── 7_firstFunctionalTest.tex  
│   ├── 8_contextualising.tex  
│   ├── 9_implementationContext.tex  
│   ├── bibliography.bib  
│   ├── images  
│   │   ├── averageAndMax.png  
│   │   ├── drawing-analogy.jpg  
│   │   ├── drawing-system.jpg  
│   │   └── uobLogo.png  
│   ├── main.tex  
│   └── tables  
│       ├── DvsEvES.txt  
│       ├── evtoebitda_analysis.txt  
│       ├── generations-default-full.txt  
│       ├── generations-default.txt  
│       ├── iterations-default-full.txt  
│       ├── iterations-default.txt  
│       ├── lambda-default-full.txt  
│       ├── lambda-default.txt  
│       ├── percentiles-default-full.txt  
│       └── percentiles-default.txt  
├── csv_reader  
│   ├── Cargo.lock  
│   ├── Cargo.toml  
│   ├── src  
│   │   ├── csv_reader_core.rs  
│   │   └── main.rs  
├── data_generator  
│   ├── Cargo.lock  
│   ├── Cargo.toml  
│   └── src  
│       ├── generator.rs  
│       └── main.rs  
├── game  
│   ├── Cargo.lock  
│   ├── Cargo.toml  
│   └── src  
│       ├── data_record.rs  
│       ├── data_trait.rs  
│       ├── game.rs  
│       ├── main.rs  
│       ├── player.rs  
│       ├── quarter.rs  
│       ├── quarters.rs  
│       └── screener.rs  
├── README.md  
├── scripts  
│   ├── build_latex.sh  
│   ├── clean_merge.sh  
│   ├── create_data_folders.sh  
│   ├── delete_extra_data.sh  
│   ├── do_zip.sh  
│   ├── intrinio_pull.py  
│   ├── run_data_unite.sh  
│   └── setup_fake_data.sh  
└── test-data  
    ├── input.txt  
    ├── Intrinio_all_symbols.csv  
    ├── PythonData  
    ├── README.md  
    └── TrimmedUnitedData  
</pre>

The directory "data_generator" and its contents are not used, and can be ignored. The contents of "PythonData" and "TrimmedUnitedData" are ommitted from this directory view, as they each contain thousands of .csv files.

Usage - GA
----------

To run the Genetic Algorithm standalone from the root directory with full default parameters:

```console
$ cd game
$ cargo run -- -run
```

The algorithm has a number of parameters than can be set on the command line.  
Before using "-run", any of the following can be typed:
* "-percentiles [x1,x2,...,xn]" - Use the values x1, x2, ..., xn as percentile gaps. Default: [10]
* "-gen_max [x1,x2,...,xn]" - Use the values x1, x2, ..., xn as generation max. Default: [10]
* "-lambda [x1,x2,...,xn]" - Use the values x1, x2, ..., xn as population sizes. Default: [100]
* "-iterations [x1,x2,...,xn]" - Use the values x1, x2, ..., xn as iteration number. Default: [2]
* "-runs [x]" - Run the algorithm x times. Default: [10]
* "-elitism" - Turn on elitism. Default: Off.
* "-speciation" - Turn on speciation. Default: Off.

I'd also recommend using "tput reset" before running the algorithm to fully clear the terminal window.

Example usage (if viewing as markdown, this panel can be scrolled):
```console
$ cd game
$ tput reset && cargo run -- -percentiles [2] -lambda [175] -iterations [2] -gen_max [12] -runs [20] -elitism -run
```

You can also run the algorithm to evaluate screening strategies that are formatted correctly. You need to provide the percentile gap that was used to generate the strategy.

Example usage (if viewing as markdown, this panel can be scrolled):
```console
$ cd game
$ tput reset && cargo run -- -percentiles [10] -test "[("ebit", Gt, 40), ("fcffgrowth", Lt, 10), ("fcfftointerestex", Gt, 30), ("nopatqoqgrowth", Gt, 20), ("pretaxincomemargin", Lt, 30), ("pricetoearnings", Lt, 30), ("rnnoa", Gt, 20), ("totalcapital", Lt, 20)]"
```

A test can also be run without providing a strategy. In that case, the program will look for a file called "./test-data/input.txt", and use the strategy in there. An example file is provided to show the correct formatting.

Example usage (if viewing as markdown, this panel can be scrolled):
```console
$ cd game
$ tput reset && cargo run -- -percentiles [10] -test
```
