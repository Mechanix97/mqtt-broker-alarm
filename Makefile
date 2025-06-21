
run:
	@env $$(cat secrets/telegram.env | xargs) cargo run

build-image:
	@docker build -t mqtt_broker_alarm .

run-dockerfile: build-image
	@docker run --env-file secrets/telegram.env -d mqtt_broker_alarm
