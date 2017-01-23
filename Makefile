build:
	cargo build

ssl: # To test https, create a local self-signed cert:
	cd target
	openssl req -x509 -newkey rsa:4096 -nodes -keyout localhost.key -out localhost.crt -days 3650
	openssl pkcs12 -export -out identity.p12 -inkey localhost.key -in localhost.crt --password mypass
