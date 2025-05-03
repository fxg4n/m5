import requests
from typing import Dict, Any
import logging

from config.settings import settings
from utils.logger import get_logger

class APIClient:
    """Client for sending data to the internal API"""
    
    def __init__(self):
        self.logger = get_logger('api.client')
        self.base_url = settings.API_BASE_URL
        self.session = requests.Session()
        self.session.headers.update({
            'Authorization': f'Bearer {settings.API_KEY}',
            'Content-Type': 'application/json'
        })
    
    def send_data(self, endpoint: str, data: Dict[str, Any]) -> bool:
        """
        Send processed data to the internal API
        
        Args:
            endpoint: API endpoint (e.g., 'macro', 'assets', 'sentiment')
            data: Processed data to send
            
        Returns:
            True if successful, False otherwise
        """
        url = f"{self.base_url}/{endpoint}"
        
        try:
            response = self.session.post(
                url,
                json=data,
                timeout=settings.API_TIMEOUT
            )
            response.raise_for_status()
            self.logger.info(f"Successfully sent data to {endpoint} endpoint")
            return True
        except requests.exceptions.RequestException as e:
            self.logger.error(f"Failed to send data to {endpoint}: {str(e)}")
            return False