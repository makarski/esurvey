.PHONY: sample all

all: sample

sample:
	./probation-csv self-assessment < sample_input.csv
