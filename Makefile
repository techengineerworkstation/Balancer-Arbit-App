.PHONY: build build-release web-server cli docker docker-push tauri tauri-build flutter-android flutter-ios clean

# Development
dev:
	cargo run --release

# Build server binary
build:
	cargo build --release

# Build web app
web:
	cd web-app && trunk build --release

# Run CLI
cli:
	cargo run --release --bin balancer-arb -- start

# Scan only
scan:
	cargo run --release --bin balancer-arb -- scan

# Simulate trade
simulate:
	cargo run --release --bin balancer-arb -- simulate

# Docker
docker:
	docker-compose build

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs -f

# Deploy smart contracts
deploy:
	cd contracts && forge script script/Deploy.s.sol --rpc-url https://polygon-rpc.com --broadcast

# Verify contract
verify:
	cd contracts && forge verify-contract $(ADDRESS) BalancerFlashLoanArbitrage --chain-id 137

# Run tests
test:
	cd contracts && forge test -vvv

# Tauri Desktop
tauri-dev:
	cd web-app && trunk build && cargo tauri dev

tauri-build:
	cd web-app && trunk build --release && cargo tauri build

# Flutter Mobile
flutter-deps:
	cd mobile-app && flutter pub get

flutter-android:
	cd mobile-app && flutter build apk --release

flutter-android-bundle:
	cd mobile-app && flutter build appbundle --release

flutter-ios:
	cd mobile-app && flutter build ios --release

flutter-run:
	cd mobile-app && flutter run

# Clean
clean:
	cargo clean
	cd contracts && forge clean
	cd web-app && rm -rf dist pkg
	cd mobile-app && flutter clean
	rm -rf target/

# Install tools
install-foundry:
	curl -L https://foundry.paradigm.xyz | bash && foundryup

install-flutter:
	git clone https://github.com/flutter/flutter.git -b stable
	export PATH="flutter/bin:$$PATH"
	flutter doctor

install-tauri:
	cargo install tauri-cli

# Status
status:
	@echo "=== Balancer Arbitrage Bot ==="
	@echo "Server: http://localhost:8080"
	@echo "Network: Polygon (137)"
	@echo "Pair: BAL/USDC"
	@echo ""
	@echo "Build targets:"
	@echo "  make build      - Build server"
	@echo "  make web        - Build web app"
	@echo "  make docker     - Build Docker image"
	@echo "  make tauri-build - Build desktop app"
	@echo "  make flutter-android - Build Android APK"
	@echo "  make flutter-ios     - Build iOS app"