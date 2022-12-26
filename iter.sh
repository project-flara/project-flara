#!/bin/bash

for file in $( find ./assets/app/icons/ -type f -name "*.svg");
do
    flatpak run org.inkscape.Inkscape --export-type="png" -w 128 -h 128 $file
done