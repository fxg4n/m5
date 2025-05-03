from typing import Dict, Any, List
from processors.base import BaseProcessor
from datetime import datetime

class SentimentProcessor(BaseProcessor):
    """Processor for market sentiment data"""
    
    def __init__(self):
        super().__init__('sentiment')
    
    def process(self, raw_data: Dict[str, Any]) -> Dict[str, Any]:
        """Process raw sentiment data into standardized format"""
        processed = {
            'source': raw_data.get('source', 'unknown'),
            'query': raw_data.get('query', ''),
            'items': []
        }
        
        if 'data' in raw_data:
            for item in raw_data['data']:
                processed_item = {
                    'id': item.get('id'),
                    'text': item.get('text', ''),
                    'created_at': item.get('created_at', ''),
                    'sentiment': self._analyze_sentiment(item.get('text', ''))
                }
                
                # Include metrics if available (likes, retweets, etc.)
                if 'public_metrics' in item:
                    processed_item['metrics'] = item['public_metrics']
                
                processed['items'].append(processed_item)
        
        if not self.validate(processed):
            raise ValueError("Processed sentiment data validation failed")
            
        return processed
    
    def _analyze_sentiment(self, text: str) -> float:
        """Basic sentiment analysis (implement more sophisticated analysis as needed)"""
        positive_words = ['bullish', 'growth', 'profit', 'gain', 'positive']
        negative_words = ['bearish', 'loss', 'drop', 'decline', 'negative']
        
        text_lower = text.lower()
        positive_score = sum(1 for word in positive_words if word in text_lower)
        negative_score = sum(1 for word in negative_words if word in text_lower)
        
        if positive_score + negative_score == 0:
            return 0.0
        return (positive_score - negative_score) / (positive_score + negative_score)
    
    def get_required_fields(self) -> list:
        return ['source', 'items']