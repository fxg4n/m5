from typing import Dict, Any
import argparse
from datetime import datetime, timedelta

from config.data_sources import DataSource
from scrapers import MacroScraper, AssetScraper, SentimentScraper
from processors import MacroProcessor, AssetProcessor, SentimentProcessor
from api.client import APIClient
from utils.logger import get_logger

logger = get_logger('main')

class MarketDataScraper:
    """Main application class orchestrating scraping, processing, and data sending"""
    
    def __init__(self):
        self.api_client = APIClient()
        self.scrapers = {
            'macro': {
                'imf': MacroScraper(DataSource.IMF),
                'fred': MacroScraper(DataSource.FRED)
            },
            'asset': {
                'yahoo': AssetScraper(DataSource.YAHOO_FINANCE)
            },
            'sentiment': {
                'twitter': SentimentScraper(DataSource.TWITTER),
                'newsapi': SentimentScraper(DataSource.NEWSAPI)
            }
        }
        self.processors = {
            'macro': MacroProcessor(),
            'asset': AssetProcessor(),
            'sentiment': SentimentProcessor()
        }
    
    def scrape_and_process(self, data_type: str, source: str, **kwargs) -> Dict[str, Any]:
        """
        Scrape and process data from a specific source
        
        Args:
            data_type: Type of data ('macro', 'asset', 'sentiment')
            source: Data source (e.g., 'imf', 'yahoo', 'twitter')
            **kwargs: Additional arguments specific to the scraper
            
        Returns:
            Processed data dictionary
        """
        if data_type not in self.scrapers or source not in self.scrapers[data_type]:
            raise ValueError(f"Unsupported data type or source: {data_type}/{source}")
            
        scraper = self.scrapers[data_type][source]
        processor = self.processors[data_type]
        
        # Scrape raw data
        raw_data = scraper.scrape(**kwargs)
        raw_data['source'] = source
        
        # Process data
        processed_data = processor.process(raw_data)
        
        return processed_data
    
    def run(self, data_type: str, source: str, send_to_api: bool = True, **kwargs) -> bool:
        """
        Run the full pipeline: scrape, process, and optionally send to API
        
        Args:
            data_type: Type of data ('macro', 'asset', 'sentiment')
            source: Data source (e.g., 'imf', 'yahoo', 'twitter')
            send_to_api: Whether to send processed data to the API
            **kwargs: Additional arguments specific to the scraper
            
        Returns:
            True if successful, False otherwise
        """
        try:
            processed_data = self.scrape_and_process(data_type, source, **kwargs)
            
            if send_to_api:
                return self.api_client.send_data(data_type, processed_data)
            return True
        except Exception as e:
            logger.error(f"Error processing {data_type} data from {source}: {str(e)}")
            return False

def parse_args():
    """Parse command line arguments"""
    parser = argparse.ArgumentParser(description='Market Data Scraper')
    parser.add_argument('--data-type', choices=['macro', 'asset', 'sentiment'], required=True,
                       help='Type of data to scrape')
    parser.add_argument('--source', required=True,
                       help='Data source to scrape from')
    parser.add_argument('--no-api', action='store_true',
                       help='Disable sending data to API')
    
    # Macro-specific args
    macro_group = parser.add_argument_group('macro', 'Macroeconomic data options')
    macro_group.add_argument('--indicator',
                            help='Economic indicator (e.g., gdp, inflation)')
    macro_group.add_argument('--country',
                            help='Country code for macroeconomic data')
    
    # Asset-specific args
    asset_group = parser.add_argument_group('asset', 'Asset/commodity data options')
    asset_group.add_argument('--symbol',
                            help='Asset symbol or comma-separated list of symbols')
    asset_group.add_argument('--days', type=int, default=30,
                            help='Number of days of historical data to fetch')
    
    # Sentiment-specific args
    sentiment_group = parser.add_argument_group('sentiment', 'Sentiment data options')
    sentiment_group.add_argument('--query',
                                help='Search query for sentiment data')
    sentiment_group.add_argument('--limit', type=int, default=100,
                                help='Maximum number of results to return')
    
    return parser.parse_args()

def main():
    args = parse_args()
    scraper = MarketDataScraper()
    
    # Prepare kwargs based on data type
    kwargs = {}
    if args.data_type == 'macro':
        kwargs.update({
            'indicator': args.indicator,
            'country': args.country
        })
    elif args.data_type == 'asset':
        end_date = datetime.now()
        start_date = end_date - timedelta(days=args.days)
        kwargs.update({
            'symbol': args.symbol,
            'start_date': start_date,
            'end_date': end_date
        })
    elif args.data_type == 'sentiment':
        kwargs.update({
            'query': args.query,
            'limit': args.limit
        })
    
    success = scraper.run(
        data_type=args.data_type,
        source=args.source,
        send_to_api=not args.no_api,
        **kwargs
    )
    
    if not success:
        logger.error("Scraping job failed")
        exit(1)
    
    logger.info("Scraping job completed successfully")

if __name__ == '__main__':
    main()