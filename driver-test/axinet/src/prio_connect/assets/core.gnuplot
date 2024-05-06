
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

set xrange [0.5 : 5.5]
set xtics ("1" 1, "2" 2, "4" 3, "8" 4, "16" 5)
set xlabel "Connections"

set logscale y 2
set ylabel 'Latency({/symbol \155}s)'
# set y2range [0 : 2000]
set logscale y2
set y2tics
datafile = 'core.dat'

plot datafile using 2 with linespoints t 'core3', '' using 4 with linespoints t 'core4', \
    datafile using (sub($4, $2)) t 'diff' with histograms linetype -1 fill transparent solid 0.2 border 0 axis x1y2, \
    datafile using ($0):(sub($4, $2)):(fn(sub($4, $2))) t '' with labels font fontSpec(10) offset char 0, 1.0  axis x1y2
