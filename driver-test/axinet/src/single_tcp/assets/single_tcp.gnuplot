set terminal pdfcairo enhanced size 8in,6in font "Arial,24"
set output 'single_tcp.pdf'
fn(v) = sprintf("%.0f", v)
compare(a, b) = a / b
fn2(v) = sprintf("%.2f", v)

set key horizontal center top Left reverse at graph 0.5, 1.125

fn(v) = sprintf("%.0f", v)
compare(a, b) = a / b
fn2(v) = sprintf("%.2f", v)

# unset key
set noborder
set xtics nomirror
set ytics nomirror
set grid ytics
set grid linetype 0

set xrange [0.5 : 5.5]
set xtics ("1" 1, "2" 2, "4" 3, "8" 4, "16" 5)
set xlabel "Matrix-size"

set logscale y 2
set ylabel 'Latency({/symbol \155}s)'
# set y2range [0 : 2000]
# set logscale y2
# set y2tics
datafile = 'single_tcp.dat'
set size 1.0, 0.9
set style fill solid 1 border -1
plot datafile using 2 with histogram title 'TAINTC', '' \
    using 4 with histogram title 'Poll', '' \
    using 6 with histogram title 'Interrupt', \
    datafile using ($0):($4):(fn2(compare($4, $2))) title '' with labels textcolor rgb "#404040" rot by 90 offset char 0, 1.0, \
    datafile using ($0):($6):(fn2(compare($6, $2))) title '' with labels textcolor rgb "#404040" rot by 90 offset char 2, 1.0, \
