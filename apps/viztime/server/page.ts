import { readFile } from 'node:fs/promises';
import * as path from 'node:path';
import { appRoot } from './path';
import yaml from 'yaml';

export type Page = {
  name: string;
  begin: string;
  end: string;
  filters: {
    tags?: {
      include?: string[];
      exclude?: string[];
    };
  };
};

export async function readPages(): Promise<Page[]> {
  const configPath = path.resolve(appRoot, 'pages.yml');
  const fileContents = await readFile(configPath, 'utf-8');
  return yaml.parse(fileContents);
}
