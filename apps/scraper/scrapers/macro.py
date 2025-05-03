from typing import Dict, Any, Optional
from config.data_sources import DataSource
from scrapers.base import BaseScraper

class MacroScraper(BaseScraper):
    """Scraper for macroeconomic data"""
    
    def __init__(self, source: DataSource):
        super().__init__(source.name)
        self.source_config = source.value
        self.base_url = self.source_config['base_url']
    
    def scrape(self, indicator: str, country: Optional[str] = None, **kwargs) -> Dict[str, Any]:
        """
        Scrape macroeconomic data for a specific indicator and optionally country
        
        Args:
            indicator: Economic indicator to scrape (e.g., 'gdp', 'inflation')
            country: Optional country code to filter by
            **kwargs: Additional parameters specific to the data source
            
        Returns:
            Dictionary containing the scraped data
        """
        if 'endpoints' in self.source_config and indicator in self.source_config['endpoints']:
            endpoint = self.source_config['endpoints'][indicator]
        else:
            endpoint = indicator
            
        url = f"{self.base_url}{endpoint}"
        
        params = kwargs.copy()
        if country:
            params['country'] = country
            
        if self.source_config.get('api_key_required', False):
            params['api_key'] = settings.API_KEY
            
        return self._make_request(url, params=params)