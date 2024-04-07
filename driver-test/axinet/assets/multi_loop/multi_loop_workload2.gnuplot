
fontSpec(s) = sprintf("Times-Roman, %d", s)
unset key
set border 0
set xtics nomirror
set xrange [-0.5 : 0.5]
set ytics nomirror
set grid ytics
set style data histograms
set boxwidth 1.0
set style histogram clustered gap 1
set size 0.4, 1
fn(v) = sprintf("%.2f", v)

plot \
    for [COL=2:4] 'multi_loop_workload2.dat' using COL:xticlabels(1) title columnheader fs fill solid 1 border -1, \
    'multi_loop_workload2.dat' u ($0-1-1./6):2:(fn($2)) w labels font fontSpec(10) right rot by 90 offset char -3, 2.5 t '' , \
    'multi_loop_workload2.dat' u ($0-1+1./6):3:(fn($3)) w labels font fontSpec(10) right rot by 90 offset char -5.5, 2.5 t '' , \
    'multi_loop_workload2.dat' u ($0-1+1./6):4:(fn($4)) w labels font fontSpec(10) right rot by 90 offset char 3, 2.5 t ''