set terminal pdfcairo enhanced size 8in,6in font "Arial,24"
set output 'single_workload.pdf'
fn(v) = sprintf("%.0f", v)
compare(a, b) = a / b
fn2(v) = sprintf("%.2f", v)

# 开始多图模式
set multiplot

# 第一个大子图
set size 1, 1
set origin 0, 0

set key horizontal center top Left reverse at graph 0.5, 1.125
set noborder
set xtics nomirror
set ytics nomirror
set grid ytics
set grid linetype 0
set style fill solid 1 border -1

set xrange [0.5 : 5.5]
set xtics ("1" 1, "2" 2, "4" 3, "8" 4, "16" 5)
set xlabel "Matrix-size"

set logscale y 2
set ylabel 'Latency({/symbol \155}s)'
set size 1.0, 0.9
datafile = 'single_workload.dat'
plot datafile using 2 with histogram title 'TAIC', '' \
    using 3 with histogram title 'Poll', '' \
    using 4 with histogram title 'Interrupt', \
    datafile using ($0):($3):(fn2(compare($3, $2))) title '' with labels textcolor rgb "#404040" rot by 90 offset char 0, 1.0, \
    datafile using ($0):($4):(fn2(compare($4, $2))) title '' with labels textcolor rgb "#404040" rot by 90 offset char 2, 1.0

# 第二个小子图，嵌套在大图内
set size 0.5, 0.4
set origin 0.2, 0.5
set object 1 rect from graph 0, 0 to graph 1, 1 behind
set object 1 fc rgb "white" fillstyle solid 1.0 noborder

set border
set xtics mirror
set ytics mirror
unset xlabel
unset ylabel
set ylabel 'Latency Diff({/symbol \155}s)'

diffdata = 'diff.dat'
plot diffdata using 3 lc 2 with histogram title '', '' \
    using 4 lc 3 with histogram title '', \
    # diffdata using ($0):($3):(fn2($3)) title '' with labels textcolor rgb "#404040" rot by 90 offset char 0, 1.0, \
    # diffdata using ($0):($4):(fn2($4)) title '' with labels textcolor rgb "#404040" rot by 90 offset char 0, 1.0, 

# 结束多图模式
unset multiplot

# 清理设置，以便后续绘图不受影响
set terminal
set output