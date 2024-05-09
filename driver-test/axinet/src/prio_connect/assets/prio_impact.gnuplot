set terminal pdfcairo enhanced size 8in,6in font "Arial,24"
set output 'prio_impact.pdf'

fn(v) = sprintf("%.0f", v)
sub(a, b) = a-b

set key left top Left reverse at graph 0.1, 1.0
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
set size 1.0, 1.0

plot datafile using 2 with linespoints t 'base', '' \
              using 3 with linespoints t 'priority-0', '' \
              using 4 with linespoints t 'priority-1', '' \
              using 5 with linespoints t 'priority-2', '' \
              using 6 with linespoints t 'priority-3', \
