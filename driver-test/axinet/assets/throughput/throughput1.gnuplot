set key left
set grid y
set style data histograms
set style histogram rowstacked
set boxwidth 0.5
set style fill solid 1.0 border -1
set xtics border in scale 1,0.5 nomirror rotate by -45  autojustify
set tics nomirror
set xrange [ * : * ] noreverse writeback
set yrange [ * : * ] noreverse writeback
set ylabel "Throughput(Gbits/s)"
set xlabel 'Frame Size(Bytes)'

$Data <<EOD
Frame             Interrupt             ATSINTC           Poll 
60                0.027                 0.002             0.002
64                0.029                 0.002             0.003
128               0.057                 0.004             0.003
256               0.110                 0.009             0.005
512               0.205                 0.016             0.005
1000              0.352                 0.023             0.010
1024              0.362                 0.026             0.008
1514              0.478                 0.028             0.007
2000              0.579                 0.027             0.014
2048              0.581                 0.035             0.020
EOD

plot $Data using 2:xtic(1) title columnheader(2), for [i=3:4] '' using i title columnheader(i)
