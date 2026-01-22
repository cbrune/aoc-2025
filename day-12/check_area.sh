#!/bin/bash
cat sample.txt | awk '
BEGIN { presenting = 0 }
/^[0-9]+:/ { 
    presenting = 1
    next
}
presenting == 1 && /^[#.]/ {
    gsub(/\./, "")
    count = gsub(/#/, "")
    total += count
}
presenting == 1 && /^$/ {
    print "Present squares:", total
    total = 0
}
/^[0-9]+x[0-9]+:/ {
    split($0, parts, ":")
    split(parts[1], dims, "x")
    area = dims[1] * dims[2]
    split(parts[2], counts, " ")
    present_area = 0
    for (i = 2; i <= length(counts); i++) {
        present_area += counts[i] * 5  # Assuming 5 squares per present on average
    }
    print "Region", dims[1] "x" dims[2], "area:", area, "present estimate:", present_area
}
'
