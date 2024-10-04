# Переменные
CONFIG_PATH =./src/config/config.yaml
ACCESS_LOG_PATH = /home/jxs-logs/access.log
ERROR_LOG_PATH = /home/jxs-logs/error.log

# Цели
.PHONY: all configure build clean

all: configure

configure:
	@echo "Building..."
	@./auto/jxs.sh --config-path=$(CONFIG_PATH) --access-log-path=$(ACCESS_LOG_PATH) --error-log-path=$(ERROR_LOG_PATH)
	@echo "Enjoy!"

clean:
	@echo "Очистка проекта..."
	@cargo clean