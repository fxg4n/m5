import RabbitMQService from './services/rabbitmq.service';
import SchedulerService from './services/scheduler.service';
import ScraperWorker from './workers/scraper.worker';
import logger from './services/logger.service';

class App {
  private scraperWorker: ScraperWorker;

  async start() {
    try {
      // Initialize services
      await RabbitMQService.connect();
      
      // Start scheduler
      SchedulerService.init();
      
      // Start workers
      this.scraperWorker = new ScraperWorker();
      
      logger.info('Application started successfully');
    } catch (error) {
      logger.error('Failed to start application:', error);
      process.exit(1);
    }
  }

  async gracefulShutdown() {
    logger.info('Shutting down gracefully...');
    
    // Stop scheduler
    SchedulerService.stopAll();
    
    // Close RabbitMQ connection
    await RabbitMQService.close();
    
    logger.info('Shutdown complete');
    process.exit(0);
  }
}

const app = new App();

// Start the application
app.start();

// Handle shutdown signals
process.on('SIGINT', () => app.gracefulShutdown());
process.on('SIGTERM', () => app.gracefulShutdown());