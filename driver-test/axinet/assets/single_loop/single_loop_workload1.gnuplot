
fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.2f", v)

set style histogram clustered gap 1
set style data histograms
set border 0
unset key
set xtics nomirror
set ytics nomirror
set grid ytics
set grid linetype 0

set boxwidth 1.0
set xrange [-0.5 : 2.5]
set yrange [40:95]
set ytics 5

plot \
    for [COL=2:4] 'single_loop_workload1.dat' using COL:xticlabels(1) title columnheader fs fill solid 1 border -1, \
    'single_loop_workload1.dat' u ($0-1-1./6):2:(fn($2)) w labels font fontSpec(10) right rot by 90 offset char -3, 2.0 t '' , \
    'single_loop_workload1.dat' u ($0-1+1./6):3:(fn($3)) w labels font fontSpec(10) right rot by 90 offset char -5.5, 2.0 t '' , \
    'single_loop_workload1.dat' u ($0-1+1./6):4:(fn($4)) w labels font fontSpec(10) right rot by 90 offset char 3, 2.0 t ''
