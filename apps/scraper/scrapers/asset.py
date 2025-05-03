from typing import Dict, Any, Optional, List, Union
from datetime import datetime
from config.data_sources import DataSource
from scrapers.base import BaseScraper

class AssetScraper(BaseScraper):
    """Scraper for asset and commodities data"""
    
    def __init__(self, source: DataSource):
        super().__init__(source.name)
        self.source_config = source.value
        self.base_url = self.source_config['base_url']
    
    def scrape(
        self,
        symbol: Union[str, List[str]],
        start_date: Optional[datetime] = None,
        end_date: Optional[datetime] = None,
        interval: str = '1d',
        **kwargs
    ) -> Dict[str, Any]:
        """
        Scrape asset/commodity data for one or more symbols
        
        Args:
            symbol: Ticker symbol or list of symbols
            start_date: Optional start date for historical data
            end_date: Optional end date for historical data
            interval: Data interval (e.g., '1d', '1h', '1m')
            **kwargs: Additional parameters specific to the data source
            
        Returns:
            Dictionary containing the scraped data
        """
        if isinstance(symbol, list):
            symbol = ','.join(symbol)
            
        endpoint = self.source_config['endpoints']['historical'].format(symbol=symbol)
        url = f"{self.base_url}{endpoint}"
        
        params = {
            'interval': interval,
            **kwargs
        }
        
        if start_date:
            params['period1'] = int(start_date.timestamp())
        if end_date:
            params['period2'] = int(end_date.timestamp())
            
        if self.source_config.get('api_key_required', False):
            params['api_key'] = settings.API_KEY
            
        return self._make_request(url, params=params)