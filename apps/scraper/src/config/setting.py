import os
from pathlib import Path
from dotenv import load_dotenv

BASE_DIR = Path(__file__).resolve().parent.parent
load_dotenv(BASE_DIR / '.env')

class Settings:
    # API Configuration
    API_BASE_URL = os.getenv('API_BASE_URL', 'https://your-api.example.com')
    API_KEY = os.getenv('API_KEY', '')
    API_TIMEOUT = int(os.getenv('API_TIMEOUT', 30))
    
    # Scraper Configuration
    REQUEST_TIMEOUT = int(os.getenv('REQUEST_TIMEOUT', 60))
    MAX_RETRIES = int(os.getenv('MAX_RETRIES', 3))
    RETRY_DELAY = int(os.getenv('RETRY_DELAY', 5))
    
    # Data Storage
    CACHE_DIR = BASE_DIR / 'cache'
    CACHE_TTL = int(os.getenv('CACHE_TTL', 3600))  # 1 hour
    
    # Logging
    LOG_LEVEL = os.getenv('LOG_LEVEL', 'INFO')
    LOG_FILE = BASE_DIR / 'logs' / 'scraper.log'

settings = Settings()