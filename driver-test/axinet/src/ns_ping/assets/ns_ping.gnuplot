fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.0f", v)
set key left top
set noborder
set style data histogram
set style histogram clustered gap 1
set style fill solid 1 border -1
set xrange [0.6 : 1.4]
unset xtics
set ytics 2
set yrange [88 : 106]
set ytics nomirror
set boxwidth 0.8
set grid linetype 0 linewidth 1 linecolor rgb "gray"

set label fn(90) at 0.725,91 textcolor rgb "#404040" font fontSpec(10)
set label fn(96) at 0.975,97 textcolor rgb "#404040" font fontSpec(10)
set label fn(104) at 1.225,105 textcolor rgb "#404040" font fontSpec(10)

datafile = 'ns_ping.dat'

plot datafile \
    using 1 title "ATSINTC", '' \
    using 2 title "Poll", '' \
    using 3 title "Interrupt"