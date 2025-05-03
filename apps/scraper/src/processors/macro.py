from typing import Dict, Any
from processors.base import BaseProcessor

class MacroProcessor(BaseProcessor):
    """Processor for macroeconomic data"""
    
    def __init__(self):
        super().__init__('macro')
    
    def process(self, raw_data: Dict[str, Any]) -> Dict[str, Any]:
        """Process raw macroeconomic data into standardized format"""
        processed = {
            'source': raw_data.get('source', 'unknown'),
            'indicator': raw_data.get('indicator'),
            'values': []
        }
        
        if 'data' in raw_data:
            for country, values in raw_data['data'].items():
                processed['values'].append({
                    'country': country,
                    'value': values.get('value'),
                    'unit': values.get('unit', ''),
                    'date': values.get('date')
                })
        
        if not self.validate(processed):
            raise ValueError("Processed macroeconomic data validation failed")
            
        return processed
    
    def get_required_fields(self) -> list:
        return ['indicator', 'values']