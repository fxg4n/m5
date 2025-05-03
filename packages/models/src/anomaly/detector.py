import numpy as np
import pandas as pd
from typing import List, Dict, Any
from sklearn.ensemble import IsolationForest
from sklearn.svm import OneClassSVM
from sklearn.preprocessing import StandardScaler
import joblib
import os

class AnomalyDetector:
    def __init__(self, model_type: str = 'isolation_forest'):
        """
        Initialize anomaly detector
        
        Args:
            model_type: Either 'isolation_forest' (default) or 'svm'
        """
        self.model_type = model_type
        self.models_dir = os.path.join(os.path.dirname(__file__), '../../models')
        self.scaler = StandardScaler()
        
        if model_type == 'isolation_forest':
            self.model = IsolationForest(contamination=0.01, random_state=42)
        elif model_type == 'svm':
            self.model = OneClassSVM(nu=0.01)
        else:
            raise ValueError(f"Unsupported model type: {model_type}")
    
    def train(self, data: List[float]) -> None:
        """
        Train the anomaly detection model
        
        Args:
            data: List of historical values
        """
        # Convert to 2D array and scale
        X = np.array(data).reshape(-1, 1)
        X_scaled = self.scaler.fit_transform(X)
        self.model.fit(X_scaled)
    
    def detect(self, data: List[float]) -> List[Dict[str, Any]]:
        """
        Detect anomalies in data
        
        Args:
            data: List of values to analyze
            
        Returns:
            List of dictionaries with detection results
        """
        X = np.array(data).reshape(-1, 1)
        X_scaled = self.scaler.transform(X)
        
        if self.model_type == 'isolation_forest':
            anomalies = self.model.predict(X_scaled)
            scores = self.model.decision_function(X_scaled)
            return [
                {
                    'value': float(X[i][0]),
                    'is_anomaly': bool(anomalies[i] == -1),
                    'anomaly_score': float(scores[i])
                }
                for i in range(len(data))
            ]
        elif self.model_type == 'svm':
            anomalies = self.model.predict(X_scaled)
            return [
                {
                    'value': float(X[i][0]),
                    'is_anomaly': bool(anomalies[i] == -1)
                }
                for i in range(len(data))
            ]
    
    def save_model(self, name: str) -> None:
        """Save trained model to disk"""
        os.makedirs(self.models_dir, exist_ok=True)
        joblib.dump(self.model, os.path.join(self.models_dir, f'{name}_{self.model_type}.joblib'))
        joblib.dump(self.scaler, os.path.join(self.models_dir, f'{name}_scaler.joblib'))
    
    def load_model(self, name: str) -> None:
        """Load trained model from disk"""
        self.model = joblib.load(os.path.join(self.models_dir, f'{name}_{self.model_type}.joblib'))
        self.scaler = joblib.load(os.path.join(self.models_dir, f'{name}_scaler.joblib'))