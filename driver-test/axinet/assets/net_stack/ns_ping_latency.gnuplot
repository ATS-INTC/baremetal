
fontSpec(s) = sprintf("Times-Roman, %d", s)
set key right top
set xtics nomirror
set xrange [-0.5 : 0.5]
set ytics nomirror
set grid ytics
set style data histograms
set boxwidth 0.8
unset xtics
set mytics
set ylabel 'Latency({/symbol \155}s)'
set xlabel 'Ping RTT'

set style histogram clustered gap 1
set yrange [* : *]
fn(v) = sprintf("%.2f", v)

plot \
    for [COL=1:4] 'ns_ping_latency.dat' using COL:xticlabels(1) title columnheader fs fill solid 1 border -1
