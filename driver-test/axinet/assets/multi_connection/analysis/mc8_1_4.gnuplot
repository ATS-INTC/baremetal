
fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.1f", v)

set style histogram clustered gap 1
set style data histograms
# set key left top
unset key
set xtics nomirror
set ytics nomirror
set grid ytics
set grid linetype 0
set boxwidth 1.0
set yrange [880 : 940]
set ytics 10
set xtics -1,8,1
set xtics("0" 0, "1" 1, "2" 2, "3" 3, "4" 4, "5" 5, "6" 6, "7" 7)
set xlabel "Priority"
set ylabel 'Latency({/symbol \155}s)'

datafile1 = 'mc8_1_4.dat'

plot \
    for [COL=2:3] datafile1 using COL title columnheader fs fill solid 1 border -1, \
    datafile1 u ($0-1-1./6):2:(fn($2)) w labels font fontSpec(10) right rot by 90 offset char -0.5, 2.0 t ''
