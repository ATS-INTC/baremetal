set boxwidth 1
set noborder
set grid ytics
set xrange [*:1600]
set xtics nomirror
set ytics nomirror
set xtics 100
set yrange [0.0001: 0.008]
set xtics rot by -45 offset -2, 0
# set xlabel "Latency({/symbol \155}s)"
# set ylabel "Probability Density"

datafile0 = 'core-3/p1-t4-m4/prio_0_freq.dat'
datafile1 = 'core-4/p1-t4-m4/prio_0_freq.dat'


set style fill transparent solid 0.8 border -2
plot datafile0 using 1:2 smooth frequency w filledcurves t "core-3", \
     datafile1 using 1:2 smooth frequency w filledcurves t "core-4", \
