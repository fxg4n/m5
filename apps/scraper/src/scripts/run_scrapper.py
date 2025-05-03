import argparse
import json
import sys
from main import MarketDataScraper

def parse_args():
    """Parse command line arguments for the script"""
    parser = argparse.ArgumentParser(description='Run market data scraper')
    
    # Required arguments
    parser.add_argument('--data-type', required=True, 
                       choices=['macro', 'asset', 'sentiment'],
                       help='Type of data to scrape')
    parser.add_argument('--source', required=True,
                       help='Data source to scrape from')
    
    # Macro-specific args
    parser.add_argument('--indicator',
                       help='Economic indicator (e.g., gdp, inflation)')
    parser.add_argument('--country',
                       help='Country code for macroeconomic data')
    
    # Asset-specific args
    parser.add_argument('--symbol',
                       help='Asset symbol or comma-separated list of symbols')
    parser.add_argument('--days', type=int, default=30,
                       help='Number of days of historical data to fetch')
    
    # Sentiment-specific args
    parser.add_argument('--query',
                       help='Search query for sentiment data')
    parser.add_argument('--limit', type=int, default=100,
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
        kwargs.update({
            'symbol': args.symbol,
            'days': args.days
        })
    elif args.data_type == 'sentiment':
        kwargs.update({
            'query': args.query,
            'limit': args.limit
        })
    
    try:
        processed_data = scraper.scrape_and_process(
            args.data_type,
            args.source,
            **kwargs
        )
        print(json.dumps(processed_data))
    except Exception as e:
        print(json.dumps({
            'error': str(e),
            'data_type': args.data_type,
            'source': args.source,
            'success': False
        }), file=sys.stderr)
        sys.exit(1)

if __name__ == '__main__':
    main()