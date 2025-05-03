from typing import Dict, Any, List
from datetime import datetime

def validate_date_format(date_str: str, fmt: str = '%Y-%m-%d') -> bool:
    """Validate that a date string matches the specified format"""
    try:
        datetime.strptime(date_str, fmt)
        return True
    except ValueError:
        return False

def validate_required_fields(data: Dict[str, Any], required_fields: List[str]) -> bool:
    """Validate that all required fields are present in the data"""
    return all(field in data for field in required_fields)

def validate_numeric(value: Any) -> bool:
    """Validate that a value is numeric (int or float)"""
    return isinstance(value, (int, float))