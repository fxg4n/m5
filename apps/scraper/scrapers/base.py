import requests
import json
from abc import ABC, abstractmethod
from typing import Dict, Any, Optional
from pathlib import Path
import time
import hashlib
import logging

from config.settings import settings
from utils.logger import get_logger

class BaseScraper(ABC):
    """Abstract base class for all scrapers"""
    
    def __init__(self, source_name: str):
        self.source_name = source_name
        self.logger = get_logger(f'scraper.{source_name.lower()}')
        self.session = requests.Session()
        self.session.headers.update({'User-Agent': 'MarketDataScraper/1.0'})
        
        settings.CACHE_DIR.mkdir(parents=True, exist_ok=True)
    
    def _get_cache_key(self, url: str, params: Dict[str, Any] = None) -> str:
        """Generate a cache key from URL and parameters"""
        key = f"{url}:{json.dumps(params, sort_keys=True) if params else ''}"
        return hashlib.md5(key.encode()).hexdigest()
    
    def _get_from_cache(self, key: str) -> Optional[Dict[str, Any]]:
        """Retrieve data from cache if available and fresh"""
        cache_file = settings.CACHE_DIR / f"{key}.json"
        
        if not cache_file.exists():
            return None
            
        if time.time() - cache_file.stat().st_mtime > settings.CACHE_TTL:
            return None
            
        try:
            with open(cache_file, 'r') as f:
                return json.load(f)
        except (json.JSONDecodeError, IOError) as e:
            self.logger.warning(f"Failed to read cache file {cache_file}: {e}")
            return None
    
    def _save_to_cache(self, key: str, data: Dict[str, Any]) -> None:
        """Save data to cache"""
        cache_file = settings.CACHE_DIR / f"{key}.json"
        try:
            with open(cache_file, 'w') as f:
                json.dump(data, f)
        except IOError as e:
            self.logger.warning(f"Failed to write cache file {cache_file}: {e}")
    
    def _make_request(self, url: str, params: Dict[str, Any] = None, method: str = 'GET', **kwargs) -> Dict[str, Any]:
        """Make HTTP request with caching and retry logic"""
        cache_key = self._get_cache_key(url, params)
        cached_data = self._get_from_cache(cache_key)
        
        if cached_data:
            self.logger.debug(f"Returning cached data for {url}")
            return cached_data
            
        for attempt in range(settings.MAX_RETRIES):
            try:
                response = self.session.request(
                    method,
                    url,
                    params=params,
                    timeout=settings.REQUEST_TIMEOUT,
                    **kwargs
                )
                response.raise_for_status()
                
                data = response.json()
                self._save_to_cache(cache_key, data)
                return data
                
            except requests.exceptions.RequestException as e:
                self.logger.warning(
                    f"Attempt {attempt + 1} failed for {url}: {str(e)}"
                )
                if attempt < settings.MAX_RETRIES - 1:
                    time.sleep(settings.RETRY_DELAY)
                else:
                    self.logger.error(f"All attempts failed for {url}")
                    raise
    
    @abstractmethod
    def scrape(self, *args, **kwargs) -> Dict[str, Any]:
        """Main method to implement in child classes"""
        pass