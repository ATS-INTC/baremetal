set terminal pdfcairo enhanced size 15in,6in font "Arial,24"
set output 'transeiver.pdf'

set key horizontal center top Left reverse autotitle nobox at graph 1.15, 1.2
set style data histograms
set style fs solid 1 border -1
set xtics border in scale 1,0.5 nomirror autojustify

set xrange [ 0 : * ]
set yrange [ * : * ]
set ytics nomirror

set multiplot layout 1, 2

datafile1 = 'transmit_cycles.dat'
set origin 0.0, 0.05
set size 0.5, 0.9
set title 'CPU cycles'
set xlabel 'a'
plot datafile1 using 2:xtic(1) t 'TAIC', '' \
    using 3:xtic(1) t 'Poll', '' \
    using 4:xtic(1) t 'Interrupt'

datafile2 = 'transeiver_latency.dat'
set origin 0.5, 0.05
set size 0.5, 0.9
set yrange [ 30 : * ]
set title 'Latency({/symbol \155}s)'
set xlabel 'b'
plot datafile2 using 2:xtic(1) t '', '' \
    using 3:xtic(1) t '', '' \
    using 4:xtic(1) t ''
