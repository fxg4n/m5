from typing import Dict, Any
from processors.base import BaseProcessor

class AssetProcessor(BaseProcessor):
    """Processor for asset and commodities data"""
    
    def __init__(self):
        super().__init__('asset')
    
    def process(self, raw_data: Dict[str, Any]) -> Dict[str, Any]:
        """Process raw asset data into standardized format"""
        processed = {
            'symbol': raw_data.get('symbol'),
            'meta': raw_data.get('meta', {}),
            'prices': []
        }
        
        if 'chart' in raw_data and 'result' in raw_data['chart']:
            for result in raw_data['chart']['result']:
                if 'timestamp' in result and 'indicators' in result:
                    for i, timestamp in enumerate(result['timestamp']):
                        quote = result['indicators']['quote'][0]
                        processed['prices'].append({
                            'date': timestamp,
                            'open': quote['open'][i],
                            'high': quote['high'][i],
                            'low': quote['low'][i],
                            'close': quote['close'][i],
                            'volume': quote['volume'][i]
                        })
        
        if not self.validate(processed):
            raise ValueError("Processed asset data validation failed")
            
        return processed
    
    def get_required_fields(self) -> list:
        return ['symbol', 'prices']