
fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.0f", v)
sub(a, b) = a-b

set key left top
# unset key
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
# set y2range [0 : 2000]
set logscale y2
set y2tics
datafile = 'prio_impact.dat'

plot datafile using 2 with linespoints t 'base', '' using 4 with linespoints t 'priority', \
    datafile using (sub($2, $4)) t 'diff' with histograms linetype -1 fill transparent solid 0.2 border 0 axis x1y2, \
    datafile using ($0):(sub($2, $4)):(fn(sub($2, $4))) t '' with labels textcolor rgb "#404040" font fontSpec(10) offset char 0, 1.0  axis x1y2
