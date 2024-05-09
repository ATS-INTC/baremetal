set terminal pdfcairo enhanced size 8in,6in font "Arial,24"
set output "prio.pdf"

fn(v) = sprintf("%.0f", v)
set key left top
set noborder
set style data histogram
set style histogram clustered gap 1
set style fill solid 1 border -1
set xrange [0.5 : 4.5]
set xtics ('P0' 1, 'P1' 2, 'P2' 3, 'P3' 4)
set xtics nomirror

set ytics 50
set yrange [* : *]
set ytics nomirror
set boxwidth 0.8
set grid y linetype 0 linewidth 1 linecolor rgb "gray"
set ylabel 'Latency({/symbol \155}s)'

datafile = 'prio.dat'

plot datafile using 2 title '',\
     datafile using ($0):($2):(fn($2)) title '' with labels textcolor rgb "#404040" offset char -0.5, 1.0