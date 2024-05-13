set terminal pdfcairo enhanced size 20in,4in font "Arial,24"
set output 'single_tcp_freq.pdf'
set boxwidth 1
set noborder
set grid ytics
set xtics nomirror
set ytics nomirror
set xtics 50
set xtics 
set yrange [0: 0.08]
set style fill transparent solid 0.8 border -1
set key horizontal reverse Left center top at graph 0.5, 1.2

set multiplot layout 1, 5
set origin 0.0, 0.0
set size 0.2, 0.9
set xrange [150:250]
set xlabel 'Matrix-1'
datafile0 = 'atsintc1/atsintc_delay_freq.dat'
datafile1 = 'poll1/poll_delay_freq.dat'
datafile2 = 'intr1/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "", \
    datafile1 using 1:2 smooth frequency w filledcurves t "", \
    datafile2 using 1:2 smooth frequency w filledcurves t "", \

#####################################################
set origin 0.2, 0.0
set size 0.2, 0.9
set xrange [175:275]
set xlabel 'Matrix-2'
datafile0 = 'atsintc2/atsintc_delay_freq.dat'
datafile1 = 'poll2/poll_delay_freq.dat'
datafile2 = 'intr2/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "TAIC", \
    datafile1 using 1:2 smooth frequency w filledcurves t "", \
    datafile2 using 1:2 smooth frequency w filledcurves t "", \

#####################################################
set origin 0.4, 0.0
set size 0.2, 0.9
set xrange [175:300]
set xlabel 'Matrix-4'
datafile0 = 'atsintc4/atsintc_delay_freq.dat'
datafile1 = 'poll4/poll_delay_freq.dat'
datafile2 = 'intr4/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "", \
    datafile1 using 1:2 smooth frequency w filledcurves t "Poll", \
    datafile2 using 1:2 smooth frequency w filledcurves t "", \

#####################################################
set origin 0.6, 0.0
set size 0.2, 0.9
set xrange [325:475]
set xlabel 'Matrix-8'
datafile0 = 'atsintc8/atsintc_delay_freq.dat'
datafile1 = 'poll8/poll_delay_freq.dat'
datafile2 = 'intr8/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "", \
    datafile1 using 1:2 smooth frequency w filledcurves t "", \
    datafile2 using 1:2 smooth frequency w filledcurves t "Interrupt", \

#####################################################
set origin 0.8, 0.0
set size 0.2, 0.9
set xrange [1550:1700]
set xlabel 'Matrix-16'
datafile0 = 'atsintc16/atsintc_delay_freq.dat'
datafile1 = 'poll16/poll_delay_freq.dat'
datafile2 = 'intr16/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "", \
    datafile1 using 1:2 smooth frequency w filledcurves t "", \
    datafile2 using 1:2 smooth frequency w filledcurves t "", \

unset multiplot
