set terminal pdfcairo enhanced size 14in,6in font "Arial,24"
set output 'prio_impact.pdf'

fn(v) = sprintf("%.0f", v)
sub(a, b) = a-b

set key horizontal center top Left reverse at graph 0.12, -0.1
# unset key
set noborder
# set title 'Latency({/symbol \155}s)'
set xtics nomirror
set ytics nomirror
set grid ytics
set grid linetype 0
set style fs solid 1 border -1
set xrange [0.5 : 1.5]
unset xtics

set multiplot layout 1, 3

# set xlabel "c-4"
# set ytics 10
# datafile = 'connect4.dat'
# set size 0.142, 0.9
# set origin 0.01, 0
# plot datafile using 2 with histogram t '', '' \
#               using 3 with histogram t '', '' \
#               using 4 with histogram t '', '' \
#               using 5 with histogram t '', '' \
#               using 6 with histogram t '', \
# ##################################################
# set xlabel "c-8"
# set ytics 10
# datafile = 'connect8.dat'
# set size 0.14, 0.9
# set origin 0.15, 0
# plot datafile using 2 with histogram t 'base', '' \
#               using 3 with histogram t '', '' \
#               using 4 with histogram t '', '' \
#               using 5 with histogram t '', '' \
#               using 6 with histogram t '', \
# ##################################################
# set xlabel "c-16"
# set ytics 40
# datafile = 'connect16.dat'
# set size 0.14, 0.9
# set origin 0.29, 0
# plot datafile using 2 with histogram t '', '' \
#               using 3 with histogram t 'Priority-0', '' \
#               using 4 with histogram t '', '' \
#               using 5 with histogram t '', '' \
#               using 6 with histogram t '', \

##################################################
# set xlabel "c-32"
# set ytics 1000
# datafile = 'connect32.dat'
# set size 0.14, 0.9
# set origin 0.43, 0
# plot datafile using 2 with histogram t '', '' \
#               using 3 with histogram t '', '' \
#               using 4 with histogram t 'Priority-1', '' \
#               using 5 with histogram t '', '' \
#               using 6 with histogram t '', \

##################################################
set xlabel "Connection-64"
set ytics 1000
set ylabel 'Latency({/symbol \155}s)'
datafile = 'connect64.dat'
set size 0.33, 0.9
set origin 0.005, 0
plot datafile using 2 with histogram t '', '' \
              using 3 with histogram t '', '' \
              using 4 with histogram t '', '' \
              using 5 with histogram t '', '' \
              using 6 with histogram t '', \

##################################################
set xlabel "Connection-128"
set ytics 1000
unset ylabel
datafile = 'connect128.dat'
set size 0.33, 0.9
set origin 0.335, 0
plot datafile using 2 with histogram t '', '' \
              using 3 with histogram t '', '' \
              using 4 with histogram t '', '' \
              using 5 with histogram t '', '' \
              using 6 with histogram t '', \

##################################################
set xlabel "Connection-256"
set ytics 1000
unset ylabel

datafile = 'connect256.dat'
set size 0.33, 0.9
set origin 0.665, 0
plot datafile using 2 with histogram t '', '' \
              using 3 with histogram t '', '' \
              using 4 with histogram t '', '' \
              using 5 with histogram t '', '' \
              using 6 with histogram t '', \

set xlabel "Connection-128"
set ytics 1000
unset ylabel
datafile = 'connect128.dat'
set size 1.0, 1.0
set origin 0.335, 1.0
plot datafile using 2 with histogram t 'Base', '' \
              using 3 with histogram t 'P0', '' \
              using 4 with histogram t 'P1', '' \
              using 5 with histogram t 'P2', '' \
              using 6 with histogram t 'P3', \