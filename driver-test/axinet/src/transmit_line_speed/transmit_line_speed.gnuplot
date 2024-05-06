
fontSpec(s) = sprintf("Times-Roman, %d", s)
fn(v) = sprintf("%.0f", v)
set key left top font ",12"
set noborder
set style data histogram
set style histogram clustered gap 1
set style fill solid 1 border -1
set xrange [0.6 : 1.4]
unset xtics
set ytics 500
set ytics nomirror
set boxwidth 0.8
set grid linetype 0 linewidth 1 linecolor rgb "gray"

set label fn(850) at 0.725,900 textcolor rgb "#404040" font fontSpec(10)
set label fn(790) at 0.975,850 textcolor rgb "#404040" font fontSpec(10)
set label fn(1400) at 1.225,1450 textcolor rgb "#404040" font fontSpec(10)

datafile = 'transmit_line_speed.dat'

plot datafile \
    using 1 title "ATSINTC", '' \
    using 2 title "Poll", '' \
    using 3 title "Interrupt"