
run:
	@env $$(cat secrets/telegram.env | xargs) cargo run

build-image:
	@docker build -t mqtt-broker-alarm .

run-docker: build-image stop-docker
	@docker run --env-file secrets/telegram.env -d mqtt-broker-alarm --name mqtt-broker-alarm

stop-docker:
	@docker stop mqtt-broker-alarm
