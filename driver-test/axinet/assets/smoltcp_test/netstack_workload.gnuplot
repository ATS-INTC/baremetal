
fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.1f", v)

set style histogram clustered gap 1
set style data histograms
set key left top
set xtics nomirror
set ytics nomirror
set grid ytics
set grid linetype 0
set boxwidth 1.0
set xtics 0,6,1
set yrange [180:440]
set ytics 20
set xtics ("1" 0, "2" 1, "4" 2, "8" 3)
set xlabel "Matrix Size"
set ylabel 'Latency({/symbol \155}s)'


datafile1 = 'netstack_workload.dat'

plot \
    for [COL=2:3] datafile1 using COL title columnheader fs fill solid 1 border -1, \
    datafile1 u ($0-1-1./6):2:(fn($2)) w labels font fontSpec(10) right rot by 90 offset char -0.5, 2.0 t '' axis x1y1, \
    datafile1 u ($0-1+1./6):3:(fn($3)) w labels font fontSpec(10) right rot by 90 offset char -0.5, 2.0 t '' axis x1y1
