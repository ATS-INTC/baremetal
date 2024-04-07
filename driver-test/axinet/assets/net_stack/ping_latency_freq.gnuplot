set boxwidth 1
set style fill solid 0.5
set grid ytics
set xrange [75:120]
set xtics nomirror
set ytics nomirror
set xtics 5
set yrange [0: 0.20]
set xlabel "Latency({/symbol \155}s)"
set ylabel "Probability Density"

datafile1 = 'points_poll_delay.dat'
datafile2 = 'points_intr_delay.dat'
datafile3 = 'points_embassy_delay.dat'
datafile4 = 'points_atsintc_delay.dat'

set style fill transparent solid 0.8 border -1
plot datafile1 using 1:2 smooth frequency w filledcurves t "Smoltcp-Poll", \
     datafile2 using 1:2 smooth frequency w filledcurves t "Smoltcp-Intr", \
     datafile3 using 1:2 smooth frequency w filledcurves t "Embassy", \
     datafile4 using 1:2 smooth frequency w filledcurves t "Embassy-ATSINTC"
