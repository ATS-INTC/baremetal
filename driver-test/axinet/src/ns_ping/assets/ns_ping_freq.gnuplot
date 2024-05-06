set boxwidth 1
set style fill solid 0.5
set noborder
set grid ytics
set xrange [75:130]
set xtics nomirror
set ytics nomirror
set xtics 5
set yrange [0: 0.2]
# set xlabel "Latency({/symbol \155}s)"
# set ylabel "Probability Density"

datafile1 = 'atsintc/atsintc_delay_freq.dat'
datafile2 = 'poll/poll_delay_freq.dat'
datafile3 = 'intr/intr_delay_freq.dat'

set style fill transparent solid 0.8 border -2
plot datafile1 using 1:2 smooth frequency w filledcurves t "ATSINTC", \
     datafile2 using 1:2 smooth frequency w filledcurves t "Poll", \
     datafile3 using 1:2 smooth frequency w filledcurves t "Interrupt", \
