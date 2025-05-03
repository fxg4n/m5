import { exec } from 'child_process';
import { promisify } from 'util';
import { ScraperTask } from '../interfaces/task.interface';
import { BaseWorker } from './base.worker';
import config from '../config';
import logger from '../services/logger.service';
import RabbitMQService from '../services/rabbitmq.service';

const execAsync = promisify(exec);

class ScraperWorker extends BaseWorker {
  constructor() {
    super(config.rabbitMQ.queues.scraper);
  }

  protected async processJob(job: { data: ScraperTask }): Promise<void> {
    const { data: task } = job;
    
    try {
      // Build Python command
      const command = this.buildPythonCommand(task);
      logger.info(`Executing command: ${command}`);
      
      // Execute Python scraper
      const { stdout, stderr } = await execAsync(command);
      
      if (stderr) {
        logger.warn(`Python scraper stderr: ${stderr}`);
      }
      
      // Process results
      const result = this.processOutput(stdout, task);
      
      // Send results to results queue
      await RabbitMQService.sendToQueue(config.rabbitMQ.queues.results, result);
      
    } catch (error) {
      logger.error(`Failed to execute scraper for task ${JSON.stringify(task)}:`, error);
      throw error;
    }
  }

  private buildPythonCommand(task: ScraperTask): string {
    const baseCommand = `${config.scraper.pythonPath} ${config.scraper.scriptPath}`;
    const args = [`--data-type ${task.type}`, `--source ${task.source}`];
    
    // Add task-specific arguments
    if (task.type === 'macro' && task.indicator) {
      args.push(`--indicator ${task.indicator}`);
      if (task.country) args.push(`--country ${task.country}`);
    } else if (task.type === 'asset' && task.symbol) {
      args.push(`--symbol ${task.symbol}`);
      if (task.days) args.push(`--days ${task.days}`);
    } else if (task.type === 'sentiment' && task.query) {
      args.push(`--query "${task.query}"`);
      if (task.limit) args.push(`--limit ${task.limit}`);
    }
    
    return `${baseCommand} ${args.join(' ')}`;
  }

  private processOutput(output: string, task: ScraperTask): any {
    try {
      const result = JSON.parse(output);
      return {
        ...task,
        status: 'completed',
        data: result,
        timestamp: new Date().toISOString()
      };
    } catch (error) {
      logger.error('Failed to parse scraper output:', error);
      return {
        ...task,
        status: 'failed',
        error: 'Failed to parse scraper output',
        timestamp: new Date().toISOString()
      };
    }
  }
}

export default ScraperWorker;