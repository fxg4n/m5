import { exec } from 'child_process';
import { promisify } from 'util';

const execAsync = promisify(exec);

export async function runPythonScript(scriptPath: string, args: string[] = []): Promise<string> {
  try {
    const command = `python3 ${scriptPath} ${args.join(' ')}`;
    const { stdout, stderr } = await execAsync(command);
    
    if (stderr) {
      console.warn(`Python stderr: ${stderr}`);
    }
    
    return stdout;
  } catch (error) {
    console.error('Error executing Python script:', error);
    throw error;
  }
}