import { readFile } from 'node:fs/promises';
import * as path from 'node:path';
import dayjs from 'dayjs';
import { basePath } from './path';
import yaml from 'yaml';
import { Page } from './page';

async function readConfig(): Promise<Record<string, unknown>> {
  const configPath = path.resolve(basePath, 'config.yml');
  const fileContents = await readFile(configPath, 'utf-8');
  return yaml.parse(fileContents);
}

export type Fiche = {
  title: string;
  dates?: number[][];
  tags?: string[];
};

function ficheHasDate(fiche: Fiche): boolean {
  return (
    Array.isArray(fiche.dates) &&
    fiche.dates.length > 0 &&
    fiche.dates.some(
      (date) =>
        Array.isArray(date) &&
        date.length === 3 &&
        date.every((num) => typeof num === 'number')
    )
  );
}

export async function getFichesWithDate(): Promise<Fiche[]> {
  const config = await readConfig();

  if (
    config === null ||
    typeof config !== 'object' ||
    'record_output' in config === false ||
    typeof config.record_output !== 'string'
  ) {
    throw new Error('Invalid config: missing record_output');
  }

  const recordPath = path.resolve(basePath, config.record_output);
  const recordContent = await readFile(recordPath, 'utf-8');
  const fiches = JSON.parse(recordContent) as Fiche[];
  return fiches.filter(ficheHasDate);
}

export function getFilterdFiches(page: Page, fiches: Fiche[]): Fiche[] {
  let filteredFiches = [...fiches];

  const begin = dayjs(page.begin);
  const end = dayjs(page.end);

  function isBetween(date: dayjs.Dayjs): boolean {
    return (
      (date.isSame(begin, 'day') || date.isAfter(begin, 'day')) &&
      (date.isSame(end, 'day') || date.isBefore(end, 'day'))
    );
  }

  filteredFiches = filteredFiches.filter((fiche) => {
    if (!Array.isArray(fiche.dates) || fiche.dates.length === 0) {
      return false;
    }

    return fiche.dates.some((date) => {
      if (!Array.isArray(date) || date.length !== 3) {
        return false;
      }

      const [year, month, day] = date;
      return isBetween(
        dayjs(
          `${year}-${String(month).padStart(2, '0')}-${String(day).padStart(2, '0')}`
        )
      );
    });
  });

  if (page.filters.tags) {
    filteredFiches = filteredFiches.filter((fiche) => {
      if (!Array.isArray(fiche.tags) || fiche.tags.length === 0) {
        return false;
      }

      const tagsLower = fiche.tags.map((tag) => tag.toLocaleLowerCase());

      if (page.filters.tags!.include) {
        const hasIncludedTag = tagsLower.some((tag) =>
          page.filters.tags!.include!.includes(tag)
        );
        if (!hasIncludedTag) {
          return false;
        }
      }

      if (page.filters.tags!.exclude) {
        const hasExcludedTag = tagsLower.some((tag) =>
          page.filters.tags!.exclude!.includes(tag)
        );
        if (hasExcludedTag) {
          return false;
        }
      }

      return true;
    });
  }

  return filteredFiches;
}
