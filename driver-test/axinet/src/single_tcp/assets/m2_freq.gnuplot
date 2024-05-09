set boxwidth 1
set noborder
set grid ytics
set xrange [100:300]
set xtics nomirror
set ytics nomirror
set xtics 50
set yrange [0: 0.1]
set xtics rot by -45 offset -2, 0
# set xlabel "Latency({/symbol \155}s)"
# set ylabel "Probability Density"

datafile0 = 'atsintc2/atsintc_delay_freq.dat'
datafile1 = 'poll2/poll_delay_freq.dat'
datafile2 = 'intr2/intr_delay_freq.dat'


set style fill transparent solid 0.8 border -2
plot datafile0 using 1:2 smooth frequency w filledcurves t "TAINTC", \
    datafile1 using 1:2 smooth frequency w filledcurves t "Poll", \
    datafile2 using 1:2 smooth frequency w filledcurves t "Interrupt", \
