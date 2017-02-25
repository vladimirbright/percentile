BIN=./target/debug/percentile
LOGS_DIR=./logs/

$(BIN): clean
	cargo build

p: $(BIN)
	time $(BIN) --separator=' ' --column=8 $(LOGS_DIR)access.log

t: $(BIN)
	time $(BIN) --separator='\t' --column=5 $(LOGS_DIR)nginx_access.log

b: $(BIN)
	time $(BIN) --separator='\t' --column=5 $(LOGS_DIR)nginx_access.log.b

clean:
	-rm $(BIN)
