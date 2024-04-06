set boxwidth 1
set style fill solid 0.5
set grid ytics
set xrange [90:150]
set yrange [0: 1.0]
set xlabel "CPU cycles"
set ylabel "Probability Density"
datafile = 'points_intr_latency.dat'
plot datafile using 1:2  smooth frequency w boxes t ""