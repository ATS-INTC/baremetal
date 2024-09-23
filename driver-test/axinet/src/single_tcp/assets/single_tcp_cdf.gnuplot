set terminal pdfcairo enhanced size 15in,5in font "Arial,24"
set output 'single_tcp_cdf.pdf'
set boxwidth 1
set grid ytics
set xtics 50
set mxtics 5
set ytics 0.2
set mytics 2
set yzeroaxis
set ylabel 'CDF'

set multiplot layout 2, 3 title "Latency({/symbol \155}s)" 
set key samplen 1 Right center bottom at graph 0.2, 0.5
set xrange [100:300]
datafile0 = 'atsintc1/atsintc_delay_freq.dat'
datafile1 = 'poll1/poll_delay_freq.dat'
datafile2 = 'intr1/intr_delay_freq.dat'

plot datafile0 smooth cumulative with lines linewidth 4 t "TAIC",\
    datafile1 smooth cumulative with lines linewidth 4 t "Poll", \
    datafile2 smooth cumulative with lines linewidth 4 t "Interrupt"

#####################################################
unset key
unset ylabel
set format y ""
set xrange [150:350]
datafile0 = 'atsintc4/atsintc_delay_freq.dat'
datafile1 = 'poll4/poll_delay_freq.dat'
datafile2 = 'intr4/intr_delay_freq.dat'

plot datafile0 smooth cumulative with lines linewidth 4 t "",\
    datafile1 smooth cumulative with lines linewidth 4 t "", \
    datafile2 smooth cumulative with lines linewidth 4 t ""

#####################################################
unset key
unset ylabel
set xrange [1500:1700]
datafile0 = 'atsintc16/atsintc_delay_freq.dat'
datafile1 = 'poll16/poll_delay_freq.dat'
datafile2 = 'intr16/intr_delay_freq.dat'

plot datafile0 smooth cumulative with lines linewidth 4 t "",\
    datafile1 smooth cumulative with lines linewidth 4 t "", \
    datafile2 smooth cumulative with lines linewidth 4 t ""

#####################################################
set xrange [100:300]
set ylabel "Latency Distribution"
unset format y
set yrange [0: 0.08]
set ytics 0.02
set mytics 2
set style fill transparent solid 0.8 border -1
set key samplen 1 Right center bottom at graph 0.2, 0.4
set xlabel 'Matrix Size = 1'
datafile0 = 'atsintc1/atsintc_delay_freq.dat'
datafile1 = 'poll1/poll_delay_freq.dat'
datafile2 = 'intr1/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "TAIC", \
    datafile1 using 1:2 smooth frequency w filledcurves t "Poll", \
    datafile2 using 1:2 smooth frequency w filledcurves t "Interrupt", \

#####################################################
set xrange [150:350]
unset ylabel
set format y ""
set xlabel 'Matrix Size = 4'
datafile0 = 'atsintc4/atsintc_delay_freq.dat'
datafile1 = 'poll4/poll_delay_freq.dat'
datafile2 = 'intr4/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "", \
    datafile1 using 1:2 smooth frequency w filledcurves t "", \
    datafile2 using 1:2 smooth frequency w filledcurves t "", \

#####################################################
set xrange [1500:1700]
set xlabel 'Matrix Size = 16'
datafile0 = 'atsintc16/atsintc_delay_freq.dat'
datafile1 = 'poll16/poll_delay_freq.dat'
datafile2 = 'intr16/intr_delay_freq.dat'

plot datafile0 using 1:2 smooth frequency w filledcurves t "", \
    datafile1 using 1:2 smooth frequency w filledcurves t "", \
    datafile2 using 1:2 smooth frequency w filledcurves t "", \

unset multiplot

