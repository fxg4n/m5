import { Worker } from 'bullmq';
import logger from '../services/logger.service';
import config from '../config';

export abstract class BaseWorker {
  protected worker: Worker;
  protected maxRetries: number;

  constructor(queueName: string) {
    this.maxRetries = config.worker.maxRetries;
    this.initializeWorker(queueName);
  }

  private initializeWorker(queueName: string) {
    this.worker = new Worker(queueName, async (job) => {
      try {
        logger.info(`Processing job ${job.id} - Attempt ${job.attemptsMade + 1}`);
        return await this.processJob(job);
      } catch (error) {
        logger.error(`Job ${job.id} failed:`, error);
        throw error;
      }
    }, {
      concurrency: config.worker.concurrency,
      connection: {
        host: config.rabbitMQ.url
      }
    });

    this.worker.on('completed', (job) => {
      logger.info(`Job ${job.id} completed successfully`);
    });

    this.worker.on('failed', (job, err) => {
      logger.error(`Job ${job?.id} failed after ${job?.attemptsMade} attempts:`, err);
    });
  }

  protected abstract processJob(job: any): Promise<any>;
}