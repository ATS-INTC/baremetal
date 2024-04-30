
# fontSpec(s) = sprintf("Times-Roman, %d", s)
# set key left top
# # set xtics nomirror
# set xrange [-0.5 : 0.25]
# set ytics nomirror
# set grid ytics
# set style data histograms
# set boxwidth 0.5
# unset xtics
# set ytics 500
# set xlabel 'The minimum bytes to reach the line speed'

# set style histogram clustered gap 0
# set yrange [* : *]
# fn(v) = sprintf("%.2f", v)
# datafile = 'transmit_line_speed.dat'

# plot \
#     for [COL=1:4] datafile using COL title columnheader fs fill solid 1 border -1, \
#     datafile u ($0-1-1./6):2:(fn($1)) w labels font fontSpec(10) right rot by 90 offset char -5, 2.0 t '' axis x1y1, \
#     datafile u ($0-1-1./6):2:(fn($2)) w labels font fontSpec(10) right rot by 90 offset char -3, 2.0 t '' axis x1y1, \
#     datafile u ($0-1-1./6):2:(fn($3)) w labels font fontSpec(10) right rot by 90 offset char -1, 2.0 t '' axis x1y1, \

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

set label fn(850) at 0.725,900 font fontSpec(12)
set label fn(790) at 0.975,850 font fontSpec(12)
set label fn(1400) at 1.225,1450 font fontSpec(12)

datafile = 'transmit_line_speed.dat'

plot datafile \
    using 1 title "ATSINTC", '' \
    using 2 title "Poll", '' \
    using 3 title "Interrupt"