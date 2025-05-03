import numpy as np
import pandas as pd
from typing import List, Dict, Any, Optional
from sklearn.preprocessing import MinMaxScaler
from tensorflow.keras.models import Sequential, load_model
from tensorflow.keras.layers import LSTM, Dense
from statsmodels.tsa.arima.model import ARIMA
import os
import joblib

class TimeSeriesForecaster:
    def __init__(self, model_type: str = 'lstm'):
        """
        Initialize time series forecaster
        
        Args:
            model_type: Either 'lstm' (default), 'arima', or 'prophet'
        """
        self.model_type = model_type
        self.models_dir = os.path.join(os.path.dirname(__file__), '../../models')
        self.scaler = MinMaxScaler(feature_range=(0, 1))
        
        if model_type == 'lstm':
            self.model = self._build_lstm_model()
        elif model_type == 'arima':
            self.model = None  # ARIMA models are fit per-series
        else:
            raise ValueError(f"Unsupported model type: {model_type}")
    
    def _build_lstm_model(self) -> Sequential:
        """Build LSTM model architecture"""
        model = Sequential()
        model.add(LSTM(50, return_sequences=True, input_shape=(60, 1)))
        model.add(LSTM(50, return_sequences=False))
        model.add(Dense(25))
        model.add(Dense(1))
        model.compile(optimizer='adam', loss='mean_squared_error')
        return model
    
    def preprocess_data(self, data: List[float], window_size: int = 60) -> tuple:
        """Prepare data for LSTM training"""
        scaled_data = self.scaler.fit_transform(np.array(data).reshape(-1, 1))
        
        x, y = [], []
        for i in range(window_size, len(scaled_data)):
            x.append(scaled_data[i-window_size:i, 0])
            y.append(scaled_data[i, 0])
        
        return np.array(x), np.array(y)
    
    def train(self, data: List[float], epochs: int = 10, batch_size: int = 32) -> None:
        """
        Train the forecasting model
        
        Args:
            data: List of historical values
            epochs: Number of training epochs (for LSTM)
            batch_size: Batch size (for LSTM)
        """
        if self.model_type == 'lstm':
            x, y = self.preprocess_data(data)
            x = np.reshape(x, (x.shape[0], x.shape[1], 1))
            self.model.fit(x, y, batch_size=batch_size, epochs=epochs)
        elif self.model_type == 'arima':
            self.model = ARIMA(data, order=(5,1,0)).fit()
    
    def predict(self, data: List[float], steps: int = 5) -> List[float]:
        """
        Make predictions
        
        Args:
            data: Historical data to base predictions on
            steps: Number of future steps to predict
            
        Returns:
            List of predicted values
        """
        if self.model_type == 'lstm':
            # Prepare last window of data
            scaled_data = self.scaler.transform(np.array(data).reshape(-1, 1))
            x_input = scaled_data[-60:].reshape(1, -1)
            x_input = np.reshape(x_input, (x_input.shape[0], x_input.shape[1], 1))
            
            # Make predictions
            predictions = []
            for _ in range(steps):
                pred = self.model.predict(x_input)[0]
                predictions.append(pred[0])
                x_input = np.append(x_input[:, 1:, :], [[pred]], axis=1)
            
            return self.scaler.inverse_transform(np.array(predictions).reshape(-1, 1)).flatten().tolist()
        elif self.model_type == 'arima':
            model = ARIMA(data, order=(5,1,0)).fit()
            return model.forecast(steps=steps).tolist()
    
    def save_model(self, name: str) -> None:
        """Save trained model to disk"""
        os.makedirs(self.models_dir, exist_ok=True)
        if self.model_type == 'lstm':
            self.model.save(os.path.join(self.models_dir, f'{name}_lstm.h5'))
            joblib.dump(self.scaler, os.path.join(self.models_dir, f'{name}_scaler.joblib'))
        elif self.model_type == 'arima':
            joblib.dump(self.model, os.path.join(self.models_dir, f'{name}_arima.joblib'))
    
    def load_model(self, name: str) -> None:
        """Load trained model from disk"""
        if self.model_type == 'lstm':
            self.model = load_model(os.path.join(self.models_dir, f'{name}_lstm.h5'))
            self.scaler = joblib.load(os.path.join(self.models_dir, f'{name}_scaler.joblib'))
        elif self.model_type == 'arima':
            self.model = joblib.load(os.path.join(self.models_dir, f'{name}_arima.joblib'))