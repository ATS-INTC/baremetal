
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
set yrange [40:115]
set ytics 5
set xtics ("1" 0, "2" 1, "4" 2, "8" 3, "16" 4)
set xlabel "Matrix Size"
set y2range [100 : 1600]
set y2tics 100
set ylabel 'Latency({/symbol \155}s)'
set y2label 'Latency({/symbol \155}s)'


datafile1 = 'single_loop_workload1.dat'
datafile2 = 'single_loop_workload816.dat'

plot \
    for [COL=2:4] datafile1 using COL title columnheader fs fill solid 1 border -1, \
    datafile1 u ($0-1-1./6):2:(fn($2)) w labels font fontSpec(10) right rot by 90 offset char -3, 2.0 t '' axis x1y1, \
    datafile1 u ($0-1+1./6):3:(fn($3)) w labels font fontSpec(10) right rot by 90 offset char -5.75, 2.0 t '' axis x1y1, \
    datafile1 u ($0-1+1./6):4:(fn($4)) w labels font fontSpec(10) right rot by 90 offset char -3.5, 2.0 t '' axis x1y1, \
    for [COL=2:4] datafile2 using COL title columnheader fs fill solid 1 border -1 axis x1y2, \
    datafile2 u ($0-1-1./6):2:(fn($2)) w labels font fontSpec(10) right rot by 90 offset char 2.75, 2.25 t '' axis x1y2, \
    datafile2 u ($0-1+1./6):3:(fn($3)) w labels font fontSpec(10) right rot by 90 offset char 0.25, 2.25 t '' axis x1y2, \
    datafile2 u ($0-1+1./6):4:(fn($4)) w labels font fontSpec(10) right rot by 90 offset char 2.5, 2.25 t '' axis x1y2
