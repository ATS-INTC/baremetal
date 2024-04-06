set boxwidth 1
set style fill solid 0.5
set grid ytics
set xrange [210:240]
set xtics 5
set yrange [0: 0.07]
set xlabel "Latency({/symbol \155}s)"
set ylabel "Probability Density"
datafile = ARGV[1]
plot datafile using 1:2  smooth frequency w boxes t ""