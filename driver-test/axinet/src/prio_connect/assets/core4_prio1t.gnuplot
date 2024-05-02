
fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.0f", v)

# set key left top
unset key
set noborder
set xtics nomirror
set ytics nomirror
set grid ytics
set grid linetype 0

set xrange [0.5 : 7.5]
set xtics ("4" 1, "8" 2, "16" 3, "32" 4, "64" 5, "128" 6, "256" 7)
set xlabel "Connections"

set logscale y 2
set ylabel 'Latency({/symbol \155}s)'

datafile = 'core4_prio1t.dat'

plot datafile using 2 with linespoints, \
    datafile u ($0):2:(fn($2)) w labels font fontSpec(10) right rot by 0 offset char 1.5, -0.5 t ''
