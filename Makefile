


# very cool book on perf :
# https://nnethercote.github.io/perf-book/profiling.html

# https://gist.github.com/KodrAus/97c92c07a90b1fdd6853654357fd557a

perfs:
	perf record --call-graph dwarf go-game/target/debug/go-game > /dev/null

perfs2:
	perf record go-game/target/debug/go-game > /dev/null


report:
	perf report

# https://perf.wiki.kernel.org/index.php/Tutorial

#TODO : install flamegraph
# http://www.brendangregg.com/flamegraphs.html
# https://nanxiao.me/en/use-perf-and-flamegraph-to-profile-program-on-linux/