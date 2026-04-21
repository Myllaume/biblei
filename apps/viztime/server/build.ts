import { Liquid } from 'liquidjs';
import * as fs from 'fs';
import * as path from 'path';
import { appRoot, basePath } from './path';
import { getFichesWithDate, getFilterdFiches, type Fiche } from './records';
import { readPages } from './page';

// __dirname = apps/viztime/dist/ when running dist/build.cjs
const engine = new Liquid({
  root: path.resolve(appRoot, 'templates'),
  extname: '.liquid',
});

function slugify(name: string): string {
  return name
    .toLowerCase()
    .normalize('NFD')
    .replace(/[\u0300-\u036f]/g, '')
    .replace(/[^a-z0-9]+/g, '-')
    .replace(/^-|-$/g, '');
}

function formatDate([year, month, day]: number[]): string {
  return `${String(day).padStart(2, '0')}/${String(month).padStart(2, '0')}/${year}`;
}

function firstDate(fiche: Fiche): number {
  if (!fiche.dates || fiche.dates.length === 0) return 0;
  const [year, month, day] = fiche.dates[0];
  return year * 10000 + month * 100 + day;
}

async function build(): Promise<void> {
  const outDir = path.resolve(basePath, 'dist/viztime');
  fs.mkdirSync(outDir, { recursive: true });

  const fiches = await getFichesWithDate();
  process.stdout.write(`Fetched ${fiches.length} fiches\n`);

  const pagesYml = await readPages();
  process.stdout.write(`Fetched ${pagesYml.length} pages\n`);

  const indexHtml = await engine.renderFile('page', {
    title: 'Accueil',
    pages: pagesYml.map((page) => ({
      name: page.name,
      slug: slugify(page.name),
    })),
  });
  fs.writeFileSync(path.join(outDir, 'index.html'), indexHtml, 'utf-8');
  process.stdout.write(`Generated: index.html\n`);

  for (const page of pagesYml) {
    const slug = slugify(page.name);
    const pageFiches = getFilterdFiches(page, fiches);
    pageFiches.sort((a, b) => firstDate(a) - firstDate(b));

    const html = await engine.renderFile('timeline', {
      title: page.name,
      begin: page.begin,
      end: page.end,
      fiches: pageFiches.map((fiche) => ({
        title: fiche.title,
        dates: fiche.dates?.map(formatDate) ?? [],
      })),
    });
    fs.writeFileSync(path.join(outDir, `${slug}.html`), html, 'utf-8');
    process.stdout.write(`Generated: ${slug}.html\n`);
  }
}

void build().catch((err: unknown): void => {
  process.stderr.write(String(err) + '\n');
});
