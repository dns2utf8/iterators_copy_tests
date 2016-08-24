
CXXFLAGS := -std=c++14 -O2 -Wall

default:
	make -j8 all
	make run

all:
	clang++ $(CXXFLAGS) -o clang_test main.cpp
	g++ $(CXXFLAGS) -o gcc_test main.cpp

run: all
	#########################################################################
	./clang_test
	#########################################################################
	./gcc_test
	#########################################################################
