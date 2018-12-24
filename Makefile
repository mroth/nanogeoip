REALDATA_URI  = https://geolite.maxmind.com/download/geoip/database/GeoLite2-City.tar.gz
REALDATA_DIR  = data
REALDATA_PATH = $(REALDATA_DIR)/GeoLite2-City.mmdb
RELEASE = target/release/nanogeoip

.PHONY: test bench benchreal realdata clobber

default: $(RELEASE)
$(RELEASE): src/*.rs
	cargo build --release

test:
	cargo test

bench:
	cargo bench

# run microbenchmarks, with a full copy of the GeoLite2 city database.
# this gets us more accurate measurements as the database size is larger.
# NOTE: since we don't bundle a production database, the dependencies for this
# task will attempt to download one from the public internet if it isn't already
# stored locally. (~26.5MB download)
# benchreal: realdata
# 	# TODO: consider porting go benchmarks for this?

# downloads a copy of the full production database (free version)
realdata: $(REALDATA_PATH)
$(REALDATA_PATH): 
	mkdir -p $(REALDATA_DIR)
	curl $(REALDATA_URI) | tar -xzv --strip-components=1 -C $(REALDATA_DIR)

image:
	docker build -t mrothy/nanogeoip .

clobber:
	cargo clean
	rm -rf $(REALDATA_DIR)
