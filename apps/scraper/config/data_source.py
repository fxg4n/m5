from enum import Enum

class DataSource(Enum):
    """Enumeration of data sources with their configurations"""
    
    IMF = {
        'base_url': 'https://www.imf.org/external/datamapper/api/v1/',
        'endpoints': {
            'gdp': 'NGDPD',
            'inflation': 'PCPIPCH',
            'unemployment': 'LUR'
        }
    }
    
    FRED = {
        'base_url': 'https://api.stlouisfed.org/fred/',
        'api_key_required': True
    }
    
    YAHOO_FINANCE = {
        'base_url': 'https://query1.finance.yahoo.com/v8/finance/chart/',
        'endpoints': {
            'historical': '{symbol}'
        }
    }
    
    TWITTER = {
        'base_url': 'https://api.twitter.com/2/',
        'api_key_required': True
    }
    
    NEWSAPI = {
        'base_url': 'https://newsapi.org/v2/',
        'api_key_required': True
    }