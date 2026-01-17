import { MeiliSearch } from 'meilisearch';

const host =
  import.meta.env?.VITE_MEILI_HOST || process.env.VITE_MEILI_HOST || 'http://localhost:7700';
const apiKey =
  import.meta.env?.VITE_MEILI_API_KEY || process.env.VITE_MEILI_API_KEY || 'aSampleMasterKey';

export const meiliClient = new MeiliSearch({
  host,
  apiKey,
});

export const DOCS_INDEX = 'docs';

export async function searchDocs(query: string, limit = 10) {
  try {
    const index = meiliClient.index(DOCS_INDEX);
    const response = await index.search(query, {
      limit,
      attributesToHighlight: ['title', 'description'],
      highlightPreTag: '<mark>',
      highlightPostTag: '</mark>',
    });
    return response.hits;
  } catch (error) {
    console.error('Meilisearch error:', error);
    return [];
  }
}

export async function addDocumentsToIndex(documents: Record<string, unknown>[]) {
  try {
    const index = meiliClient.index(DOCS_INDEX);
    const response = await index.addDocuments(documents);
    console.log('Documents added:', response);
    return response;
  } catch (error) {
    console.error('Error adding documents:', error);
    throw error;
  }
}

export async function deleteIndex() {
  try {
    await meiliClient.deleteIndex(DOCS_INDEX);
    console.log('Index deleted');
  } catch (error) {
    console.error('Error deleting index:', error);
  }
}
