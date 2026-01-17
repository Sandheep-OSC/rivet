import fs from 'node:fs';
import path from 'node:path';
import matter from 'gray-matter';
import { addDocumentsToIndex, deleteIndex } from '../src/lib/utils/meilisearch';

interface Document {
  id: string;
  slug: string;
  title: string;
  description: string;
  section: string;
  content: string;
  order: number;
}

const docsDir = path.join(process.cwd(), 'src/lib/docs');
const shouldDeleteIndex = process.argv.includes('--delete');

async function buildSearchIndex() {
  if (shouldDeleteIndex) {
    console.log('Deleting existing index...');
    await deleteIndex();
  }

  const files = fs.readdirSync(docsDir);
  const documents: Document[] = [];

  for (const file of files) {
    if (!file.endsWith('.md')) continue;

    const filePath = path.join(docsDir, file);
    const content = fs.readFileSync(filePath, 'utf-8');
    const { data, content: body } = matter(content);

    documents.push({
      id: file.replace('.md', ''),
      slug: file.replace('.md', ''),
      title: data.title,
      description: data.description,
      section: data.section || 'General',
      content: body,
      order: data.order || 99,
    });
  }

  documents.sort((a, b) => a.order - b.order);

  if (documents.length > 0) {
    console.log(`Indexing ${documents.length} documents...`);
    await addDocumentsToIndex(documents);
    console.log('Search index built successfully!');
  } else {
    console.log('No documents found to index.');
  }
}

buildSearchIndex().catch(console.error);
