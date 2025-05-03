import dotenv from 'dotenv';

dotenv.config();

export default {
  rabbitMQ: {
    url: process.env.RABBITMQ_URL || 'amqp://localhost',
    queues: {
      scraper: process.env.RABBITMQ_SCRAPER_QUEUE || 'scraper_tasks',
      results: process.env.RABBITMQ_RESULT_QUEUE || 'scraper_results',
    },
  },
  scraper: {
    pythonPath: process.env.PYTHON_PATH || 'python3',
    scriptPath: process.env.SCRAPER_SCRIPT_PATH || './scripts/run-scraper.py',
  },
  worker: {
    concurrency: parseInt(process.env.WORKER_CONCURRENCY || '3', 10),
    maxRetries: parseInt(process.env.MAX_RETRIES || '3', 10),
  },
  logging: {
    level: process.env.LOG_LEVEL || 'info',
  },
};