import os
import joblib
from typing import List, Dict, Any
from transformers import pipeline
from sklearn.feature_extraction.text import TfidfVectorizer
from sklearn.svm import LinearSVC

class SentimentAnalyzer:
    def __init__(self, model_type: str = 'transformers'):
        """
        Initialize sentiment analyzer with either transformers (default) or traditional ML model
        
        Args:
            model_type: Either 'transformers' for HF pipeline or 'ml' for traditional model
        """
        self.model_type = model_type
        self.models_dir = os.path.join(os.path.dirname(__file__), '../../models')
        
        if model_type == 'transformers':
            self.model = pipeline(
                "sentiment-analysis",
                model="distilbert-base-uncased-finetuned-sst-2-english",
                tokenizer="distilbert-base-uncased-finetuned-sst-2-english"
            )
        else:
            # Load traditional ML model
            self.vectorizer = joblib.load(os.path.join(self.models_dir, 'tfidf_vectorizer.joblib'))
            self.classifier = joblib.load(os.path.join(self.models_dir, 'sentiment_classifier.joblib'))
    
    def analyze_text(self, text: str) -> Dict[str, Any]:
        """
        Analyze sentiment of a single text
        
        Args:
            text: Input text to analyze
            
        Returns:
            Dictionary with sentiment analysis results
        """
        if self.model_type == 'transformers':
            result = self.model(text)[0]
            return {
                'sentiment': result['label'],
                'confidence': result['score']
            }
        else:
            features = self.vectorizer.transform([text])
            prediction = self.classifier.predict(features)[0]
            probabilities = self.classifier.predict_proba(features)[0]
            
            return {
                'sentiment': 'POSITIVE' if prediction == 1 else 'NEGATIVE',
                'confidence': max(probabilities)
            }
    
    def analyze_batch(self, texts: List[str]) -> List[Dict[str, Any]]:
        """
        Analyze sentiment of multiple texts
        
        Args:
            texts: List of texts to analyze
            
        Returns:
            List of analysis results
        """
        return [self.analyze_text(text) for text in texts]
    
    def analyze_market_sentiment(self, news_items: List[Dict[str, Any]]) -> Dict[str, Any]:
        """
        Specialized method for analyzing market sentiment from news/social media items
        
        Args:
            news_items: List of news/social media items with 'text' and optionally 'source'
            
        Returns:
            Aggregated sentiment analysis
        """
        texts = [item['text'] for item in news_items]
        results = self.analyze_batch(texts)
        
        # Calculate aggregate sentiment
        positive_count = sum(1 for r in results if r['sentiment'] == 'POSITIVE')
        total = len(results)
        
        return {
            'positive': positive_count,
            'negative': total - positive_count,
            'ratio': positive_count / total if total > 0 else 0,
            'items': [
                {**item, 'sentiment': result}
                for item, result in zip(news_items, results)
            ]
        }