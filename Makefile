
run:
	@env $$(cat secrets/telegram.env | xargs) cargo run
