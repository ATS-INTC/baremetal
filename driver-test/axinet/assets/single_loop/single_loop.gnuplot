# set terminal pngcairo  transparent enhanced font "arial,10" fontscale 1.0 size 600, 400 
# set output 'histograms.1.png'
set key fixed right center vertical Right noreverse noenhanced autotitle nobox
set style data linespoints
set datafile missing '-'
set xtics border in scale 1,0.5 nomirror rotate by -45  autojustify
set xtics  norangelimit 
set xtics   ()
set xlabel 'Frame Size(Bytes)'
set ylabel 'Latency({/symbol \155}s)'
set title "Throughput" 
set xrange [ * : * ] noreverse writeback
set yrange [ * : * ] noreverse writeback
set colorbox vertical origin screen 0.9, 0.2 size screen 0.05, 0.6 front  noinvert bdefault
NO_ANIMATION = 1
## Last datafile plotted: "throughput.dat"
plot 'single_loop.dat' using 2:xtic(1) title columnheader(2), for [i=3:4] '' using i title columnheader(i)
