default:
	cargo build

clean:
	@cargo clean

coverage: migrate
	#cd browser && cargo tarpaulin -t 1200 --out Html
	cargo tarpaulin -t 1200 --out Html

dev-env:
	cargo fetch

doc:
	#yarn doc
	cargo doc --no-deps --workspace --all-features
	#cd browser && cargo doc --no-deps --workspace --all-features

docker:
	#docker build -t mcaptcha/mcaptcha:master -t mcaptcha/mcaptcha:latest .

docker-publish:
	#docker push mcaptcha/mcaptcha:master 
	#docker push mcaptcha/mcaptcha:latest

migrate:
	cargo run --bin tests-migrate

release:
	cargo build --release

run:
	cargo run

test:
	cargo test --all-features --no-fail-fast --all

xml-test-coverage: migrate
	cargo tarpaulin -t 1200 --out Xml

help:
	@echo  '  clean                   - drop builds and environments'
	@echo  '  coverage                - build test coverage in HTML format'
	@echo  '  dev-env                 - download dependencies'
	@echo  '  docker                  - build docker image'
	@echo  '  docker-publish          - build and publish docker image'
	@echo  '  doc                     - build documentation'
	@echo  '  migrate                 - run database migrations'
	@echo  '  run                     - run developer instance'
	@echo  '  test                    - run unit and integration tests'
	@echo  '  xml-coverage            - build test coverage in XML for upload to codecov'
	@echo  ''
