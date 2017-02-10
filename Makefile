build:
	cargo build

ssl: # To test https, create a local self-signed cert:
	@echo "Generating test ssl cert. Do not use in production."
	openssl req -x509 -newkey rsa:4096 -nodes -keyout target/localhost.key -out target/localhost.crt -days 3650
	openssl pkcs12 -export -out target/identity.p12 -inkey target/localhost.key -in target/localhost.crt
