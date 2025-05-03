export interface ScraperTask {
    type: 'macro' | 'asset' | 'sentiment';
    source: string;
    
    // Macro-specific
    indicator?: string;
    country?: string;
    
    // Asset-specific
    symbol?: string;
    days?: number;
    
    // Sentiment-specific
    query?: string;
    limit?: number;
    
    // Metadata
    retryCount?: number;
    priority?: number;
  }