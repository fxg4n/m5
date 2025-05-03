from abc import ABC, abstractmethod
from typing import Dict, Any
import logging

from utils.logger import get_logger

class BaseProcessor(ABC):
    """Abstract base class for all data processors"""
    
    def __init__(self, processor_name: str):
        self.processor_name = processor_name
        self.logger = get_logger(f'processor.{processor_name.lower()}')
    
    @abstractmethod
    def process(self, raw_data: Dict[str, Any]) -> Dict[str, Any]:
        """Process raw data into standardized format"""
        pass
    
    def validate(self, processed_data: Dict[str, Any]) -> bool:
        """Validate processed data structure"""
        required_fields = self.get_required_fields()
        missing_fields = [field for field in required_fields if field not in processed_data]
        
        if missing_fields:
            self.logger.error(f"Missing required fields: {missing_fields}")
            return False
        return True
    
    @abstractmethod
    def get_required_fields(self) -> list:
        """List of required fields in processed data"""
        pass