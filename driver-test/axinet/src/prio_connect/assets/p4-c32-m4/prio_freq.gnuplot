set boxwidth 1
set noborder
set grid ytics
set xrange [*:30000]
set xtics nomirror
set ytics nomirror
set xtics 10000
set yrange [0.00004: 0.0014]
set xtics rot by -45 offset -2, 0
# set xlabel "Latency({/symbol \155}s)"
# set ylabel "Probability Density"

datafile0 = 'prio_0_freq.dat'
datafile1 = 'prio_1_freq.dat'
datafile2 = 'prio_2_freq.dat'
datafile3 = 'prio_3_freq.dat'

set style fill transparent solid 1 border -2
plot datafile0 using 1:2 smooth frequency w filledcurves fillcolor rgb "#3b6291" t "priority-0", \
    datafile1 using 1:2 smooth frequency w filledcurves fillcolor rgb "#943c39" t "priority-1", \
    datafile2 using 1:2 smooth frequency w filledcurves fillcolor rgb "#779043" t "priority-2", \
    datafile3 using 1:2 smooth frequency w filledcurves fillcolor rgb "#624c7c" t "priority-3", \
