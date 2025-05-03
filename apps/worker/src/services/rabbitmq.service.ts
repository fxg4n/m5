import amqp, { Channel, Connection } from 'amqplib';
import config from '../config';
import logger from './logger.service';

class RabbitMQService {
  private connection: Connection | null = null;
  private channel: Channel | null = null;

  async connect(): Promise<void> {
    try {
      this.connection = await amqp.connect(config.rabbitMQ.url);
      this.channel = await this.connection.createChannel();
      
      // Assert queues
      await this.channel.assertQueue(config.rabbitMQ.queues.scraper, { durable: true });
      await this.channel.assertQueue(config.rabbitMQ.queues.results, { durable: true });
      
      logger.info('Connected to RabbitMQ');
    } catch (error) {
      logger.error('Failed to connect to RabbitMQ:', error);
      throw error;
    }
  }

  async sendToQueue(queue: string, message: any): Promise<boolean> {
    if (!this.channel) {
      throw new Error('RabbitMQ channel not initialized');
    }
    
    try {
      const sent = this.channel.sendToQueue(
        queue,
        Buffer.from(JSON.stringify(message)),
        { persistent: true }
      );
      return sent;
    } catch (error) {
      logger.error(`Failed to send message to queue ${queue}:`, error);
      return false;
    }
  }

  async consume(queue: string, callback: (msg: any) => Promise<void>): Promise<void> {
    if (!this.channel) {
      throw new Error('RabbitMQ channel not initialized');
    }

    try {
      await this.channel.consume(queue, async (msg) => {
        if (msg) {
          try {
            const content = JSON.parse(msg.content.toString());
            await callback(content);
            this.channel?.ack(msg);
          } catch (error) {
            logger.error('Error processing message:', error);
            this.channel?.nack(msg, false, false);
          }
        }
      });
    } catch (error) {
      logger.error(`Failed to consume from queue ${queue}:`, error);
      throw error;
    }
  }

  async close(): Promise<void> {
    if (this.channel) await this.channel.close();
    if (this.connection) await this.connection.close();
    logger.info('RabbitMQ connection closed');
  }
}

export default new RabbitMQService();