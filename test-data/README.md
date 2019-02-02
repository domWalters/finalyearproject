### File Properties

This zip file contains formatted data for acceptable stocks in alphanumeric indexes 10000-10727 from the Intrinio Platform.
Stocks are only downloaded from the IntrinioAPI if the following conditions are met:

* The "last\_filing\_date" field in the Intrinio "all\_companies" list has an entry.
* The entry in "last\_filing\_date" was from 2018.
* The files "prices", "financials\_calculations", "financials\_cash\_flow\_statement", and "financials\_balance\_sheet" all exist on the IntrinioAPI and are downloadable.

This set of assumptions allows me to minimise my calls to the IntrinioAPI, and maximise the amount of stocks for which I can get complete information (stocks where bullet point #3 is satisfied).

Furthermore, the following properties are __true__ for __all__ files in this zip:

* Every file contains the same set of columns in ascending alphanumerical order (i.e in every file in this zip the column "period" is in the same position, in every file in this zip the column "year" is in the same position).
* Every file has a continuous set of rows (no quarter is missed).

To accomodate these properties, __certain data entries have been ommitted__. I still have the files that this dataset is based off of, so this data can still be obtained. However, it will not be in this format.

### Future Concerns

Due to the nature of how my program runs, in future zips certain columns may be removed (if any stock I download doesn't contain the datapoint to which that column refers). Do not assume a column is in a specific column number (i.e. "year" may not always be in column EK). However, as said before, you __can__ assume that the columns are in ascending alphabetical order.
