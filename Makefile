# Переменные
CONFIG_PATH = ./src/config/config.yaml

# Цели
.PHONY: all configure build clean

all: configure

configure:
	@echo "Building..."
	@./auto/jxs.sh --config=$(CONFIG_PATH)
	@echo "Enjoy!"

clean:
	@echo "Очистка проекта..."
	@cargo clean