set boxwidth 1
set noborder
set grid ytics
set xrange [0:14000]
set xtics nomirror
set ytics nomirror
set xtics 2000
set yrange [0.00002: 0.001]
set xtics rot by -45 offset -2, 0
# set xlabel "Latency({/symbol \155}s)"
# set ylabel "Probability Density"

datafile0 = 'prio_0_freq.dat'

set style fill transparent solid 1 border -2
plot datafile0 using 1:2 smooth frequency w filledcurves fillcolor rgb "#3b6291" t "priority-0", \
