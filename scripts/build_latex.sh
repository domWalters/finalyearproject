#!/bin/bash

cd "$(dirname "$0")"
cd ../
git submodule update --remote
cd CSProjectReport
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
