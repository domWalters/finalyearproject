#!/bin/bash

cd report
rm *.pdf
rm *.log

pdflatex main.tex
bibtex main.aux
pdflatex main.tex
pdflatex main.tex

rm *.aux
rm *.toc
rm *.bbl
rm *.blg
