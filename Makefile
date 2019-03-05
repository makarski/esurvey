.PHONY: sample all

all: sample

sample:
	./probation-csv -name=john -type=self-assessment < sample_input.csv
