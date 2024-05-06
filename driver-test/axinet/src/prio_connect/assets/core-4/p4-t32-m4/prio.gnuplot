fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.0f", v)
set key left top
set noborder
set style data histogram
set style histogram clustered gap 1
set style fill solid 1 border -1
set xrange [0.5 : 4.5]
set xtics ('P0' 1, 'P1' 2, 'P2' 3, 'P3' 4)
set xtics nomirror

set ytics 100
set yrange [* : *]
set ytics nomirror
set boxwidth 0.8
set grid y linetype 0 linewidth 1 linecolor rgb "gray"


datafile = 'prio.dat'

plot datafile using 2 title '',\
     datafile using ($0):($2):(fn($2)) title '' with labels textcolor rgb "#404040" font fontSpec(10) offset char 0, 1.0