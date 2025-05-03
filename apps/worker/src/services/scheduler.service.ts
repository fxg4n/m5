import cron from 'node-cron';
import logger from './logger.service';
import RabbitMQService from './rabbitmq.service';
import { ScraperTask } from '../interfaces/task.interface';

class SchedulerService {
  private scheduledJobs: cron.ScheduledTask[] = [];

  init() {
    // Schedule macro data scraping every day at 2 AM
    this.scheduleTask('0 2 * * *', {
      type: 'macro',
      source: 'imf',
      indicator: 'gdp'
    });

    // Schedule asset data scraping every hour
    this.scheduleTask('0 * * * *', {
      type: 'asset',
      source: 'yahoo',
      symbol: 'AAPL,MSFT,GOOGL'
    });

    // Schedule sentiment analysis every 30 minutes
    this.scheduleTask('*/30 * * * *', {
      type: 'sentiment',
      source: 'twitter',
      query: 'stock market'
    });
  }

  private scheduleTask(cronExpression: string, task: ScraperTask) {
    const job = cron.schedule(cronExpression, async () => {
      logger.info(`Running scheduled task: ${task.type} from ${task.source}`);
      await RabbitMQService.sendToQueue('scraper_tasks', task);
    });

    this.scheduledJobs.push(job);
    logger.info(`Scheduled task ${task.type} with expression ${cronExpression}`);
  }

  stopAll() {
    this.scheduledJobs.forEach(job => job.stop());
    logger.info('All scheduled jobs stopped');
  }
}

export default new SchedulerService();