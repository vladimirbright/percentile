ifdef RELEASE
BIN=./target/release/percentile
else
BIN=./target/debug/percentile
endif

LOGS_DIR=./logs/

$(BIN): clean
ifdef RELEASE
	cargo build --release
else
	cargo build
endif

p: $(BIN)
	time $(BIN) --separator=' ' --column=8 $(LOGS_DIR)access.log

t: $(BIN)
	time $(BIN) --separator='\t' --column=5 $(LOGS_DIR)nginx_access.log

b: $(BIN)
	time $(BIN) --separator='\t' --column=5 $(LOGS_DIR)nginx_access.log.b

clean:
	-rm $(BIN)
