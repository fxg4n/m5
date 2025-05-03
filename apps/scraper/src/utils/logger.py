import logging
import sys
from pathlib import Path
from typing import Optional

from config.settings import settings

def get_logger(name: str, log_file: Optional[Path] = None) -> logging.Logger:
    """Configure and return a logger with the given name"""
    logger = logging.getLogger(name)
    logger.setLevel(settings.LOG_LEVEL)
    
    logger.handlers.clear()
    
    formatter = logging.Formatter(
        '%(asctime)s - %(name)s - %(levelname)s - %(message)s'
    )
    
    console_handler = logging.StreamHandler(sys.stdout)
    console_handler.setFormatter(formatter)
    logger.addHandler(console_handler)
    
    if log_file or settings.LOG_FILE:
        log_file = log_file or settings.LOG_FILE
        log_file.parent.mkdir(parents=True, exist_ok=True)
        file_handler = logging.FileHandler(log_file)
        file_handler.setFormatter(formatter)
        logger.addHandler(file_handler)
    
    return logger